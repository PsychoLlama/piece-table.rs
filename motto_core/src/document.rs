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
    // (split_at_byte, resume_at_byte)
    Split(usize, usize),
    // Remove fragment text from the beginning, end, or both.
    // (trim_start_bytes, trim_end_bytes)
    Trim(usize, usize),
    // Remove the fragment. Metadata just for convenience.
    // (deleted_byte_count)
    Delete(usize),
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

    fn get_fragment_source(&self, fragment: &Fragment) -> &IndexedString {
        return match fragment.source {
            Source::Insertion => &self.insertions,
            Source::Original => &self.original,
        };
    }

    // Find all fragments representing the given byte range.
    #[allow(dead_code)]
    fn find_affected_fragments(&self, start_byte: &usize) -> Vec<Selector> {
        let (start_offset, _) = self
            .fragments
            .range(..=start_byte)
            .rev()
            .next()
            .expect("Empty fragment set");

        return self.fragments.range(start_offset..).collect();
    }

    // Handles 4 cases:
    // 1. `fr>ag` Deletion ends on fragment
    // 2. `fr<ag` Deletion begins on fragment
    // 3. `f<ra>g` Deletion exists in fragment
    // 4. `<prev|frag|next>` Deletion covers fragment
    fn get_operation_for_fragment(&self, ranges: DeletionRange) -> FragmentUpdate {
        // Deletion covers the entire fragment.
        if ranges.deletion.start <= ranges.fragment.start
            && ranges.deletion.end >= ranges.fragment.end
        {
            let Range { start, end } = ranges.fragment;
            return FragmentUpdate {
                operation: FragmentOperation::Delete(end - start),
                move_to: ranges.fragment.start,
                key: ranges.fragment.start,
            };
        }

        // Deletion exists entirely within fragment.
        if ranges.deletion.start > ranges.fragment.start
            && ranges.deletion.end < ranges.fragment.end
        {
            let Range { start, end } = ranges.deletion;

            return FragmentUpdate {
                operation: FragmentOperation::Split(start, end),
                move_to: ranges.fragment.start,
                key: ranges.fragment.start,
            };
        }

        // Deletion partially intersects with fragment.
        let mut trim_start = 0;
        let mut trim_end = 0;

        let Range {
            start: delete_start,
            end: delete_end,
        } = ranges.deletion;

        let Range {
            start: frag_start,
            end: frag_end,
        } = ranges.fragment;

        // If deletion begins in the fragment, trim from the end.
        if delete_start > frag_start && delete_start < frag_end {
            trim_end = frag_end - delete_start;
        }

        // If deletion ends in the fragment, trim from the start.
        if delete_end < frag_end && delete_end > frag_start {
            trim_start = delete_end - frag_start;
        }

        let operation = match (trim_start, trim_end) {
            (0, 0) => FragmentOperation::None,
            (start, end) => FragmentOperation::Trim(start, end),
        };

        return FragmentUpdate {
            move_to: ranges.fragment.start,
            key: ranges.fragment.start,
            operation,
        };
    }

    fn calc_deleted_bytes(&self, op: &FragmentOperation) -> usize {
        match op {
            FragmentOperation::Split(start, end) => *end - *start,
            FragmentOperation::Trim(start, end) => *start + *end,
            FragmentOperation::Delete(bytes) => *bytes,
            _ => 0,
        }
    }

    #[allow(dead_code)]
    fn get_changes_for_deletion(&self, deletion_range: &Range<usize>) -> Vec<FragmentUpdate> {
        let frags = self.find_affected_fragments(&deletion_range.start);

        let mut deleted_bytes = 0;
        return frags
            .iter()
            .map(|(start_offset, frag)| {
                let frag_end_offset = *start_offset + frag.byte_length;

                self.get_operation_for_fragment(DeletionRange {
                    fragment: **start_offset..frag_end_offset,
                    deletion: deletion_range.clone(),
                })
            })
            .map(|mut update| {
                update.move_to = update.key - deleted_bytes;

                let operation = &update.operation;
                deleted_bytes += self.calc_deleted_bytes(&operation);

                return update;
            })
            .collect();
    }

    // Danger: fragment mutation and resizing zone.
    // Remember not to confuse fragment offsets with derived offsets.
    fn apply_change(&mut self, change: &FragmentUpdate) -> Option<()> {
        match change.operation {
            FragmentOperation::Delete(_) => {
                self.fragments.remove(&change.key);
            }

            FragmentOperation::Trim(start, end) => {
                let mut frag = self.fragments.remove(&change.key)?;
                let new_offset = frag.byte_offset + start;
                let new_length = frag.byte_length - start - end;

                frag.resize(new_offset, new_length);

                self.fragments.insert(change.move_to, frag);
            }

            FragmentOperation::Split(stop, resume) => {
                let mut left = self.fragments.remove(&change.key)?;
                let frag_offset_diff = resume - change.key;

                let right_frag_byte_offset = left.byte_offset + frag_offset_diff;
                let right_frag_byte_length = left.byte_length - frag_offset_diff;

                let right = Fragment::new(
                    left.source.clone(),
                    right_frag_byte_offset,
                    right_frag_byte_length,
                );

                left.resize(left.byte_offset, stop - change.key);

                self.fragments.insert(change.move_to, left);
                self.fragments.insert(resume, right);
            }
            _ => {}
        }

        Some(())
    }

    fn apply_changes(&mut self, changes: &Vec<FragmentUpdate>) -> Option<()> {
        for change in changes {
            self.apply_change(&change);
        }

        Some(())
    }

    #[allow(dead_code)]
    pub fn delete(&mut self, range: &Range<usize>) {
        let changes = self.get_changes_for_deletion(&range);
        self.apply_changes(&changes);
    }
}

