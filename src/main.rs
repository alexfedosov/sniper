use clap::{App, Arg};
use std::{env, fmt, fs::File, io, io::BufRead, path};

struct Snippet {
    command: String,
    description: String,
}

impl fmt::Display for Snippet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = writeln!(f, "{}", self.command);
        writeln!(f, "{}", self.description)
    }
}

fn main() {
    let matches = App::new("Yet another command-line snippet manager")
        .arg(
            Arg::new("debug")
                .about("Show debugging information")
                .short('d')
                .long("debug"),
        )
        .subcommand(App::new("list").about("Show all available snippets"))
        .get_matches();

    if matches.is_present("debug") {
        println!("Debugging is turned on");
    }

    match matches.subcommand_name() {
        Some("list") => list().unwrap(),
        None => println!("Try 'sniper help'"),
        _ => println!("Unknown command"),
    }
}

fn list() -> Result<(), std::io::Error> {
    let current_dir = env::current_dir()?;
    let ancestors = current_dir.ancestors();
    for dir in ancestors {
        let sniper_file_path = dir.join(".sniper");
        if sniper_file_path.exists() {
            let snippets = parse_sniper_file(sniper_file_path.as_path());
            for snippet in snippets {
                println!("{:}", snippet)
            }
            break;
        }
    }

    Ok(())
}

#[must_use]
fn parse_sniper_file(path: &path::Path) -> Vec<Snippet> {
    let mut snippets = Vec::default();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(line) = line {
                snippets.push(Snippet {
                    command: line,
                    description: String::from("no description provided"),
                });
            }
        }
    }
    snippets
}

fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<path::Path>,
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
