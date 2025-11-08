use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

fn wget(rootdir: &Path, year: u32, day: u32, session_id: &str) {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let header = format!("--header=Cookie: session={session_id}");
    let outpath = rootdir
        .join("src")
        .join("input1.txt")
        .to_str()
        .unwrap()
        .to_string();
    let output = format!("--output-document={outpath}");

    let _proc = process::Command::new("wget")
        .arg(&header)
        .arg(&output)
        .arg(&url)
        .output()
        .unwrap();

    // println!("wget {header} {output} {url}");
    // let stdout = String::from_utf8(proc.stdout).unwrap();
    // let stderr = String::from_utf8(proc.stderr).unwrap();
    // println!("stdout: \n{stdout}\n");
    // println!("stderr: \n{stderr}\n");

    println!("Downloaded year {year} day {day} in {outpath}");
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
            wget(&dir, year, day, &session_id);
        }
    }
}
