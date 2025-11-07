use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// get input for `year` and `day` from the web
fn wgets(session_id: &str, year: u32, day: u32) -> String {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let session_cookie = format!("session={session_id}");

    let response = ureq::get(&url)
        .set("Cookie", &session_cookie)
        .call()
        .map_err(|e| e.to_string())
        .unwrap();

    if response.status() != 200 {
        panic!(
            "Request failed with status {}: {:?}",
            response.status(),
            response.into_string()
        );
    }

    response.into_string().unwrap()
}

/// get input and write to disk
fn get_input(dir: &Path, session_id: &str, year: u32, day: u32) {
    let contents = wgets(session_id, year, day);
    let outpath = dir.join("src").join("input1.txt");
    fs::write(outpath, contents).unwrap();

    println!("Yoinked year {year} day {day}");
}

/// get folders in `path`
fn list_folders(path: &PathBuf) -> impl Iterator<Item = PathBuf> {
    fs::read_dir(path)
        .unwrap()
        .flatten()
        .filter_map(|entry| entry.path().is_dir().then_some(entry.path()))
}

/// get all days that have folders created
fn list_days(year_root: &PathBuf) -> impl Iterator<Item = (PathBuf, u32)> {
    list_folders(year_root).filter_map(|dir| {
        dir.file_name()
            .and_then(|dirname| dirname.to_str())
            .and_then(|dirname| dirname.starts_with("day").then_some(&dirname[3..]))
            .and_then(|day| day.parse::<u32>().ok())
            .map(|day| (dir, day))
    })
}

/// get all the challenges that have folders created
fn get_challenges() -> HashMap<u32, Vec<(PathBuf, u32)>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");

    list_folders(&root)
        .filter_map(|dir| {
            let year = dir
                .file_name()
                .and_then(|dirname| dirname.to_str())
                .and_then(|dirname| dirname.parse::<u32>().ok())?;
            let days = list_days(&dir).collect::<Vec<_>>();

            Some((year, days))
        })
        .collect()
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
        println!(
            "Usage: cargo run --package={:?} -- session_id [year] [day]",
            env::current_exe().unwrap().file_stem().unwrap()
        );
        return;
    }

    let session_id = pop_arg(&mut args, |session_id| session_id.to_owned()).unwrap();
    let year = pop_arg(&mut args, |year| year.parse::<u32>().unwrap());
    let day = pop_arg(&mut args, |day| day.parse::<u32>().unwrap());

    let mut challenges = get_challenges();

    // if i have a specific year, discard all others
    if let Some(year) = year {
        let days = challenges.remove(&year).unwrap();

        // if i have a specific day, discard all others
        if let Some(day) = day {
            let day_entry = days.into_iter().find(|(_, d)| *d == day).unwrap();
            challenges = HashMap::from([(year, vec![day_entry])]);
        } else {
            challenges = HashMap::from([(year, days)]);
        }
    }

    for (year, days) in challenges {
        for (dir, day) in days {
            get_input(&dir, &session_id, year, day);
        }
    }
}
