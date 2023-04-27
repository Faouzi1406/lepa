use super::error::{BuildError, ErrorBuilder};

pub fn non_ending_variable(var: String, line: usize) -> ErrorBuilder {
    ErrorBuilder::new()
        .message(format!(
            "Found a variable without and ending semicolon {}",
            var
        ))
        .line(line)
        .file_name("todo:.rs")
        .helper(format!("Consider adding a semicolon: let {} = var ;", var))
        .build_error()
}
