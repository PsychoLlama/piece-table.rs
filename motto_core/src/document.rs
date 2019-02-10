use super::fragment::{Fragment, Source};
use super::indexed_string::IndexedString;
use std::collections::BTreeMap;
use std::fmt;

#[allow(dead_code)]
pub struct Document {
    fragments: BTreeMap<usize, Fragment>,
    linebreaks: BTreeMap<usize, usize>,
    insertions: IndexedString,
    original: IndexedString,
}

impl Document {
    fn create_fragment_map(source: &IndexedString) -> BTreeMap<usize, Fragment> {
        let mut fragments = BTreeMap::new();
        let initial_fragment = Fragment::from_string(&source);

        fragments.insert(0, initial_fragment);

        return fragments;
    }

    #[allow(dead_code)]
    pub fn from(text: &str) -> Self {
        let original = IndexedString::from(text);

        Document {
            fragments: Document::create_fragment_map(&original),
            insertions: IndexedString::new(),
            linebreaks: BTreeMap::new(),
            original,
        }
    }

    #[allow(dead_code)]
    pub fn new() -> Self {
        return Document::from("");
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        return self.original.len();
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, byte_offset: usize, text: &str) {
        let insertion_offset = self.insertions.len();
        self.insertions.append(text);

        let frag = Fragment::of_insertion(insertion_offset, text.len());
        self.fragments.insert(byte_offset, frag);
    }

    fn get_source_for_fragment(&self, fragment: &Fragment) -> &IndexedString {
        return match fragment.source {
            Source::Insertion => &self.insertions,
            Source::Original => &self.original,
        };
    }
}

impl fmt::Display for Document {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let result: String = self
            .fragments
            .iter()
            .map(|(_, frag)| (frag, self.get_source_for_fragment(&frag)))
            .map(|(frag, source)| frag.get_slice(&source))
            .collect();

        return write!(fmt, "{}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fragment_tuple(text: &Document, index: usize) -> (&usize, &Fragment) {
        return text
            .fragments
            .iter()
            .take(index + 1)
            .last()
            .expect(format!("No fragment at index {}", index).as_ref());
    }

    fn get_fragment(text: &Document, index: usize) -> &Fragment {
        return get_fragment_tuple(&text, index).1;
    }

    #[test]
    fn test_empty_text_construction() {
        let text = Document::new();

        assert_eq!(text.original.len(), 0);
        assert_eq!(text.insertions.len(), 0);
    }

    #[test]
    fn test_filled_construction() {
        let text = "Initial value";
        let st = Document::from(text);

        assert_eq!(st.original.len(), text.len());
        assert_eq!(st.insertions.len(), 0);
    }

    #[test]
    fn test_simple_initial_fragment_list() {
        let text = Document::from("value");

        assert_eq!(text.fragments.len(), 1);

        let fragment = get_fragment(&text, 0);
        assert_eq!(fragment.byte_offset, 0);
        assert_eq!(fragment.byte_length, text.original.len());
        assert_eq!(fragment.source, Source::Original);
    }

    #[test]
    fn test_simple_empty_fragment_list() {
        let text = Document::new();

        assert_eq!(text.fragments.len(), 1);

        let fragment = get_fragment(&text, 0);
        assert_eq!(fragment.byte_offset, 0);
        assert_eq!(fragment.byte_length, 0);
        assert_eq!(fragment.source, Source::Original);
    }

    #[test]
    fn test_length() {
        let source = "hello world";
        let text = Document::from(&source);

        assert_eq!(text.len(), source.len());
    }

    #[test]
    fn test_display() {
        let source = "source text";
        let text = Document::from(&source);

        assert_eq!(text.to_string(), source);
    }

    #[test]
    fn test_insert_adds_insertion_string() {
        let mut text = Document::from("hello");
        text.insert(0, "first ");
        text.insert(0, "second ");
        text.insert(0, "third");

        assert_eq!(text.insertions.to_string(), "first second third");
    }

    #[test]
    fn test_appending_insert_fragment_construction() {
        let mut text = Document::from("hello");
        text.insert(6, " world!");

        let expected = (&6, &Fragment::of_insertion(0, 7));
        assert_eq!(text.fragments.len(), 2);
        assert_eq!(get_fragment_tuple(&text, 1), expected);
    }

    #[test]
    fn test_display_with_fragments() {
        let mut text = Document::from("original");
        text.insert(8, " with");
        text.insert(13, " insertions");

        assert_eq!(text.to_string(), "original with insertions");
    }
}
