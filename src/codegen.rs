use crate::error::{CompileError, CompileResult};
use crate::parser::{Func, Program, Statement};
use crate::tokenizer::Spanned;

static ENTRY_POINT: &str = "main";
macro_rules! template {
    () => {
r#"
#include <stdio.h>
#include <stdlib.h>

typedef enum ValueType {{
    ValueNumber,
    ValueString,
    ValueFunc
}} ValueType;

typedef struct Value {{
    union {{
        double number;
        char *string;
        void (*func) ();        
    }} data;

    ValueType type;
}} Value;

typedef struct Stack {{
    Value *data;
    size_t length;
    size_t capacity;
}} Stack;

Stack stack;

int main() {{
    
}}
"#
    };
}

pub struct Codegen<'a> {
    program: &'a Program,
}

impl<'a> Codegen<'a> {
    pub fn new(program: &'a Program) -> Self {
        Self { program }
    }

    fn gen_push_number() {

    }

    fn gen_func(&mut self, func: &Func) {
        use Statement::*;

        for Spanned {
            value: statement, ..
        } in func.statements()
        {
            match statement {
                PushNumber(num) => self.gen_push_number();
                PushString(string) => self.gen_push_string();
            }
        }
    }

    pub fn gen(&mut self) -> CompileResult<String> {
        let entry = self.program.has_func(ENTRY_POINT);

        match entry {
            true => {
                Ok("".into())
            },
            false => Err(CompileError::General(
                "main entry point is not found, try adding \\main {}".into(),
            ))
        }
    }
}
