use std::collections::BTreeSet;

struct IndexedString {
    linebreaks: BTreeSet<usize>,
    source: String,
}

static carriage_return: u8 = 13; // '\r'
static line_feed: u8 = 10; // '\n'

impl IndexedString {
    fn index_linebreaks(&mut self) {
        for (index, character) in self.source.bytes().enumerate() {
            if character != line_feed {
                continue;
            }

            self.linebreaks.insert(index);
        }
    }

    pub fn from(source: &str) -> Self {
        let mut text = IndexedString {
            linebreaks: BTreeSet::new(),
            source: source.to_owned(),
        };

        text.index_linebreaks();

        return text;
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
}
