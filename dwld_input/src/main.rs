use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// get input for `year` and `day` from the web
fn request_input(session_id: &str, year: u32, day: u32) -> Result<String, String> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let session_cookie = format!("session={session_id}");

    let response = ureq::get(&url)
        .set("Cookie", &session_cookie)
        .call()
        .map_err(|e| e.to_string())?;

    if response.status() != 200 {
        return Err(format!(
            "Request failed with status: {:?}",
            response.status()
        ));
    }

    response.into_string().map_err(|e| e.to_string())
}

/// get input and write to disk
fn download_input(dir: &Path, session_id: &str, year: u32, day: u32) -> Option<()> {
    let input = request_input(session_id, year, day).unwrap();

    let out_file = dir.join("src").join("input1.txt");
    fs::write(out_file, input.strip_suffix('\n')?).ok()?;

    Some(())
}

/// get folders in `path`
fn get_folders(path: &PathBuf) -> Option<impl Iterator<Item = PathBuf>> {
    let folders = fs::read_dir(path)
        .ok()?
        .flatten()
        .filter_map(|entry| entry.path().is_dir().then_some(entry.path()));

    Some(folders)
}

/// get all days that have folders created
fn get_days(year_root: &PathBuf) -> Option<Vec<(PathBuf, u32)>> {
    let days = get_folders(year_root)?
        .filter_map(|dir| {
            dir.file_name()
                .and_then(|dirname| dirname.to_str())
                .and_then(|dirname| dirname.starts_with("day").then_some(&dirname[3..]))
                .and_then(|day| day.parse::<u32>().ok())
                .map(|day| (dir, day))
        })
        .collect::<Vec<_>>();

    Some(days)
}

/// get all the challenges that have folders created
fn get_challenges() -> Option<HashMap<u32, Vec<(PathBuf, u32)>>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");

    let challenges = get_folders(&root)?
        .filter_map(|dir| {
            dir.file_name()
                .and_then(|dirname| dirname.to_str())
                .and_then(|dirname| dirname.parse().ok())
                .and_then(|year| get_days(&dir).map(|days| (year, days)))
        })
        .collect::<_>();

    Some(challenges)
}

/// pop first element if present, and `map` it
fn pop_arg<T>(args: &mut Vec<String>, map: fn(&str) -> T) -> Option<T> {
    match args.first() {
        Some(value) => {
            let extracted = map(value);
            args.remove(0);
            Some(extracted)
        }
        _ => None,
    }
}

/// download input files.
/// requires a `session_id`, and optionally `year` and `day`
/// puts them in `year`/day`day`/src/input1.txt
fn main() {
    let mut args = env::args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        panic!("cargo run --package=dwnl_input -- session_id [year] [day]");
    }

    let session_id = pop_arg(&mut args, |session_id| session_id.to_owned()).unwrap();
    let year = pop_arg(&mut args, |year| year.parse::<u32>().unwrap());
    let day = pop_arg(&mut args, |day| day.parse::<u32>().unwrap());

    let mut challenges = get_challenges().unwrap();

    // if i have a specific year, discard all others
    if let Some(year) = year {
        if let Some(days) = challenges.remove(&year) {
            challenges = HashMap::from([(year, days)]);

            // if i have a specific day, discard all others
            if let Some(day) = day {
                challenges.entry(year).and_modify(|days| {
                    if let Some((dir, day)) = days.iter().find(|(_, d)| *d == day) {
                        *days = vec![(dir.to_path_buf(), *day)];
                    }
                });
            }
        } else {
            challenges = HashMap::new();
        }
    }

    for (year, days) in challenges {
        for (dir, day) in days {
            download_input(&dir, &session_id, year, day).unwrap();
        }
    }
}
