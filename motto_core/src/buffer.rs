use super::document::Document;
use std::fmt;

struct Buffer {
    document: Document,
}

impl fmt::Display for Buffer {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.document)
    }
}

impl Buffer {
    #[allow(dead_code)]
    pub fn from_string(content: &str) -> Self {
        Buffer {
            document: Document::from(content),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction_from_string() {
        let buffer = Buffer::from_string("content");

        assert_eq!(buffer.to_string(), "content".to_owned());
    }
}
