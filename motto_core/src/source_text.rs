use super::indexed_string::IndexedString;

#[allow(dead_code)]
pub struct SourceText {
    text: IndexedString,
}

impl SourceText {
    #[allow(dead_code)]
    pub fn new() -> SourceText {
        SourceText {
            text: IndexedString::new(),
        }
    }

    #[allow(dead_code)]
    pub fn from(text: &str) -> SourceText {
        SourceText {
            text: IndexedString::from(text),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_text_construction() {
        let st = SourceText::new();

        assert_eq!(st.text.len(), 0);
    }

    #[test]
    fn test_filled_construction() {
        let text = "Initial value";
        let st = SourceText::from(text);

        assert_eq!(st.text.len(), text.len());
    }
}
