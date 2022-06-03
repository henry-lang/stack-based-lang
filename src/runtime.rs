use crate::parser::Func;
use crate::parser::Program;
use crate::stack::Stack;
use std::collections::HashMap;

pub enum Value<'a> {
    Number(i64),
    String(&'a str), // For now the only string literals are at "compile time", and can't be dynamically created
}

impl<'a> std::fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{}", num),
            Self::String(string) => write!(f, "{}", string),
        }
    }
}

type BuiltIn = fn(&mut Stack<Value>);

pub struct Runtime {
    funcs: HashMap<String, Func>,
    builtins: HashMap<&'static str, BuiltIn>,
}

impl Runtime {
    pub fn insert_builtins(&mut self) {
        self.builtins
            .insert(".", |stack| println!("{}", stack.pop()));
        self.builtins.insert("+", |stack| {
            let operands = stack.pop_several::<2>();

            match operands {
                [Value::Number(lhs), Value::Number(rhs)] => stack.push(Value::Number(lhs + rhs)),
                _ => panic!(),
            }
        });
        self.builtins.insert("-", |stack| {
            let operands = stack.pop_several::<2>();

            match operands {
                [Value::Number(lhs), Value::Number(rhs)] => stack.push(Value::Number(lhs - rhs)),
                _ => panic!(),
            }
        });
        self.builtins.insert("*", |stack| {
            let operands = stack.pop_several::<2>();

            match operands {
                [Value::Number(lhs), Value::Number(rhs)] => stack.push(Value::Number(lhs * rhs)),
                _ => panic!(),
            }
        });
        self.builtins.insert("/", |stack| {
            let operands = stack.pop_several::<2>();

            match operands {
                [Value::Number(lhs), Value::Number(rhs)] => stack.push(Value::Number(lhs / rhs)),
                _ => panic!(),
            }
        });
    }

    pub fn new(program: Program) -> Self {
        let mut it = Self {
            funcs: HashMap::new(),
            builtins: HashMap::new(),
        };

        it.insert_builtins();

        it
    }
}
