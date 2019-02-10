use super::indexed_string::IndexedString;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Source {
    Insertion,
    Original,
}

#[derive(Debug, PartialEq)]
pub struct Fragment {
    pub byte_offset: usize,
    pub byte_length: usize,
    pub source: Source,
}

impl Fragment {
    fn new(source: Source, byte_offset: usize, byte_length: usize) -> Self {
        Fragment {
            byte_length,
            byte_offset,
            source,
        }
    }

    pub fn from_string(text: &IndexedString) -> Self {
        let size = text.len();

        return Fragment::new(Source::Original, 0, size);
    }

    #[allow(dead_code)]
    pub fn of_original(offset: usize, size: usize) -> Self {
        return Fragment::new(Source::Original, offset, size);
    }

    #[allow(dead_code)]
    pub fn of_insertion(offset: usize, size: usize) -> Self {
        return Fragment::new(Source::Insertion, offset, size);
    }

    #[allow(dead_code)]
    pub fn get_slice(&self, source: &IndexedString) -> String {
        let source_text = source.to_string();
        let end_byte = self.byte_offset + self.byte_length;

        return source_text[self.byte_offset..end_byte].to_owned();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let source = IndexedString::from("content");
        let frag = Fragment::from_string(&source);

        assert_eq!(frag.source, Source::Original);
        assert_eq!(frag.byte_length, source.len());
        assert_eq!(frag.byte_offset, 0);
    }

    #[test]
    fn test_get_slice() {
        let source = IndexedString::from("content");
        let frag = Fragment::from_string(&source);

        assert_eq!(frag.get_slice(&source), "content");
    }

    #[test]
    fn test_original_constructor() {
        let frag = Fragment::of_original(1, 5);

        assert_eq!(frag.source, Source::Original);
        assert_eq!(frag.byte_offset, 1);
        assert_eq!(frag.byte_length, 5);
    }

    #[test]
    fn test_insertion_constructor() {
        let frag = Fragment::of_insertion(5, 10);

        assert_eq!(frag.source, Source::Insertion);
        assert_eq!(frag.byte_offset, 5);
        assert_eq!(frag.byte_length, 10);
    }

    #[test]
    fn test_fragment_substring_slice() {
        let frag = Fragment::of_original(2, 5);
        let source = IndexedString::from("first second third");
        let slice = frag.get_slice(&source);

        assert_eq!(slice, "rst s");
    }
}
