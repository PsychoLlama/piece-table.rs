use std::collections::BTreeSet;
use std::fmt;

pub struct IndexedString {
    linebreaks: BTreeSet<usize>,
    source: String,
}

#[allow(dead_code)]
static CARRIAGE_RETURN: u8 = 13; // '\r'
static LINE_FEED: u8 = 10; // '\n'

impl IndexedString {
    fn find_linebreaks(source: &str, byte_offset: usize) -> Vec<usize> {
        let mut linebreaks = vec![];

        for (index, character) in source.bytes().enumerate() {
            if character != LINE_FEED {
                continue;
            }

            linebreaks.push(index + byte_offset);
        }

        linebreaks
    }

    fn index_linebreaks(&mut self) {
        for linebreak in IndexedString::find_linebreaks(&self.source[..], 0) {
            self.linebreaks.insert(linebreak);
        }
    }

    #[allow(dead_code)]
    pub fn new() -> Self {
        IndexedString {
            linebreaks: BTreeSet::new(),
            source: String::new(),
        }
    }

    #[allow(dead_code)]
    pub fn from(source: &str) -> Self {
        let mut text = IndexedString {
            linebreaks: BTreeSet::new(),
            source: source.to_owned(),
        };

        text.index_linebreaks();

        return text;
    }

    #[allow(dead_code)]
    pub fn append(&mut self, text: &str) {
        let bytes = IndexedString::find_linebreaks(text, self.source.len());

        for byte_index in bytes {
            self.linebreaks.insert(byte_index);
        }

        self.source += text;
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.source.len()
    }
}

impl fmt::Display for IndexedString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_linebreaks(text: IndexedString) -> Vec<usize> {
        text.linebreaks.into_iter().collect()
    }

    fn get_first_linebreak<'a>(text: IndexedString) -> usize {
        let linebreaks = get_linebreaks(text);

        return linebreaks
            .first()
            .expect("Linebreaks set was empty.")
            .clone();
    }

    #[test]
    fn test_construction() {
        let text = IndexedString::new();

        assert_eq!(text.source.len(), 0);
    }

    #[test]
    fn test_populated_construction() {
        let text = IndexedString::from("slice");

        assert_eq!(text.source, "slice".to_owned());
    }

    #[test]
    fn test_no_newlines() {
        let text = IndexedString::from("single line");

        assert_eq!(text.linebreaks, BTreeSet::new());
    }

    #[test]
    fn test_single_newline() {
        let text = IndexedString::from("first\nsecond");

        assert_eq!(text.linebreaks.len(), 1);
    }

    #[test]
    fn test_multiple_newlines() {
        let text = IndexedString::from("first\nsecond\n\nfourth");

        assert_eq!(text.linebreaks.len(), 3);
    }

    #[test]
    fn test_dangling_newline() {
        let text = IndexedString::from("line\n");

        assert_eq!(text.linebreaks.len(), 1);
    }

    #[test]
    fn test_byte_indexing() {
        let text = IndexedString::from("line\n");

        assert_eq!(get_first_linebreak(text), 4);
    }

    #[test]
    fn test_emoji_ignorance() {
        let elf_emoji = String::from_utf8(vec![
            240, 159, 167, 157, 226, 128, 141, 226, 153, 130, 239, 184, 143,
        ])
        .unwrap();

        let mut source = elf_emoji.clone();
        source.push('\n');

        let text = IndexedString::from(&source);

        let expected_offset = elf_emoji.bytes().len();
        assert_eq!(get_first_linebreak(text), expected_offset);
    }

    #[test]
    fn test_string_append() {
        let mut text = IndexedString::from("hello");
        text.append(" world");

        assert_eq!(text.source, "hello world".to_owned());
    }

    #[test]
    fn test_line_append() {
        let mut text = IndexedString::from("first line");
        text.append("\nsecond line");

        assert_eq!(text.linebreaks.len(), 1);
        assert_eq!(get_first_linebreak(text), 10);
    }

    #[test]
    fn test_line_length() {
        let text = IndexedString::from("value");

        assert_eq!(text.len(), text.source.len());
    }

    #[test]
    fn test_to_string() {
        let text = IndexedString::from("value");

        assert_eq!(text.to_string(), "value".to_owned());
    }
}
