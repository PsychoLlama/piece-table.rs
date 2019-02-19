#[macro_use]
extern crate clap;
extern crate motto_core;
extern crate termion;

use clap::{App, Arg, ArgMatches};
use std::io::{self, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::{clear, cursor};

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
    let stdin = io::stdin();
    let stdout = io::stdout().into_raw_mode().unwrap();

    let app = get_app_config();
    let matches = app.get_matches();
    let files_to_edit = get_file_list(&matches);

    let mut screen = AlternateScreen::from(stdout);

    write!(
        screen,
        "{}{}{:?}",
        clear::All,
        cursor::Goto(1, 1),
        files_to_edit
    )
    .unwrap();

    screen.flush().unwrap();

    for character in stdin.keys() {
        match character.unwrap() {
            Key::Ctrl('c') => break,
            _ => {}
        }
    }
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
