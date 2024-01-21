use reqwest::blocking::Client;
use reqwest::header::COOKIE;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

fn download_input(session_id: &str, year: u32, day: u32) -> Result<String, String> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let session_cookie = format!("session={session_id}");

    let client = Client::new();
    let response = client
        .get(url)
        .header(COOKIE, session_cookie)
        .send()
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!(
            "Request failed with status: {:?}",
            response.status()
        ));
    }

    let body = response.text().map_err(|e| e.to_string())?;
    Ok(body)
}

fn get_input(session_id: &str, year: u32, day: u32) -> Result<(), String> {
    let input = download_input(session_id, year, day)?;

    fs::write(
        format!("./{year}/day{day}/src/input1.txt"),
        input.strip_suffix('\n').unwrap(),
    ).map_err(|e| e.to_string())?;

    Ok(())
}

fn get_inputs(session_id: &str, workspace: HashMap<u32, Vec<u32>>) -> Option<()> {
    for (year, days) in workspace {
        for day in days {
            let _ = get_input(session_id, year, day);
        }
    }

    Some(())
}

fn get_directories_in_path(path: &PathBuf) -> Option<impl Iterator<Item = PathBuf>> {
    let entries = fs::read_dir(path).ok()?;

    let folders = entries.flatten().filter_map(|entry| {
        if entry.path().is_dir() {
            Some(entry.path())
        } else {
            None
        }
    });

    Some(folders)
}

fn get_days_directories(year_root: &PathBuf) -> Option<Vec<u32>> {
    let mut days = Vec::new();

    for directory in get_directories_in_path(year_root)? {
        if let Some(dirname) = directory.file_name() {
            if dirname.to_string_lossy().starts_with("day") {
                if let Ok(day) = dirname.to_str()?[3..].parse::<u32>() {
                    days.push(day);
                }
            }
        }
    }

    Some(days)
}

fn get_aoc_workspaces(workspace_root: &PathBuf) -> Option<HashMap<u32, Vec<u32>>> {
    let mut years = HashMap::new();

    for directory in get_directories_in_path(workspace_root)? {
        if let Some(dirname) = directory.file_name() {
            if let Ok(year) = dirname.to_string_lossy().parse::<u32>() {
                if let Some(days) = get_days_directories(&directory) {
                    years.insert(year, days);
                }
            }
        }
    }

    Some(years)
}

// downloads input files and puts them in {year}/day{day}/src/input1.txt
// don't abuse it
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let path = PathBuf::from(".");
        let session_id = &args[1];

        if let Some(workspace) = get_aoc_workspaces(&path) {
            get_inputs(session_id, workspace);
        }
    } else if args.len() == 4  {
        let session_id = &args[1];
        let year = args[2].parse().unwrap();
        let day = args[3].parse().unwrap();

        let _ = get_input(session_id, year, day);
    }
    else {
        println!("example 1 (dl everything for current workspace):");
        println!("\tcargo run --package=input_dl -- session_id");
        println!("example 2 (dl only a specific one): ");
        println!("\tcargo run --package=input_dl -- session_id year day");
    }
}
