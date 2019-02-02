use super::indexed_string::IndexedString;

#[derive(Debug)]
struct Fragment {
    byte_offset: usize,
    byte_length: usize,
    is_new: bool,
}

#[allow(dead_code)]
pub struct SourceText {
    fragments: Vec<Fragment>,
    insertions: IndexedString,
    source: IndexedString,
}

impl SourceText {
    // Create the initial source fragment. Spans the whole string.
    fn create_source_fragment(source: &IndexedString) -> Fragment {
        Fragment {
            byte_length: source.len(),
            byte_offset: 0,
            is_new: false,
        }
    }

    #[allow(dead_code)]
    pub fn new() -> SourceText {
        let source = IndexedString::new();
        let fragment = SourceText::create_source_fragment(&source);

        SourceText {
            insertions: IndexedString::new(),
            fragments: vec![fragment],
            source,
        }
    }

    #[allow(dead_code)]
    pub fn from(text: &str) -> SourceText {
        let source = IndexedString::from(text);
        let fragment = SourceText::create_source_fragment(&source);

        SourceText {
            insertions: IndexedString::new(),
            fragments: vec![fragment],
            source,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fragment(text: &SourceText, index: usize) -> &Fragment {
        &text.fragments[index]
    }

    #[test]
    fn test_empty_text_construction() {
        let text = SourceText::new();

        assert_eq!(text.source.len(), 0);
        assert_eq!(text.insertions.len(), 0);
    }

    #[test]
    fn test_filled_construction() {
        let text = "Initial value";
        let st = SourceText::from(text);

        assert_eq!(st.source.len(), text.len());
        assert_eq!(st.insertions.len(), 0);
    }

    #[test]
    fn test_simple_initial_fragment_list() {
        let text = SourceText::from("value");

        assert_eq!(text.fragments.len(), 1);

        let fragment = get_fragment(&text, 0);
        assert_eq!(fragment.byte_offset, 0);
        assert_eq!(fragment.byte_length, text.source.len());
        assert_eq!(fragment.is_new, false);
    }

    #[test]
    fn test_simple_empty_fragment_list() {
        let text = SourceText::new();

        assert_eq!(text.fragments.len(), 1);

        let fragment = get_fragment(&text, 0);
        assert_eq!(fragment.byte_offset, 0);
        assert_eq!(fragment.byte_length, 0);
        assert_eq!(fragment.is_new, false);
    }
}
