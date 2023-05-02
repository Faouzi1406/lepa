## Todo's 

# When 25-04-2023
- [x] Add tests for parser function

- [x] Add logger so that parser can log stuff like warnings, errors etc.

# When 27-04-2023

Everything here if ofcourse going to get improved over times.

- [x] Support for parsing variables

- [x] Parsing blocks 

- [x] Tests for both parsing blocks and variables

- [x] Sample code for testing

- [x] Added parsing for functions

# When 28-04-2023

Today was just me working on setting up llvm and making a start with compiling to llvm ir, the llvm ir will then get compiled to a binary. Want to add logic tommorow like if else etc., Also currently the compiler is just mainly for testing  it already compiles a function without any logic etc, it doesn't compile function bodies yet, it does have the correct return types etc, it also compiles global variables, however I'm going change the global variables to be const / static during parsing. That way we prevent the user from trying to change global variables. After the compiler is semi stable I'm probably going to change most of how it works to support more complex stuff. Maybe even add support for structs and impl's :)

- [x] Added support for parsing function return types
- [x] Setup llvm to start working on compiling to llvm ir.
- [x] Started working on the compiler, currently compiles a simple function just for testing, doesn't really work yet it's currently just for testing;

# When 29-04-2023

- [x] Changed the way parsing arguments works
- [x] Tests for parsing args
- [x] Compiler support return
- [x] Compiler creating local variables

# When 30-04-2023

- [x] Support for calling functions from variables 

- [x] Printf support; This is bassicly the beginning point of adding the std library for the langauge.

- [x] Const support; Added support for creating global constant variables, this means they are assured to never be reassigned or changed in anyway, or atleast it will later. 

- [x] Support for use; Using different files
