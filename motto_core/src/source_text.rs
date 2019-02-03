use super::indexed_string::IndexedString;
use std::fmt;

#[derive(Debug)]
struct Fragment {
    byte_offset: usize,
    byte_length: usize,
    lines: Vec<usize>,
    is_new: bool,
}

impl Fragment {
    fn new(is_new: bool, byte_offset: usize, byte_length: usize, source: &IndexedString) -> Self {
        let ending_offset = byte_offset + byte_length;
        let lines = source.select_relative_linebreaks(byte_offset, ending_offset);

        Fragment {
            byte_length,
            byte_offset,
            is_new,
            lines,
        }
    }

    pub fn of_insertion(offset: usize, size: usize, text: &IndexedString) -> Self {
        return Fragment::new(true, offset, size, text);
    }

    pub fn of_original(offset: usize, size: usize, text: &IndexedString) -> Self {
        return Fragment::new(false, offset, size, text);
    }

    // Get the full text of the fragment.
    pub fn to_string(&self, text: &SourceText) -> String {
        return self.slice(&text, 0, self.byte_length);
    }

    #[cfg(any(debug, test))]
    fn check_slice_bounds(&self, end: &usize) {
        let max = self.byte_offset + self.byte_length;

        if end > &max {
            panic!(
                "fragment.slice(..) was out of range by {} byte(s)",
                end - &self.byte_length
            );
        }
    }

    // Get a smaller piece of the fragment's text.
    pub fn slice(&self, text: &SourceText, start: usize, end: usize) -> String {
        let source = text.get_source_text(self.is_new);
        let start_byte = self.byte_offset + start;
        let end_byte = self.byte_offset + end;

        #[cfg(any(debug, test))]
        self.check_slice_bounds(&end_byte);

        return source[start_byte..end_byte].to_owned();
    }
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
        Fragment::of_original(0, source.len(), &source)
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

    fn make_insert_fragment(&mut self, insertion: &str) -> Fragment {
        let offset = self.insertions.len();
        let length = insertion.len();

        self.insertions.append(insertion);

        let fragment = Fragment::of_insertion(offset, length, &self.insertions);

        return fragment;
    }

    // Find the fragment index and relative fragment byte offset. Do this by
    // iterating over the fragments, keeping track of their byte length, then
    // returning information about the first fragment that fits in the
    // byte_offset range.
    fn find_insertion_target(&self, byte_offset: usize) -> Option<(usize, usize)> {
        let mut bytes_already_searched = 0;

        for (index, fragment) in self.fragments.iter().enumerate() {
            let relative_offset = byte_offset - bytes_already_searched;
            bytes_already_searched += fragment.byte_length;

            // Is this the insertion?
            if relative_offset <= fragment.byte_length {
                return Some((index, relative_offset));
            }
        }

        None
    }

    fn split_fragment(&mut self, indices: (usize, usize), fragment: Fragment) {
        let (fragment_index, fragment_offset) = indices;
        let target_fragment = &self.fragments[fragment_index];

        let new_fragments = vec![
            Fragment {
                byte_offset: target_fragment.byte_offset,
                is_new: target_fragment.is_new,
                byte_length: fragment_offset,
                lines: Vec::new(),
            },
            fragment,
            Fragment {
                byte_offset: target_fragment.byte_offset + fragment_offset,
                byte_length: target_fragment.byte_length - fragment_offset,
                is_new: target_fragment.is_new,
                lines: Vec::new(),
            },
        ];

        self.fragments
            .splice(fragment_index..fragment_index + 1, new_fragments)
            .for_each(drop);
    }

    fn apply_insert(&mut self, indices: (usize, usize), fragment: Fragment) {
        let (fragment_index, fragment_offset) = indices;
        let target_fragment = &self.fragments[fragment_index];

        // Prepend operation.
        if target_fragment.byte_offset == fragment_offset {
            return self.fragments.insert(0, fragment);
        }

        // Append operation.
        if target_fragment.byte_length == fragment_offset {
            return self.fragments.push(fragment);
        }

        self.split_fragment(indices, fragment);
    }

    fn get_source_text(&self, is_new: bool) -> String {
        let source = match is_new {
            true => &self.insertions,
            false => &self.source,
        };

        return source.to_string();
    }

    #[allow(dead_code)]
    pub fn insert(&mut self, byte_offset: usize, text: &str) {
        let inserted_fragment = self.make_insert_fragment(text);
        let indices = match self.find_insertion_target(byte_offset) {
            Some(indices) => indices,
            None => return,
        };

        self.apply_insert(indices, inserted_fragment);
    }
}

impl fmt::Display for SourceText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string_bytes = self
            .fragments
            .iter()
            .fold(0, |sum, frag| sum + frag.byte_length);

        let mut result = String::with_capacity(string_bytes);

        for fragment in &self.fragments {
            result.push_str(&fragment.to_string(&self));
        }

        write!(f, "{}", result)
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

    #[test]
    fn test_appending_insert() {
        let mut text = SourceText::from("a b");
        text.insert(3, " c");

        assert_eq!(text.insertions.to_string(), " c");
        assert_eq!(text.fragments.len(), 2);
        assert_eq!(get_fragment(&text, 0).byte_length, 3);
        assert_eq!(get_fragment(&text, 1).byte_length, 2);
        assert_eq!(get_fragment(&text, 1).byte_offset, 0);
        assert_eq!(get_fragment(&text, 1).is_new, true);
    }

    #[test]
    fn test_middle_insert() {
        let mut text = SourceText::from("a c");
        text.insert(2, "b ");

        assert_eq!(text.fragments.len(), 3);
        assert_eq!(text.insertions.len(), 2);
        assert_eq!(get_fragment(&text, 0).byte_offset, 0);
        assert_eq!(get_fragment(&text, 0).byte_length, 2);
        assert_eq!(get_fragment(&text, 0).is_new, false);

        assert_eq!(get_fragment(&text, 1).byte_offset, 0);
        assert_eq!(get_fragment(&text, 1).byte_length, 2);
        assert_eq!(get_fragment(&text, 1).is_new, true);

        assert_eq!(get_fragment(&text, 2).byte_offset, 2);
        assert_eq!(get_fragment(&text, 2).byte_length, 1);
        assert_eq!(get_fragment(&text, 2).is_new, false);
    }

    #[test]
    fn test_prepending_insert() {
        let mut text = SourceText::from("world");
        text.insert(0, "hello ");

        assert_eq!(text.fragments.len(), 2);
        assert_eq!(get_fragment(&text, 0).is_new, true);
        assert_eq!(get_fragment(&text, 0).byte_offset, 0);
        assert_eq!(get_fragment(&text, 0).byte_length, 6);

        assert_eq!(get_fragment(&text, 1).is_new, false);
        assert_eq!(get_fragment(&text, 1).byte_offset, 0);
        assert_eq!(get_fragment(&text, 1).byte_length, 5);
    }

    #[test]
    fn test_derive_source() {
        let mut text = SourceText::from("hello world");
        text.insert(6, "weird ");

        assert_eq!(text.to_string(), "hello weird world");
    }
}
