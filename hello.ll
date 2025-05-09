; ModuleID = 'hello'
source_filename = "hello"

@str_0 = private constant [3 x i8] c"%s\00"
@str_1 = private constant [3 x i8] c"\\n\00"

declare i32 @printf(ptr, ...)

declare i32 @sprintf(ptr, ptr, ...)

declare i64 @atoll(ptr)

declare ptr @malloc(i64)

declare i64 @strlen(ptr)

define i32 @main() {
entry:
  %greeting = alloca ptr, align 8
  %malloc_call = call ptr @malloc(i64 16)
  %char_ptr_0 = getelementptr i8, ptr %malloc_call, i32 0
  store i8 72, ptr %char_ptr_0, align 1
  %char_ptr_1 = getelementptr i8, ptr %malloc_call, i32 1
  store i8 101, ptr %char_ptr_1, align 1
  %char_ptr_2 = getelementptr i8, ptr %malloc_call, i32 2
  store i8 108, ptr %char_ptr_2, align 1
  %char_ptr_3 = getelementptr i8, ptr %malloc_call, i32 3
  store i8 108, ptr %char_ptr_3, align 1
  %char_ptr_4 = getelementptr i8, ptr %malloc_call, i32 4
  store i8 111, ptr %char_ptr_4, align 1
  %char_ptr_5 = getelementptr i8, ptr %malloc_call, i32 5
  store i8 32, ptr %char_ptr_5, align 1
  %char_ptr_6 = getelementptr i8, ptr %malloc_call, i32 6
  store i8 102, ptr %char_ptr_6, align 1
  %char_ptr_7 = getelementptr i8, ptr %malloc_call, i32 7
  store i8 114, ptr %char_ptr_7, align 1
  %char_ptr_8 = getelementptr i8, ptr %malloc_call, i32 8
  store i8 111, ptr %char_ptr_8, align 1
  %char_ptr_9 = getelementptr i8, ptr %malloc_call, i32 9
  store i8 109, ptr %char_ptr_9, align 1
  %char_ptr_10 = getelementptr i8, ptr %malloc_call, i32 10
  store i8 32, ptr %char_ptr_10, align 1
  %char_ptr_11 = getelementptr i8, ptr %malloc_call, i32 11
  store i8 108, ptr %char_ptr_11, align 1
  %char_ptr_12 = getelementptr i8, ptr %malloc_call, i32 12
  store i8 117, ptr %char_ptr_12, align 1
  %char_ptr_13 = getelementptr i8, ptr %malloc_call, i32 13
  store i8 116, ptr %char_ptr_13, align 1
  %char_ptr_14 = getelementptr i8, ptr %malloc_call, i32 14
  store i8 33, ptr %char_ptr_14, align 1
  %null_ptr = getelementptr i8, ptr %malloc_call, i32 15
  store i8 0, ptr %null_ptr, align 1
  store ptr %malloc_call, ptr %greeting, align 8
  %greeting_ptr = load ptr, ptr %greeting, align 8
  %printf_call_0 = call i32 (ptr, ...) @printf(ptr @str_0, ptr %greeting_ptr)
  %printf_call_2 = call i32 (ptr, ...) @printf(ptr @str_1)
  ret i32 0
}
