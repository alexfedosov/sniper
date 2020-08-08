use clap::{App, Arg};

fn main() {
    let matches = App::new("Yet another command-line snippet manager")
        .arg(Arg::new("debug")
            .about("Show debugging information")
            .short('d')
            .long("debug"))
        .subcommand(App::new("list").about("Show all available snippets"))
        .get_matches();

    if matches.is_present("debug") {
        println!("Debugging is turned on");
    }

    match matches.subcommand_name() {
        Some("list") => println!("No snippets found"),
        None => println!("Try 'sniper help'"),
        _ => println!("Unknown command"),
    }
}
