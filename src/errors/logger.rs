use std::fmt::Debug;

use colored::Colorize;

/// The logger struct, this can be used to log events to the terminal.
///
/// It contains some metadata used for logging, in this case the LogLevel.
pub struct Logger(pub LogLevels);

/// The log levels of and error,
///
/// # Example of warning LogLevels
///
/// ```Rust
/// use leppa::errors::errors::{Error, LogLevels};
///
/// let log = Error(LogLevels:Warning);
///
/// log.info("This wont be logged"); // This  wont be logged
/// log.warning("This is a warning..."); // will be logged
/// log.error("This is a error..."); // will be logged
/// ```
pub enum LogLevels {
    /// ```Rust
    /// LogLevels::Info // logs everything, that would be and error, warning, and Info.
    /// ```
    Info,
    /// ```Rust
    /// LogLevels::Warning // logs everything after warning, meaning error and warning.
    /// ```
    Warning,
    /// ```Rust
    /// LogLevels::Error // logs only errors.
    /// ```
    Error,
}

/// The Log trait.
///
/// It contains all the functions for the logger, it can be used to implement the functions
/// anywhere T implements Debug.
///
/// T must implement Debug, because we assume that in the case of a log event, T is allowed to be printed to the
/// terminal.
///
/// T in this case would then be the item being logged.
pub trait Log {
    /// Create a new Logger struct which can be used to log.
    fn new(level: LogLevels) -> Self;
    /// Info logs a new message to the terminal,  it will not be logged if the loglevel is any
    /// higher then info.
    ///
    /// # Example
    /// ```Rust
    /// let log = Logger::new(LogLevels::Info);
    /// log.info("This is a info log"); // This will be printed to terminal
    ///
    /// let log = Logger::new(LogLevels::Warning);
    /// log.info("This is a info log"); // This will NOT be printed to terminal
    /// ```
    fn info<T: Debug>(&self, input: &T);
    /// warning logs a new warning message to the terminal, if the LogLevel is anyhigher then
    /// warning it wont be logged.
    ///
    /// # Example
    /// ```Rust
    /// let log = Logger::new(LogLevels::Info);
    /// log.warning("This is a info log"); // This will be printed to terminal
    ///
    /// let log = Logger::new(LogLevels::Warning);
    /// log.warning("This is a info log"); // This will be printed to terminal
    ///
    /// let log = Logger::new(LogLevels::Error);
    /// log.info("This is a info log"); // This will NOT be printed to terminal
    /// ```
    fn warning<T: Debug>(&self, input: &T);
    /// error logs a new error message to the terminal.
    /// Error logs are toplevel meaning they can't and wont be ignored.
    ///
    /// # Example
    /// ```Rust
    /// let log = Logger::new(LogLevels::Info);
    /// log.error("This is a info log"); // This will be printed to terminal
    ///
    /// let log = Logger::new(LogLevels::Warning);
    /// log.error("This is a info log"); // This will be printed to terminal
    ///
    /// let log = Logger::new(LogLevels::Error);
    /// log.error("This is a info log"); // This will be printed to terminal
    /// ```
    fn error<T: Debug>(&self, input: &T);
}

impl Log for Logger {
    fn new(level: LogLevels) -> Self {
        Self(level)
    }
    fn info<T: Debug>(&self, input: &T) {
        match self.0 {
            LogLevels::Info => {
                println!("{} {:#?}", "[INFO]".blue().bold(), input)
            }
            _ => (),
        }
    }
    fn warning<T: Debug>(&self, input: &T) {
        match &self.0 {
            LogLevels::Info | LogLevels::Warning => {
                println!("{} {:#?}",  "[WARNING]".yellow().bold(), input)
            }
            _ => (),
        }
    }
    fn error<T: Debug>(&self, input: &T) {
        match &self.0 {
            _ => println!("{} {:#?}", "[Error]".red().bold(), input),
        }
    }
}