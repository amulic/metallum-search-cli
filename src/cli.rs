use clap::{Arg, Command};

pub fn build_cli() -> Command {
    Command::new("metal_search_cli")
        .version("1.0")
        .about("A command-line tool to search for bands on metal-archives.com")
        .arg(
            Arg::new("band")
            .help("Search query for the band")
            .required(false)
            .index(1),
        )
        .arg (
            Arg::new("genre")
            .help("Search query for the genre")
            .short('g')
            .long("genre")
        )
        .arg (
            Arg::new("title")
            .help("Search query for the album title")
            .short('t')
            .long("title")
        )
 }