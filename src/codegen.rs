use crate::error::{CompileError, CompileResult};
use crate::parser::Program;

static ENTRY_POINT: &str = "main";

pub fn codegen(program: &Program) -> CompileResult<String> {
    let entry = program.get_func(ENTRY_POINT);
    if entry.is_none() {
        return Err(CompileError::General(
            "main entry point is not found, try adding \\main {}".into(),
        ));
    }

    Ok("".into())
}
