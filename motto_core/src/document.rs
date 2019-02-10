use super::fragment::{Fragment, Source};
use super::indexed_string::IndexedString;
use std::{collections::BTreeMap, fmt, ops::Range};

type Selector<'a> = (&'a usize, &'a Fragment);

#[derive(Debug)]
struct DeletionRange {
    fragment: Range<usize>,
    deletion: Range<usize>,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum FragmentOperation {
    // Insert a fragment into another fragment.
    // (byte_offset_from_start, Fragment)
    Insert(usize, Fragment),
    // Split the fragment into two parts.
    // (split_at_byte, deleted_length)
    Split(usize, usize),
    // Remove fragment text from the beginning, end, or both.
    // (trim_start_bytes, trim_end_bytes)
    Trim(usize, usize),
    // Remove the fragment.
    Delete,
    // No change.
    None,
}

#[derive(PartialEq, Debug)]
struct FragmentUpdate {
    operation: FragmentOperation,
    move_to: usize,
    key: usize,
}

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

    fn get_fragment_source_string(&self, fragment: &Fragment) -> &IndexedString {
        return match fragment.source {
            Source::Insertion => &self.insertions,
            Source::Original => &self.original,
        };
    }

    // Find all fragments representing the given byte range.
    #[allow(dead_code)]
    fn find_fragments_in_range(&self, range: &Range<usize>) -> Vec<Selector> {
        let (start_offset, _) = self
            .fragments
            .range(..=range.start)
            .rev()
            .next()
            .expect("Empty fragment set");

        return self.fragments.range(start_offset..=&range.end).collect();
    }

    fn get_operation_for_fragment(&self, ranges: DeletionRange) -> FragmentUpdate {
        // Deletion covers the entire fragment.
        if ranges.deletion.start <= ranges.fragment.start
            && ranges.deletion.end >= ranges.fragment.end
        {
            return FragmentUpdate {
                operation: FragmentOperation::Delete,
                move_to: ranges.fragment.start,
                key: ranges.fragment.start,
            };
        }

        // TODO: Deletion exists entirely within fragment.

        // Fragment intersects with deletion range.
        let mut trim_start = 0;
        let mut trim_end = 0;

        // If deletion starts into the fragment, trim from the end.
        if ranges.deletion.start > ranges.fragment.start
            && ranges.deletion.start < ranges.fragment.end
        {
            trim_end = ranges.fragment.end - ranges.deletion.start;
        }

        // If deletion ends into the fragment, trim from the start.
        if ranges.deletion.end < ranges.fragment.end && ranges.deletion.end > ranges.fragment.start
        {
            trim_start = ranges.deletion.end - ranges.fragment.start;
        }

        return FragmentUpdate {
            operation: FragmentOperation::Trim(trim_start, trim_end),
            move_to: ranges.fragment.start,
            key: ranges.fragment.start,
        };
    }

    #[allow(dead_code)]
    fn get_delete_fragment_operations(&self, deletion_range: Range<usize>) -> Vec<FragmentUpdate> {
        let frags = self.find_fragments_in_range(&deletion_range);

        return frags
            .iter()
            .map(|(start_offset, frag)| {
                let frag_end_offset = *start_offset + frag.byte_length;

                // TODO: track deletions to compensate for position.
                self.get_operation_for_fragment(DeletionRange {
                    fragment: **start_offset..frag_end_offset,
                    deletion: deletion_range.clone(),
                })
            })
            .collect();
    }
}

impl fmt::Display for Document {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let result: String = self
            .fragments
            .iter()
            .map(|(_, frag)| (frag, self.get_fragment_source_string(&frag)))
            .map(|(frag, source)| frag.get_slice(&source))
            .collect();

        return write!(fmt, "{}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_fragment_tuple(text: &Document, index: usize) -> Selector {
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

    #[test]
    fn test_find_fragments_in_range() {
        let mut text = Document::from("original");
        text.insert(8, " with");
        text.insert(13, " insertions");

        assert_eq!(text.find_fragments_in_range(&(0..7)).len(), 1);
        assert_eq!(text.find_fragments_in_range(&(0..8)).len(), 2);
        assert_eq!(text.find_fragments_in_range(&(0..12)).len(), 2);
        assert_eq!(text.find_fragments_in_range(&(0..13)).len(), 3);
        assert_eq!(text.find_fragments_in_range(&(0..23)).len(), 3);
        assert_eq!(text.find_fragments_in_range(&(8..23)).len(), 2);
        assert_eq!(text.find_fragments_in_range(&(13..23)).len(), 1);
    }

    #[test]
    fn test_fragment_delete_at_end_operation() {
        let mut text = Document::from("original");
        text.insert(8, " with");
        text.insert(13, " insertions");

        assert_eq!(
            text.get_delete_fragment_operations(15..24),
            vec![FragmentUpdate {
                operation: FragmentOperation::Trim(0, 9),
                move_to: 13,
                key: 13,
            }]
        );
    }

    #[test]
    fn test_fragment_delete_at_beginning() {
        let mut text = Document::from("original");
        text.insert(8, " with");
        text.insert(13, " insertions");

        assert_eq!(
            text.get_delete_fragment_operations(13..22),
            vec![FragmentUpdate {
                operation: FragmentOperation::Trim(9, 0),
                move_to: 13,
                key: 13,
            }]
        );
    }

    #[test]
    fn test_fragment_delete_entire_fragment() {
        let mut text = Document::from("original");
        text.insert(8, " with");
        text.insert(13, " insertions");

        assert_eq!(
            text.get_delete_fragment_operations(13..24),
            vec![FragmentUpdate {
                operation: FragmentOperation::Delete,
                move_to: 13,
                key: 13,
            }]
        );
    }

    #[test]
    fn test_delete_multiple_fragments() {
        let mut text = Document::from("original");
        text.insert(8, " with");
        text.insert(13, " insertions");

        assert_eq!(
            text.get_delete_fragment_operations(8..32),
            vec![
                FragmentUpdate {
                    operation: FragmentOperation::Delete,
                    move_to: 8,
                    key: 8,
                },
                FragmentUpdate {
                    operation: FragmentOperation::Delete,
                    move_to: 13,
                    key: 13,
                }
            ]
        );
    }
}
