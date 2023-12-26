use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

fn search_dir(path: &Path, found_dirs: &mut Vec<String>) {
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let element = entry.unwrap();
            if element.path().is_dir() {
                search_dir(&element.path(), found_dirs)
            } else if element.path().is_file()
                && element
                    .path()
                    .file_name()
                    .map_or(false, |name| name == "qmldir")
            {
                found_dirs.push(format!(
                    "-b {}",
                    element.path().parent().unwrap().display().to_string()
                ));
            }
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No build directory passed.");
        return ExitCode::from(1);
    }

    let build_dir = &args[1];
    if !Path::new(&build_dir).exists() {
        println!("Build directory {} doesn't exists.", build_dir);
    }

    let mut found_dirs: Vec<String> = Vec::new();
    search_dir(Path::new(build_dir), &mut found_dirs);

    for dir in found_dirs {
        print!(" {}", dir);
    }
    return ExitCode::from(0);
}
