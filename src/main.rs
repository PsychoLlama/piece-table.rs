#[macro_use]
extern crate clap;
extern crate motto_core;
extern crate termion;

use clap::{App, Arg, ArgMatches};

// Generate a clap CLI config.
fn get_app_config<'a, 'b>() -> App<'a, 'b> {
    return app_from_crate!().arg(Arg::with_name("files").multiple(true));
}

// Figure out which files we need to edit.
fn get_file_list<'a>(matches: &'a ArgMatches) -> Vec<&'a str> {
    return match matches.values_of("files") {
        Some(values) => values.into_iter().collect(),
        None => vec![],
    };
}

fn main() {
    let app = get_app_config();
    let matches = app.get_matches();
    let _files_to_edit = get_file_list(&matches);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_file_list() {
        let app = get_app_config();
        let matches = app.get_matches_from(vec!["prg", "first", "second"]);
        let files = get_file_list(&matches);

        assert_eq!(files, vec!["first", "second"]);
    }

    #[test]
    fn test_empty_files_default() {
        let app = get_app_config();
        let matches = app.get_matches_from(vec!["prg"]);
        let files = get_file_list(&matches);
        let expected: Vec<&str> = vec![];

        assert_eq!(files, expected);
    }
}
