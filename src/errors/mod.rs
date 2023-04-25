/// The main errors mod
///
///  It contains the ErrorBuilder. The error builder wil be used to build errors that are user
///  friendly.
///
pub mod error;
///  The logger, this can be used to log errors/warning/info without directly stopping compilation.
///  This could be usefull if the warning doesn't need to stop compilation etc.
pub mod logger;
