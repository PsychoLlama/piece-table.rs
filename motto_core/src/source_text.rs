use super::transaction::{Operation, Transaction};

pub struct SourceText {
    text: String,
}

impl SourceText {
    fn new() -> SourceText {
        SourceText {
            text: String::new(),
        }
    }

    fn from(text: &str) -> SourceText {
        SourceText {
            text: text.to_string(),
        }
    }

    fn apply_transaction(&mut self, tsc: &Transaction) {
        for operation in &tsc.operations {
            match operation {
                Operation::Insertion(byte_index, text) => self.insert(byte_index, text),
                _ => {}
            }
        }
    }

    fn insert(&mut self, byte_index: &usize, text: &String) {
        self.text.insert_str(*byte_index, text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn insert(byte_index: usize, content: &str) -> Operation {
        Operation::Insertion(byte_index, content.to_string())
    }

    #[test]
    fn test_empty_text_construction() {
        let st = SourceText::new();

        assert_eq!(st.text.len(), 0);
    }

    #[test]
    fn test_filled_construction() {
        let text = "Initial value";
        let st = SourceText::from(text);

        assert_eq!(st.text, text);
    }

    #[test]
    fn test_leading_insertion() {
        let mut st = SourceText::new();
        let tsc = Transaction::from(vec![insert(0, "Text")]);
        st.apply_transaction(&tsc);

        assert_eq!(st.text, "Text".to_string());
    }

    #[test]
    fn test_append_insertion() {
        let mut st = SourceText::from("Hello");
        let tsc = Transaction::from(vec![insert(5, " world!")]);
        st.apply_transaction(&tsc);

        assert_eq!(st.text, "Hello world!".to_string());
    }
}
