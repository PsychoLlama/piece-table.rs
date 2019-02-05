use super::indexed_string::IndexedString;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum Source {
    Insertion,
    Original,
}

#[derive(Debug)]
struct Fragment {
    byte_offset: usize,
    byte_length: usize,
    source: Source,
}

#[allow(dead_code)]
pub struct Document {
    fragments: BTreeMap<usize, Fragment>,
    linebreaks: BTreeMap<usize, usize>,
    insertions: IndexedString,
    original: IndexedString,
}

impl Fragment {
    fn new(source: Source, byte_offset: usize, byte_length: usize) -> Self {
        Fragment {
            byte_length,
            byte_offset,
            source,
        }
    }

    pub fn from_original(text: &IndexedString) -> Self {
        let size = text.len();

        return Fragment::new(Source::Original, 0, size);
    }
}

impl Document {
    fn create_fragment_map(source: &IndexedString) -> BTreeMap<usize, Fragment> {
        let mut fragments = BTreeMap::new();
        let initial_fragment = Fragment::from_original(&source);

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
}