impl fmt::Display for Document {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let result: String = self
            .fragments
            .iter()
            .map(|(_, frag)| (frag, self.get_fragment_source(&frag)))
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
    fn test_find_affected_fragments() {
        let mut text = Document::from("original");
        text.insert(8, " with");
        text.insert(13, " insertions");

        assert_eq!(text.find_affected_fragments(&0).len(), 3);
        assert_eq!(text.find_affected_fragments(&1).len(), 3);
        assert_eq!(text.find_affected_fragments(&7).len(), 3);
        assert_eq!(text.find_affected_fragments(&8).len(), 2);
        assert_eq!(text.find_affected_fragments(&13).len(), 1);
    }

    #[test]
    fn test_fragment_delete_at_end_operation() {
        let mut text = Document::from("original");
        text.insert(8, " with");
        text.insert(13, " insertions");

        assert_eq!(
            text.get_changes_for_deletion(&(15..24)),
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
            text.get_changes_for_deletion(&(13..22)),
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
            text.get_changes_for_deletion(&(13..24)),
            vec![FragmentUpdate {
                operation: FragmentOperation::Delete(11),
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
            text.get_changes_for_deletion(&(8..32)),
            vec![
                FragmentUpdate {
                    operation: FragmentOperation::Delete(5),
                    move_to: 8,
                    key: 8,
                },
                FragmentUpdate {
                    operation: FragmentOperation::Delete(11),
                    move_to: 8,
                    key: 13,
                }
            ]
        );
    }

    #[test]
    fn test_delete_middle_of_fragment() {
        let mut text = Document::from("original");
        text.insert(8, " with");
        text.insert(13, " insertions");

        assert_eq!(
            text.get_changes_for_deletion(&(15..20)),
            vec![FragmentUpdate {
                operation: FragmentOperation::Split(15, 20),
                move_to: 13,
                key: 13,
            }]
        );
    }

    #[test]
    fn test_delete_fragment_beginning() {
        let mut text = Document::from("text");
        text.insert(5, " with fragments");
        text.delete(&(5..10));

        assert_eq!(text.to_string(), "text fragments");
    }

    #[test]
    fn test_deletion_adjusts_later_elements() {
        let mut text = Document::from("original");
        text.insert(8, " with");
        text.insert(13, " insertions");

        assert_eq!(
            text.get_changes_for_deletion(&(10..13)),
            vec![
                FragmentUpdate {
                    operation: FragmentOperation::Trim(0, 3),
                    move_to: 8,
                    key: 8,
                },
                FragmentUpdate {
                    operation: FragmentOperation::None,
                    move_to: 10,
                    key: 13,
                }
            ]
        );
    }

    #[test]
    fn test_deletion_split_adjusts_later_elements() {
        let mut text = Document::from("original");
        text.insert(8, " with");
        text.insert(13, " insertions");

        assert_eq!(
            text.get_changes_for_deletion(&(1..3)),
            vec![
                FragmentUpdate {
                    operation: FragmentOperation::Split(1, 3),
                    move_to: 0,
                    key: 0,
                },
                FragmentUpdate {
                    operation: FragmentOperation::None,
                    move_to: 6,
                    key: 8,
                },
                FragmentUpdate {
                    operation: FragmentOperation::None,
                    move_to: 11,
                    key: 13,
                },
            ]
        );
    }

    #[test]
    fn test_delete_removes_deleted_fragments() {
        let mut text = Document::from("original");
        text.insert(8, " with");
        text.insert(13, " insertions");
        text.delete(&(13..24));

        assert_eq!(text.to_string(), "original with");
    }

    #[test]
    fn test_delete_trims_truncated_fragments() {
        let mut text = Document::from("original");
        text.insert(8, " with");
        text.insert(13, " insertions");
        text.delete(&(15..24));

        assert_eq!(text.to_string(), "original with i");
    }

    #[test]
    fn test_delete_can_split_fragments() {
        let mut text = Document::from("original");
        text.insert(8, " with");
        text.insert(13, " insertions");

        text.delete(&(14..20));

        assert_eq!(text.to_string(), "original with ions");
    }

    #[test]
    fn test_delete_works_across_fragments() {
        let mut text = Document::from("original");
        text.insert(8, " with");
        text.insert(13, " insertions");

        text.delete(&(7..19));

        assert_eq!(text.to_string(), "originations");
    }
}
