use crate::buffer::Buffer;
use std::collections::HashMap;

#[allow(dead_code)]
struct Editor {
    buffers: HashMap<String, Buffer>,
}

impl Editor {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let empty_buffer = Buffer::from_string("");
        let mut buffers = HashMap::new();

        buffers.insert(empty_buffer.id.clone(), empty_buffer);

        Editor { buffers }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_construction() {
        Editor::new();
    }

    #[test]
    fn test_creates_new_buffer() {
        let editor = Editor::new();

        assert_eq!(editor.buffers.len(), 1);
    }
}
