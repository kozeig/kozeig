; ModuleID = 'arrays'
source_filename = "arrays"

@str_0 = private constant [3 x i8] c"%s\00"
@str_1 = private constant [3 x i8] c"\\n\00"
@str_2 = private constant [3 x i8] c"%s\00"
@str_3 = private constant [3 x i8] c"\\n\00"
@str_4 = private constant [3 x i8] c"%s\00"
@str_5 = private constant [3 x i8] c"\\n\00"

declare i32 @printf(ptr, ...)

declare i32 @sprintf(ptr, ptr, ...)

declare i64 @atoll(ptr)

declare ptr @malloc(i64)

declare i64 @strlen(ptr)

define i32 @main() {
entry:
  %mixed = alloca ptr, align 8
  %matrix = alloca ptr, align 8
  %numbers = alloca ptr, align 8
  %malloc_call = call ptr @malloc(i64 16)
  %char_ptr_0 = getelementptr i8, ptr %malloc_call, i32 0
  store i8 91, ptr %char_ptr_0, align 1
  %char_ptr_1 = getelementptr i8, ptr %malloc_call, i32 1
  store i8 63, ptr %char_ptr_1, align 1
  %char_ptr_2 = getelementptr i8, ptr %malloc_call, i32 2
  store i8 44, ptr %char_ptr_2, align 1
  %char_ptr_3 = getelementptr i8, ptr %malloc_call, i32 3
  store i8 32, ptr %char_ptr_3, align 1
  %char_ptr_4 = getelementptr i8, ptr %malloc_call, i32 4
  store i8 63, ptr %char_ptr_4, align 1
  %char_ptr_5 = getelementptr i8, ptr %malloc_call, i32 5
  store i8 44, ptr %char_ptr_5, align 1
  %char_ptr_6 = getelementptr i8, ptr %malloc_call, i32 6
  store i8 32, ptr %char_ptr_6, align 1
  %char_ptr_7 = getelementptr i8, ptr %malloc_call, i32 7
  store i8 63, ptr %char_ptr_7, align 1
  %char_ptr_8 = getelementptr i8, ptr %malloc_call, i32 8
  store i8 44, ptr %char_ptr_8, align 1
  %char_ptr_9 = getelementptr i8, ptr %malloc_call, i32 9
  store i8 32, ptr %char_ptr_9, align 1
  %char_ptr_10 = getelementptr i8, ptr %malloc_call, i32 10
  store i8 63, ptr %char_ptr_10, align 1
  %char_ptr_11 = getelementptr i8, ptr %malloc_call, i32 11
  store i8 44, ptr %char_ptr_11, align 1
  %char_ptr_12 = getelementptr i8, ptr %malloc_call, i32 12
  store i8 32, ptr %char_ptr_12, align 1
  %char_ptr_13 = getelementptr i8, ptr %malloc_call, i32 13
  store i8 63, ptr %char_ptr_13, align 1
  %char_ptr_14 = getelementptr i8, ptr %malloc_call, i32 14
  store i8 93, ptr %char_ptr_14, align 1
  %null_ptr = getelementptr i8, ptr %malloc_call, i32 15
  store i8 0, ptr %null_ptr, align 1
  store ptr %malloc_call, ptr %numbers, align 8
  %numbers_ptr = load ptr, ptr %numbers, align 8
  %printf_call_0 = call i32 (ptr, ...) @printf(ptr @str_0, ptr %numbers_ptr)
  %printf_call_2 = call i32 (ptr, ...) @printf(ptr @str_1)
  %malloc_call1 = call ptr @malloc(i64 34)
  %char_ptr_02 = getelementptr i8, ptr %malloc_call1, i32 0
  store i8 91, ptr %char_ptr_02, align 1
  %char_ptr_15 = getelementptr i8, ptr %malloc_call1, i32 1
  store i8 91, ptr %char_ptr_15, align 1
  %char_ptr_26 = getelementptr i8, ptr %malloc_call1, i32 2
  store i8 63, ptr %char_ptr_26, align 1
  %char_ptr_37 = getelementptr i8, ptr %malloc_call1, i32 3
  store i8 44, ptr %char_ptr_37, align 1
  %char_ptr_48 = getelementptr i8, ptr %malloc_call1, i32 4
  store i8 32, ptr %char_ptr_48, align 1
  %char_ptr_59 = getelementptr i8, ptr %malloc_call1, i32 5
  store i8 63, ptr %char_ptr_59, align 1
  %char_ptr_610 = getelementptr i8, ptr %malloc_call1, i32 6
  store i8 44, ptr %char_ptr_610, align 1
  %char_ptr_711 = getelementptr i8, ptr %malloc_call1, i32 7
  store i8 32, ptr %char_ptr_711, align 1
  %char_ptr_812 = getelementptr i8, ptr %malloc_call1, i32 8
  store i8 63, ptr %char_ptr_812, align 1
  %char_ptr_913 = getelementptr i8, ptr %malloc_call1, i32 9
  store i8 93, ptr %char_ptr_913, align 1
  %char_ptr_1014 = getelementptr i8, ptr %malloc_call1, i32 10
  store i8 59, ptr %char_ptr_1014, align 1
  %char_ptr_1115 = getelementptr i8, ptr %malloc_call1, i32 11
  store i8 32, ptr %char_ptr_1115, align 1
  %char_ptr_1216 = getelementptr i8, ptr %malloc_call1, i32 12
  store i8 91, ptr %char_ptr_1216, align 1
  %char_ptr_1317 = getelementptr i8, ptr %malloc_call1, i32 13
  store i8 63, ptr %char_ptr_1317, align 1
  %char_ptr_1418 = getelementptr i8, ptr %malloc_call1, i32 14
  store i8 44, ptr %char_ptr_1418, align 1
  %char_ptr_1519 = getelementptr i8, ptr %malloc_call1, i32 15
  store i8 32, ptr %char_ptr_1519, align 1
  %char_ptr_16 = getelementptr i8, ptr %malloc_call1, i32 16
  store i8 63, ptr %char_ptr_16, align 1
  %char_ptr_17 = getelementptr i8, ptr %malloc_call1, i32 17
  store i8 44, ptr %char_ptr_17, align 1
  %char_ptr_18 = getelementptr i8, ptr %malloc_call1, i32 18
  store i8 32, ptr %char_ptr_18, align 1
  %char_ptr_19 = getelementptr i8, ptr %malloc_call1, i32 19
  store i8 63, ptr %char_ptr_19, align 1
  %char_ptr_20 = getelementptr i8, ptr %malloc_call1, i32 20
  store i8 93, ptr %char_ptr_20, align 1
  %char_ptr_21 = getelementptr i8, ptr %malloc_call1, i32 21
  store i8 59, ptr %char_ptr_21, align 1
  %char_ptr_22 = getelementptr i8, ptr %malloc_call1, i32 22
  store i8 32, ptr %char_ptr_22, align 1
  %char_ptr_23 = getelementptr i8, ptr %malloc_call1, i32 23
  store i8 91, ptr %char_ptr_23, align 1
  %char_ptr_24 = getelementptr i8, ptr %malloc_call1, i32 24
  store i8 63, ptr %char_ptr_24, align 1
  %char_ptr_25 = getelementptr i8, ptr %malloc_call1, i32 25
  store i8 44, ptr %char_ptr_25, align 1
  %char_ptr_2620 = getelementptr i8, ptr %malloc_call1, i32 26
  store i8 32, ptr %char_ptr_2620, align 1
  %char_ptr_27 = getelementptr i8, ptr %malloc_call1, i32 27
  store i8 63, ptr %char_ptr_27, align 1
  %char_ptr_28 = getelementptr i8, ptr %malloc_call1, i32 28
  store i8 44, ptr %char_ptr_28, align 1
  %char_ptr_29 = getelementptr i8, ptr %malloc_call1, i32 29
  store i8 32, ptr %char_ptr_29, align 1
  %char_ptr_30 = getelementptr i8, ptr %malloc_call1, i32 30
  store i8 63, ptr %char_ptr_30, align 1
  %char_ptr_31 = getelementptr i8, ptr %malloc_call1, i32 31
  store i8 93, ptr %char_ptr_31, align 1
  %char_ptr_32 = getelementptr i8, ptr %malloc_call1, i32 32
  store i8 93, ptr %char_ptr_32, align 1
  %null_ptr21 = getelementptr i8, ptr %malloc_call1, i32 33
  store i8 0, ptr %null_ptr21, align 1
  store ptr %malloc_call1, ptr %matrix, align 8
  %matrix_ptr = load ptr, ptr %matrix, align 8
  %printf_call_222 = call i32 (ptr, ...) @printf(ptr @str_2, ptr %matrix_ptr)
  %printf_call_4 = call i32 (ptr, ...) @printf(ptr @str_3)
  %malloc_call23 = call ptr @malloc(i64 13)
  %char_ptr_024 = getelementptr i8, ptr %malloc_call23, i32 0
  store i8 91, ptr %char_ptr_024, align 1
  %char_ptr_125 = getelementptr i8, ptr %malloc_call23, i32 1
  store i8 63, ptr %char_ptr_125, align 1
  %char_ptr_226 = getelementptr i8, ptr %malloc_call23, i32 2
  store i8 44, ptr %char_ptr_226, align 1
  %char_ptr_327 = getelementptr i8, ptr %malloc_call23, i32 3
  store i8 32, ptr %char_ptr_327, align 1
  %char_ptr_428 = getelementptr i8, ptr %malloc_call23, i32 4
  store i8 63, ptr %char_ptr_428, align 1
  %char_ptr_529 = getelementptr i8, ptr %malloc_call23, i32 5
  store i8 44, ptr %char_ptr_529, align 1
  %char_ptr_630 = getelementptr i8, ptr %malloc_call23, i32 6
  store i8 32, ptr %char_ptr_630, align 1
  %char_ptr_731 = getelementptr i8, ptr %malloc_call23, i32 7
  store i8 63, ptr %char_ptr_731, align 1
  %char_ptr_832 = getelementptr i8, ptr %malloc_call23, i32 8
  store i8 44, ptr %char_ptr_832, align 1
  %char_ptr_933 = getelementptr i8, ptr %malloc_call23, i32 9
  store i8 32, ptr %char_ptr_933, align 1
  %char_ptr_1034 = getelementptr i8, ptr %malloc_call23, i32 10
  store i8 63, ptr %char_ptr_1034, align 1
  %char_ptr_1135 = getelementptr i8, ptr %malloc_call23, i32 11
  store i8 93, ptr %char_ptr_1135, align 1
  %null_ptr36 = getelementptr i8, ptr %malloc_call23, i32 12
  store i8 0, ptr %null_ptr36, align 1
  store ptr %malloc_call23, ptr %mixed, align 8
  %mixed_ptr = load ptr, ptr %mixed, align 8
  %printf_call_437 = call i32 (ptr, ...) @printf(ptr @str_4, ptr %mixed_ptr)
  %printf_call_6 = call i32 (ptr, ...) @printf(ptr @str_5)
  ret i32 0
}
