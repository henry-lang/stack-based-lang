use crate::parser::Program;
use crate::stack::Stack;
use std::collections::HashMap;

pub enum Value<'a> {
    Number(i64),
    String(&'a str), // For now the only string literals are at "compile time", and can't be dynamically created
}

impl<'a> std::fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Number(num) => &num.to_string(),
                Self::String(string) => *string,
            }
        )
    }
}

type BuiltIn = fn(&Stack<Value>);

pub struct Runtime<'a> {
    program: &'a Program<'a>,
    builtins: HashMap<&'static str, BuiltIn>,
}

impl<'a> Runtime<'a> {
    pub fn new(program: &'a Program) -> Self {
        let mut builtins = HashMap::new();

        builtins.insert(".", |stack: &Stack<Value>| println!("{}", stack.pop()));

        Self { program, builtins }
    }
}
