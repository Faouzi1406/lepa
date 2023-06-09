use colored::Colorize;

use crate::parser_lexer::lexer::{Operators, Token};

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
        .message("Found invalid function syntax.")
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
        .message("Found invalid function syntax.")
        .line(line)
        .file_name("todo:")
        .helper(format!(
            "Consider adding a body to the function -> fn {name} {}{}{}",
            "{".blue().bold(),
            " <<body>> ".yellow().bold(),
            "}".blue().bold()
        ))
        .build_error()
}

pub fn invalid_var_syntax_token(token: Token) -> ErrorBuilder {
    ErrorBuilder::new()
        .message(format!(
            "Invalid syntax found {} while parsing variable",
            token.value
        ))
        .line(token.line)
        .file_name("todo:")
        .build_error()
}

pub fn invalid_function_call(name: String, line: usize) -> ErrorBuilder {
    ErrorBuilder::new()
        .message(format!(
            "Invalid function call ${name}",
        ))
        .line(line)
        .helper(format!("Found a function call to {}  with no leading OpenBrace en CloseBrace consider changing it to {}(...).", name.yellow().bold(), name.blue().bold()))
        .file_name("todo:")
        .build_error()
}

pub fn invalid_arr_no_end(line: usize) -> ErrorBuilder {
    ErrorBuilder::new()
        .message("Invalid syntax found for array")
        .helper(format!(
            "consider adding a end to the array {}",
            "]".yellow().bold()
        ))
        .line(line)
        .file_name("todo:")
        .build_error()
}

pub fn invalid_return_no_end(line: usize) -> ErrorBuilder {
    ErrorBuilder::new()
        .message("Invalid syntax found for return")
        .helper(format!(
            "consider adding a end to the return statement: {}",
            ";".yellow().bold()
        ))
        .line(line)
        .file_name("todo:")
        .build_error()
}

pub fn invalid_if_statement_operator(token: Operators) -> ErrorBuilder {
    ErrorBuilder::new()
        .message("Found a invalid if statement operator")
        .helper(format!("Found: {:#?}", token))
        .file_name("todo:")
        .build_error()
}
pub fn invalid_if_statement_body(_line: usize) -> ErrorBuilder {
    ErrorBuilder::new()
        .message("Found a invalid if statement, the if statement doesn't have a body.")
        .helper(format!(
            "Consider adding  a body: {:#?}",
            "-> { <<body>> } <-".bold().yellow()
        ))
        .file_name("todo:")
        .build_error()
}
pub fn invalid_use(file: Option<String>, line: usize) -> ErrorBuilder {
    match file {
        Some(file) => ErrorBuilder::new()
            .file_name(format!("Found invalid use statement: use {file}"))
            .helper("Consider adding and ending semicolon")
            .line(line)
            .build_error(),
        None => ErrorBuilder::new()
            .file_name("Found invalid use statement: no filename was found")
            .helper(format!(
                "Consider adding a file name: {}",
                "use \"file\"".bold().yellow()
            ))
            .line(line)
            .build_error(),
    }
}

/// This error occurs whenever we parse a functions argument but find no close:
///
/// # Cases
///
/// Here there is no close so we return the error with the helper: Consider ending a closing brace
/// - (string hello, number hello ...
pub fn args_no_close(file: Option<String>, line: usize) -> ErrorBuilder {
    let mut builder = ErrorBuilder::new();
    if let Some(file) = file {
        builder.file_name(file);
    }
    builder.line(line);
    builder.helper("Add a closing brace -> ) <- to the end of the listed arguments");
    builder.build_error()
}
