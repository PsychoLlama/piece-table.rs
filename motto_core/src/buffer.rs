use super::document::Document;
use std::fmt;
use uuid::Uuid;

#[allow(dead_code)]
pub struct Buffer {
    document: Document,
    pub id: String,
}

impl fmt::Display for Buffer {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.document)
    }
}

impl Buffer {
    fn create_id() -> String {
        Uuid::new_v4().to_string()
    }

    #[allow(dead_code)]
    pub fn from_string(content: &str) -> Self {
        Buffer {
            document: Document::from(content),
            id: Buffer::create_id(),
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

    #[test]
    fn test_generates_uuid() {
        let buf1 = Buffer::from_string("content");
        let buf2 = Buffer::from_string("content");

        assert_ne!(buf1.id, buf2.id);
    }
}
