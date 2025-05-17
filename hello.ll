; ModuleID = 'hello'
source_filename = "hello"

@str_0 = private constant [14 x i8] c"Hello, World!\00"
@str_1 = private constant [3 x i8] c"%s\00"
@str_2 = private constant [3 x i8] c"\\n\00"

declare i32 @printf(ptr, ...)

declare i32 @sprintf(ptr, ptr, ...)

declare i64 @atoll(ptr)

declare ptr @malloc(i64)

declare i64 @strlen(ptr)

define i64 @main() {
entry:
  %malloc_call = call ptr @malloc(i64 14)
  call void @llvm.memcpy.p0.p0.i64(ptr %malloc_call, ptr @str_0, i64 14, i1 false)
  %printf_call_1 = call i32 (ptr, ...) @printf(ptr @str_1, ptr %malloc_call)
  %printf_call_3 = call i32 (ptr, ...) @printf(ptr @str_2)
  ret i64 0
}

define i64 @main.1() {
entry:
  %dummy = alloca i64, align 8
  %user_main_call = call i64 @main()
  ret i64 %user_main_call
}

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #0

attributes #0 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
