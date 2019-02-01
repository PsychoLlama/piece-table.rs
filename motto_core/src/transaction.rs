#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(dead_code)]
pub enum Operation {
    Insertion(usize, String),
    Deletion(usize, u32),
}

pub struct Transaction {
    pub operations: Vec<Operation>,
}

impl Transaction {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Transaction {
            operations: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn from(operations: Vec<Operation>) -> Self {
        Transaction { operations }
    }

    #[allow(dead_code)]
    pub fn append_operation(&mut self, op: Operation) {
        self.operations.push(op);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let tsc = Transaction::new();

        assert_eq!(tsc.operations.len(), 0);
    }

    #[test]
    fn test_construction_with_initial_value() {
        let ins = Operation::Insertion(1, "no".to_string());
        let del = Operation::Deletion(0, 1);
        let tsc = Transaction::from(vec![ins, del]);

        assert_eq!(tsc.operations.len(), 2);
    }

    #[test]
    fn test_add_operation() {
        let mut tsc = Transaction::new();
        let op = Operation::Insertion(0, "Text".to_string());
        tsc.append_operation(op.clone());

        assert_eq!(tsc.operations.len(), 1);
        assert_eq!(tsc.operations[0], op);
    }

    #[test]
    fn test_operations_append() {
        let mut tsc = Transaction::new();
        let ins = Operation::Insertion(0, "Content".to_string());
        let del = Operation::Deletion(0, 3);

        tsc.append_operation(ins);
        tsc.append_operation(del.clone());

        assert_eq!(tsc.operations.len(), 2);
        assert_eq!(tsc.operations[1], del);
    }
}
