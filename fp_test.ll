; ModuleID = 'fp_test'
source_filename = "fp_test"

@str_0 = private constant [3 x i8] c"%s\00"
@str_1 = private constant [5 x i8] c"%lld\00"
@str_2 = private constant [3 x i8] c"%s\00"
@str_3 = private constant [5 x i8] c"%lld\00"
@str_4 = private constant [3 x i8] c"\\n\00"
@str_5 = private constant [3 x i8] c"%s\00"
@str_6 = private constant [3 x i8] c"%s\00"
@str_7 = private constant [3 x i8] c"\\n\00"
@str_8 = private constant [3 x i8] c"%s\00"
@str_9 = private constant [3 x i8] c"%s\00"
@str_10 = private constant [3 x i8] c"\\n\00"

declare i32 @printf(ptr, ...)

declare i32 @sprintf(ptr, ptr, ...)

declare i64 @atoll(ptr)

declare ptr @malloc(i64)

declare i64 @strlen(ptr)

define i32 @main() {
entry:
  %bin_value = alloca ptr, align 8
  %hex_value = alloca ptr, align 8
  %area = alloca i64, align 8
  %radius = alloca i64, align 8
  %pi = alloca i64, align 8
  store i64 3, ptr %pi, align 4
  store i64 2, ptr %radius, align 4
  %pi_int = load i64, ptr %pi, align 4
  %radius_int = load i64, ptr %radius, align 4
  %mul = mul i64 %pi_int, %radius_int
  %radius_int1 = load i64, ptr %radius, align 4
  %mul2 = mul i64 %mul, %radius_int1
  store i64 %mul2, ptr %area, align 4
  %malloc_call = call ptr @malloc(i64 34)
  %char_ptr_0 = getelementptr i8, ptr %malloc_call, i32 0
  store i8 84, ptr %char_ptr_0, align 1
  %char_ptr_1 = getelementptr i8, ptr %malloc_call, i32 1
  store i8 104, ptr %char_ptr_1, align 1
  %char_ptr_2 = getelementptr i8, ptr %malloc_call, i32 2
  store i8 101, ptr %char_ptr_2, align 1
  %char_ptr_3 = getelementptr i8, ptr %malloc_call, i32 3
  store i8 32, ptr %char_ptr_3, align 1
  %char_ptr_4 = getelementptr i8, ptr %malloc_call, i32 4
  store i8 97, ptr %char_ptr_4, align 1
  %char_ptr_5 = getelementptr i8, ptr %malloc_call, i32 5
  store i8 114, ptr %char_ptr_5, align 1
  %char_ptr_6 = getelementptr i8, ptr %malloc_call, i32 6
  store i8 101, ptr %char_ptr_6, align 1
  %char_ptr_7 = getelementptr i8, ptr %malloc_call, i32 7
  store i8 97, ptr %char_ptr_7, align 1
  %char_ptr_8 = getelementptr i8, ptr %malloc_call, i32 8
  store i8 32, ptr %char_ptr_8, align 1
  %char_ptr_9 = getelementptr i8, ptr %malloc_call, i32 9
  store i8 111, ptr %char_ptr_9, align 1
  %char_ptr_10 = getelementptr i8, ptr %malloc_call, i32 10
  store i8 102, ptr %char_ptr_10, align 1
  %char_ptr_11 = getelementptr i8, ptr %malloc_call, i32 11
  store i8 32, ptr %char_ptr_11, align 1
  %char_ptr_12 = getelementptr i8, ptr %malloc_call, i32 12
  store i8 97, ptr %char_ptr_12, align 1
  %char_ptr_13 = getelementptr i8, ptr %malloc_call, i32 13
  store i8 32, ptr %char_ptr_13, align 1
  %char_ptr_14 = getelementptr i8, ptr %malloc_call, i32 14
  store i8 99, ptr %char_ptr_14, align 1
  %char_ptr_15 = getelementptr i8, ptr %malloc_call, i32 15
  store i8 105, ptr %char_ptr_15, align 1
  %char_ptr_16 = getelementptr i8, ptr %malloc_call, i32 16
  store i8 114, ptr %char_ptr_16, align 1
  %char_ptr_17 = getelementptr i8, ptr %malloc_call, i32 17
  store i8 99, ptr %char_ptr_17, align 1
  %char_ptr_18 = getelementptr i8, ptr %malloc_call, i32 18
  store i8 108, ptr %char_ptr_18, align 1
  %char_ptr_19 = getelementptr i8, ptr %malloc_call, i32 19
  store i8 101, ptr %char_ptr_19, align 1
  %char_ptr_20 = getelementptr i8, ptr %malloc_call, i32 20
  store i8 32, ptr %char_ptr_20, align 1
  %char_ptr_21 = getelementptr i8, ptr %malloc_call, i32 21
  store i8 119, ptr %char_ptr_21, align 1
  %char_ptr_22 = getelementptr i8, ptr %malloc_call, i32 22
  store i8 105, ptr %char_ptr_22, align 1
  %char_ptr_23 = getelementptr i8, ptr %malloc_call, i32 23
  store i8 116, ptr %char_ptr_23, align 1
  %char_ptr_24 = getelementptr i8, ptr %malloc_call, i32 24
  store i8 104, ptr %char_ptr_24, align 1
  %char_ptr_25 = getelementptr i8, ptr %malloc_call, i32 25
  store i8 32, ptr %char_ptr_25, align 1
  %char_ptr_26 = getelementptr i8, ptr %malloc_call, i32 26
  store i8 114, ptr %char_ptr_26, align 1
  %char_ptr_27 = getelementptr i8, ptr %malloc_call, i32 27
  store i8 97, ptr %char_ptr_27, align 1
  %char_ptr_28 = getelementptr i8, ptr %malloc_call, i32 28
  store i8 100, ptr %char_ptr_28, align 1
  %char_ptr_29 = getelementptr i8, ptr %malloc_call, i32 29
  store i8 105, ptr %char_ptr_29, align 1
  %char_ptr_30 = getelementptr i8, ptr %malloc_call, i32 30
  store i8 117, ptr %char_ptr_30, align 1
  %char_ptr_31 = getelementptr i8, ptr %malloc_call, i32 31
  store i8 115, ptr %char_ptr_31, align 1
  %char_ptr_32 = getelementptr i8, ptr %malloc_call, i32 32
  store i8 32, ptr %char_ptr_32, align 1
  %null_ptr = getelementptr i8, ptr %malloc_call, i32 33
  store i8 0, ptr %null_ptr, align 1
  %printf_call_0 = call i32 (ptr, ...) @printf(ptr @str_0, ptr %malloc_call)
  %radius_int3 = load i64, ptr %radius, align 4
  %printf_call_1 = call i32 (ptr, ...) @printf(ptr @str_1, i64 %radius_int3)
  %malloc_call4 = call ptr @malloc(i64 5)
  %char_ptr_05 = getelementptr i8, ptr %malloc_call4, i32 0
  store i8 32, ptr %char_ptr_05, align 1
  %char_ptr_110 = getelementptr i8, ptr %malloc_call4, i32 1
  store i8 105, ptr %char_ptr_110, align 1
  %char_ptr_211 = getelementptr i8, ptr %malloc_call4, i32 2
  store i8 115, ptr %char_ptr_211, align 1
  %char_ptr_312 = getelementptr i8, ptr %malloc_call4, i32 3
  store i8 32, ptr %char_ptr_312, align 1
  %null_ptr13 = getelementptr i8, ptr %malloc_call4, i32 4
  store i8 0, ptr %null_ptr13, align 1
  %printf_call_2 = call i32 (ptr, ...) @printf(ptr @str_2, ptr %malloc_call4)
  %area_int = load i64, ptr %area, align 4
  %printf_call_3 = call i32 (ptr, ...) @printf(ptr @str_3, i64 %area_int)
  %printf_call_5 = call i32 (ptr, ...) @printf(ptr @str_4)
  %malloc_call14 = call ptr @malloc(i64 5)
  %char_ptr_015 = getelementptr i8, ptr %malloc_call14, i32 0
  store i8 48, ptr %char_ptr_015, align 1
  %char_ptr_116 = getelementptr i8, ptr %malloc_call14, i32 1
  store i8 120, ptr %char_ptr_116, align 1
  %char_ptr_217 = getelementptr i8, ptr %malloc_call14, i32 2
  store i8 70, ptr %char_ptr_217, align 1
  %char_ptr_318 = getelementptr i8, ptr %malloc_call14, i32 3
  store i8 70, ptr %char_ptr_318, align 1
  %null_ptr19 = getelementptr i8, ptr %malloc_call14, i32 4
  store i8 0, ptr %null_ptr19, align 1
  store ptr %malloc_call14, ptr %hex_value, align 8
  %malloc_call20 = call ptr @malloc(i64 7)
  %char_ptr_021 = getelementptr i8, ptr %malloc_call20, i32 0
  store i8 48, ptr %char_ptr_021, align 1
  %char_ptr_122 = getelementptr i8, ptr %malloc_call20, i32 1
  store i8 98, ptr %char_ptr_122, align 1
  %char_ptr_223 = getelementptr i8, ptr %malloc_call20, i32 2
  store i8 49, ptr %char_ptr_223, align 1
  %char_ptr_324 = getelementptr i8, ptr %malloc_call20, i32 3
  store i8 48, ptr %char_ptr_324, align 1
  %char_ptr_425 = getelementptr i8, ptr %malloc_call20, i32 4
  store i8 49, ptr %char_ptr_425, align 1
  %char_ptr_526 = getelementptr i8, ptr %malloc_call20, i32 5
  store i8 48, ptr %char_ptr_526, align 1
  %null_ptr27 = getelementptr i8, ptr %malloc_call20, i32 6
  store i8 0, ptr %null_ptr27, align 1
  store ptr %malloc_call20, ptr %bin_value, align 8
  %malloc_call28 = call ptr @malloc(i64 12)
  %char_ptr_029 = getelementptr i8, ptr %malloc_call28, i32 0
  store i8 72, ptr %char_ptr_029, align 1
  %char_ptr_130 = getelementptr i8, ptr %malloc_call28, i32 1
  store i8 101, ptr %char_ptr_130, align 1
  %char_ptr_231 = getelementptr i8, ptr %malloc_call28, i32 2
  store i8 120, ptr %char_ptr_231, align 1
  %char_ptr_332 = getelementptr i8, ptr %malloc_call28, i32 3
  store i8 32, ptr %char_ptr_332, align 1
  %char_ptr_433 = getelementptr i8, ptr %malloc_call28, i32 4
  store i8 118, ptr %char_ptr_433, align 1
  %char_ptr_534 = getelementptr i8, ptr %malloc_call28, i32 5
  store i8 97, ptr %char_ptr_534, align 1
  %char_ptr_635 = getelementptr i8, ptr %malloc_call28, i32 6
  store i8 108, ptr %char_ptr_635, align 1
  %char_ptr_736 = getelementptr i8, ptr %malloc_call28, i32 7
  store i8 117, ptr %char_ptr_736, align 1
  %char_ptr_837 = getelementptr i8, ptr %malloc_call28, i32 8
  store i8 101, ptr %char_ptr_837, align 1
  %char_ptr_938 = getelementptr i8, ptr %malloc_call28, i32 9
  store i8 58, ptr %char_ptr_938, align 1
  %char_ptr_1039 = getelementptr i8, ptr %malloc_call28, i32 10
  store i8 32, ptr %char_ptr_1039, align 1
  %null_ptr40 = getelementptr i8, ptr %malloc_call28, i32 11
  store i8 0, ptr %null_ptr40, align 1
  %printf_call_541 = call i32 (ptr, ...) @printf(ptr @str_5, ptr %malloc_call28)
  %hex_value_ptr = load ptr, ptr %hex_value, align 8
  %printf_call_6 = call i32 (ptr, ...) @printf(ptr @str_6, ptr %hex_value_ptr)
  %printf_call_8 = call i32 (ptr, ...) @printf(ptr @str_7)
  %malloc_call42 = call ptr @malloc(i64 15)
  %char_ptr_043 = getelementptr i8, ptr %malloc_call42, i32 0
  store i8 66, ptr %char_ptr_043, align 1
  %char_ptr_144 = getelementptr i8, ptr %malloc_call42, i32 1
  store i8 105, ptr %char_ptr_144, align 1
  %char_ptr_245 = getelementptr i8, ptr %malloc_call42, i32 2
  store i8 110, ptr %char_ptr_245, align 1
  %char_ptr_346 = getelementptr i8, ptr %malloc_call42, i32 3
  store i8 97, ptr %char_ptr_346, align 1
  %char_ptr_447 = getelementptr i8, ptr %malloc_call42, i32 4
  store i8 114, ptr %char_ptr_447, align 1
  %char_ptr_548 = getelementptr i8, ptr %malloc_call42, i32 5
  store i8 121, ptr %char_ptr_548, align 1
  %char_ptr_649 = getelementptr i8, ptr %malloc_call42, i32 6
  store i8 32, ptr %char_ptr_649, align 1
  %char_ptr_750 = getelementptr i8, ptr %malloc_call42, i32 7
  store i8 118, ptr %char_ptr_750, align 1
  %char_ptr_851 = getelementptr i8, ptr %malloc_call42, i32 8
  store i8 97, ptr %char_ptr_851, align 1
  %char_ptr_952 = getelementptr i8, ptr %malloc_call42, i32 9
  store i8 108, ptr %char_ptr_952, align 1
  %char_ptr_1053 = getelementptr i8, ptr %malloc_call42, i32 10
  store i8 117, ptr %char_ptr_1053, align 1
  %char_ptr_1154 = getelementptr i8, ptr %malloc_call42, i32 11
  store i8 101, ptr %char_ptr_1154, align 1
  %char_ptr_1255 = getelementptr i8, ptr %malloc_call42, i32 12
  store i8 58, ptr %char_ptr_1255, align 1
  %char_ptr_1356 = getelementptr i8, ptr %malloc_call42, i32 13
  store i8 32, ptr %char_ptr_1356, align 1
  %null_ptr57 = getelementptr i8, ptr %malloc_call42, i32 14
  store i8 0, ptr %null_ptr57, align 1
  %printf_call_858 = call i32 (ptr, ...) @printf(ptr @str_8, ptr %malloc_call42)
  %bin_value_ptr = load ptr, ptr %bin_value, align 8
  %printf_call_9 = call i32 (ptr, ...) @printf(ptr @str_9, ptr %bin_value_ptr)
  %printf_call_11 = call i32 (ptr, ...) @printf(ptr @str_10)
  ret i32 0
}
