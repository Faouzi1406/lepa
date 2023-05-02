use std::fmt::Display;

/// ErrorBuilder,
///
/// It allows for building error messages based on:
///
/// - Filename : File in which error happened
///
/// - Message : The error message
///
/// - Helper : A help message explaining how the error could be resolved.
///
/// - Line : The line on which the error took place
#[derive(Debug, Clone)]
pub struct ErrorBuilder {
    file_name: String,
    message_: String,
    helper_: Option<String>,
    line: usize,
}

impl Display for ErrorBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = format!(" File -> {:#?}\n", self.file_name,);
        let message = format!(" Message -> {}\n", self.message_);
        let helps = match &self.helper_ {
            Some(msg) => format!(" Help -> {msg}\n"),
            None => "".into(),
        };
        let line = format!(" line -> {}\n", self.line);
        write!(
            f,
            "_________________\n{file}{message}{helps}{line}\n__________________"
        )
    }
}

impl std::error::Error for ErrorBuilder {}

pub trait BuildError {
    fn new() -> Self;
    fn file_name(&mut self, file_name: impl AsRef<str>) -> &mut Self;
    fn message(&mut self, mess: impl AsRef<str>) -> &mut Self;
    fn helper(&mut self, help_message: impl AsRef<str>) -> &mut Self;
    fn line(&mut self, line: usize) -> &mut Self;
    /// Used to build the error into a string
    fn build(&mut self) -> String;
    /// Used to build the error into a non mutuable version of the error
    fn build_error(&mut self) -> Self;
}

impl BuildError for ErrorBuilder {
    fn new() -> Self {
        Self {
            file_name: String::new(),
            message_: String::new(),
            helper_: None,
            line: 0,
        }
    }

    fn file_name(&mut self, file_name: impl AsRef<str>) -> &mut Self {
        self.file_name = file_name.as_ref().into();
        self
    }

    fn message(&mut self, mess: impl AsRef<str>) -> &mut Self {
        self.message_ = mess.as_ref().into();
        self
    }

    fn helper(&mut self, help_message: impl AsRef<str>) -> &mut Self {
        self.helper_ = Some(help_message.as_ref().into());
        self
    }

    fn line(&mut self, line: usize) -> &mut Self {
        self.line = line;
        self
    }
    fn build(&mut self) -> String {
        format!("{self}")
    }
    fn build_error(&mut self) -> Self {
        self.clone()
    }
}
