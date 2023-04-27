use colored::Colorize;

use super::error::{BuildError, ErrorBuilder};

/// The error message for a non ending variable
///
/// Non ending variables would be:
///                  
/// - let some = "wow"
///      -> Missing semicolon
///      -> Helper consider adding a semicolon
pub fn non_ending_variable(var: String, line: usize) -> ErrorBuilder {
    ErrorBuilder::new()
        .message(format!(
            "Found a variable without and ending semicolon {}",
            var
        ))
        .line(line)
        .file_name("todo:.rs")
        .helper(format!(
            "Consider adding a semicolon: let {} = var {}",
            "--> ; <--".blue().bold(),
            var
        ))
        .build_error()
}

/// Invalid function syntax
///
/// Invalid function syntax would look something like:
///                  
/// - fn {}
///      -> Missing identifier
///      -> Helper consider adding a identifier.
/// - fn
///     -> fn doesn't mean anything it doesn't have a function body or identifier therefore can't
///     be parsed.
pub fn invalid_function_syntax_missing_id(line: usize) -> ErrorBuilder {
    ErrorBuilder::new()
        .message(format!("Found invalid function syntax."))
        .line(line)
        .file_name("todo:.rs")
        .helper(format!(
            "Consider adding a identifier to the function: {} {}",
            "fn".blue().bold(),
            "hello_world() ".yellow().bold()
        ))
        .build_error()
}

/// Invalid function body syntax
///
/// Invalid function syntax would look something like:
///                  
/// - fn {}
///      -> Missing identifier
///      -> Helper consider adding a identifier.
/// - fn
///     -> fn doesn't mean anything it doesn't have a function body or identifier therefore can't
///     be parsed.
pub fn invalid_function_body_syntax(name: String, line: usize) -> ErrorBuilder {
    ErrorBuilder::new()
        .message(format!("Found invalid function syntax."))
        .line(line)
        .file_name("todo:.rs")
        .helper(format!(
            "Consider adding a body to the function -> fn {name} {}{}{}",
            "{".blue().bold(),
            " <<body>> ".yellow().bold(),
            "}".blue().bold()
        ))
        .build_error()
}
