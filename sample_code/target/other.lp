; ModuleID = 'start'
source_filename = "start"

@var = addrspace(1) global i32 20
@print_str0.029234888108423118 = private unnamed_addr constant [18 x i8] c"Goodbye world! \\n\00", align 1

define void @wow() {
entry:
  %printf_call = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([18 x i8], [18 x i8]* @print_str0.029234888108423118, i32 0, i32 0))
  ret void
}

declare i32 @printf(i8*, ...)
