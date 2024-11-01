use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    id: usize,
    stack: Vec<Value>,
    words: HashMap<String, usize>,
    procedures: HashMap<usize, Vec<Operation>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(Clone)]
enum Operation {
    Push(Value),
    Suboperation(usize),
    EndSuboperation,
    Add,
    Subtract,
    Multiply,
    Divide,
    Duplicate,
    Drop,
    Swap,
    Overduplicate,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            id: 0,
            stack: Vec::new(),
            words: HashMap::new(),
            procedures: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.as_slice()
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let input = input.to_uppercase();
        let mut iter = input.split_ascii_whitespace().peekable();
        match iter.peek() {
            // New word definition
            Some(&":") => {
                iter.next();
                let suboperation = iter.next().ok_or(Error::InvalidWord)?;
                if suboperation.parse::<Value>().is_ok() {
                    return Err(Error::InvalidWord);
                }
                let mut instructions = vec![];
                loop {
                    match iter.next().map(|i| self.parse_operation(i)) {
                        Some(Ok(Operation::EndSuboperation)) => break,
                        Some(Ok(instruction)) => {
                            instructions.push(instruction);
                        }
                        Some(Err(e)) => return Err(e),
                        None => return Err(Error::InvalidWord),
                    }
                }
                if iter.next().is_some() {
                    return Err(Error::InvalidWord);
                } else {
                    self.words.insert(suboperation.to_string(), self.id);
                    self.procedures.insert(self.id, instructions);
                    self.id += 1;
                }
                return Ok(());
            }
            None => return Ok(()),
            Some(_) => (),
        }
        // Expression evaluation
        for operation in iter {
            let operation = self.parse_operation(operation)?;
            self.execute_operation(&operation)?;
        }
        Ok(())
    }

    fn pop(&mut self) -> std::result::Result<Value, Error> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }

    fn parse_operation(&mut self, input: &str) -> std::result::Result<Operation, Error> {
        match (input, input.parse::<Value>()) {
            (_, Ok(value)) => Ok(Operation::Push(value)),
            (suboperation, _) if self.words.contains_key(suboperation) => Ok(
                Operation::Suboperation(*self.words.get(suboperation).unwrap()),
            ),
            ("+", _) => Ok(Operation::Add),
            ("-", _) => Ok(Operation::Subtract),
            ("*", _) => Ok(Operation::Multiply),
            ("/", _) => Ok(Operation::Divide),
            ("DUP", _) => Ok(Operation::Duplicate),
            ("DROP", _) => Ok(Operation::Drop),
            ("SWAP", _) => Ok(Operation::Swap),
            ("OVER", _) => Ok(Operation::Overduplicate),
            (";", _) => Ok(Operation::EndSuboperation),
            (":", _) => Err(Error::InvalidWord),
            _ => Err(Error::UnknownWord),
        }
    }

    fn execute_operation(&mut self, operation: &Operation) -> Result {
        match operation {
            Operation::Push(value) => {
                self.stack.push(*value);
            }
            Operation::Add => {
                let b = self.pop()?;
                let a = match self.pop() {
                    Err(e) => {
                        self.stack.push(b);
                        return Err(e);
                    }
                    Ok(a) => a,
                };
                self.stack.push(a + b);
            }
            Operation::Subtract => {
                let b = self.pop()?;
                let a = match self.pop() {
                    Err(e) => {
                        self.stack.push(b);
                        return Err(e);
                    }
                    Ok(a) => a,
                };
                self.stack.push(a - b);
            }
            Operation::Multiply => {
                let b = self.pop()?;
                let a = match self.pop() {
                    Err(e) => {
                        self.stack.push(b);
                        return Err(e);
                    }
                    Ok(a) => a,
                };
                self.stack.push(a * b);
            }
            Operation::Divide => {
                let b = self.pop()?;
                if b == 0 {
                    self.stack.push(b);
                    return Err(Error::DivisionByZero);
                }
                let a = match self.pop() {
                    Err(e) => {
                        self.stack.push(b);
                        return Err(e);
                    }
                    Ok(a) => a,
                };
                self.stack.push(a / b);
            }
            Operation::Duplicate => {
                let a = self.stack.last().ok_or(Error::StackUnderflow)?;
                self.stack.push(*a);
            }
            Operation::Drop => {
                self.stack.pop().ok_or(Error::StackUnderflow)?;
            }
            Operation::Swap => {
                let b = self.pop()?;
                let a = match self.pop() {
                    Err(e) => {
                        self.stack.push(b);
                        return Err(e);
                    }
                    Ok(a) => a,
                };
                self.stack.push(b);
                self.stack.push(a);
            }
            Operation::Overduplicate => {
                let index = self
                    .stack
                    .len()
                    .checked_sub(2)
                    .ok_or(Error::StackUnderflow)?;
                let a = self.stack[index];
                self.stack.push(a);
            }
            Operation::EndSuboperation => return Err(Error::InvalidWord),
            Operation::Suboperation(suboperation) => {
                let instructions = self
                    .procedures
                    .get(suboperation)
                    .ok_or(Error::UnknownWord)?;
                for instruction in instructions.clone().iter() {
                    self.execute_operation(instruction)?;
                }
            }
        }
        Ok(())
    }
}
