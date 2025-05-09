; ModuleID = 'basic_math'
source_filename = "basic_math"

@str_0 = private constant [33 x i8] c"Runtime error: Division by zero\0A\00"
@str_1 = private constant [31 x i8] c"Runtime error: Modulo by zero\0A\00"
@str_2 = private constant [3 x i8] c"%s\00"
@str_3 = private constant [5 x i8] c"%lld\00"
@str_4 = private constant [3 x i8] c"\\n\00"
@str_5 = private constant [3 x i8] c"%s\00"
@str_6 = private constant [5 x i8] c"%lld\00"
@str_7 = private constant [3 x i8] c"\\n\00"
@str_8 = private constant [3 x i8] c"%s\00"
@str_9 = private constant [5 x i8] c"%lld\00"
@str_10 = private constant [3 x i8] c"\\n\00"
@str_11 = private constant [3 x i8] c"%s\00"
@str_12 = private constant [5 x i8] c"%lld\00"
@str_13 = private constant [3 x i8] c"\\n\00"
@str_14 = private constant [3 x i8] c"%s\00"
@str_15 = private constant [5 x i8] c"%lld\00"
@str_16 = private constant [3 x i8] c"\\n\00"
@str_17 = private constant [3 x i8] c"%s\00"
@str_18 = private constant [5 x i8] c"%lld\00"
@str_19 = private constant [3 x i8] c"%s\00"
@str_20 = private constant [5 x i8] c"%lld\00"
@str_21 = private constant [3 x i8] c"%s\00"
@str_22 = private constant [5 x i8] c"%lld\00"
@str_23 = private constant [3 x i8] c"\\n\00"
@str_24 = private constant [3 x i8] c"%s\00"
@str_25 = private constant [5 x i8] c"%lld\00"
@str_26 = private constant [3 x i8] c"%s\00"
@str_27 = private constant [5 x i8] c"%lld\00"
@str_28 = private constant [3 x i8] c"%s\00"
@str_29 = private constant [5 x i8] c"%lld\00"
@str_30 = private constant [3 x i8] c"\\n\00"
@str_31 = private constant [5 x i8] c"%lld\00"
@str_32 = private constant [5 x i8] c"%lld\00"
@str_33 = private constant [3 x i8] c"%s\00"
@str_34 = private constant [3 x i8] c"%s\00"
@str_35 = private constant [3 x i8] c"%s\00"
@str_36 = private constant [3 x i8] c"%s\00"
@str_37 = private constant [3 x i8] c"\\n\00"
@str_38 = private constant [3 x i8] c"%s\00"
@str_39 = private constant [5 x i8] c"%lld\00"
@str_40 = private constant [3 x i8] c"\\n\00"

declare i32 @printf(ptr, ...)

declare i32 @sprintf(ptr, ptr, ...)

declare i64 @atoll(ptr)

declare ptr @malloc(i64)

declare i64 @strlen(ptr)

define i32 @main() {
entry:
  %text_b = alloca ptr, align 8
  %text_a = alloca ptr, align 8
  %compound2 = alloca i64, align 8
  %compound1 = alloca i64, align 8
  %remainder = alloca i64, align 8
  %quotient = alloca i64, align 8
  %product = alloca i64, align 8
  %difference = alloca i64, align 8
  %sum = alloca i64, align 8
  %b = alloca i64, align 8
  %a = alloca i64, align 8
  store i64 10, ptr %a, align 4
  store i64 5, ptr %b, align 4
  %a_int = load i64, ptr %a, align 4
  %b_int = load i64, ptr %b, align 4
  %add = add i64 %a_int, %b_int
  store i64 %add, ptr %sum, align 4
  %a_int1 = load i64, ptr %a, align 4
  %b_int2 = load i64, ptr %b, align 4
  %sub = sub i64 %a_int1, %b_int2
  store i64 %sub, ptr %difference, align 4
  %a_int3 = load i64, ptr %a, align 4
  %b_int4 = load i64, ptr %b, align 4
  %mul = mul i64 %a_int3, %b_int4
  store i64 %mul, ptr %product, align 4
  %a_int5 = load i64, ptr %a, align 4
  %b_int6 = load i64, ptr %b, align 4
  %is_zero = icmp eq i64 %b_int6, 0
  br i1 %is_zero, label %div_by_zero, label %div

div:                                              ; preds = %entry
  %div7 = sdiv i64 %a_int5, %b_int6
  br label %div_cont

div_by_zero:                                      ; preds = %entry
  %div_zero_error_msg = call i32 (ptr, ...) @printf(ptr @str_0)
  ret i32 1

div_cont:                                         ; preds = %div
  %div_result = phi i64 [ %div7, %div ]
  store i64 %div_result, ptr %quotient, align 4
  %a_int8 = load i64, ptr %a, align 4
  %b_int9 = load i64, ptr %b, align 4
  %is_zero10 = icmp eq i64 %b_int9, 0
  br i1 %is_zero10, label %mod_by_zero, label %mod

mod:                                              ; preds = %div_cont
  %mod11 = srem i64 %a_int8, %b_int9
  br label %mod_cont

mod_by_zero:                                      ; preds = %div_cont
  %mod_zero_error_msg = call i32 (ptr, ...) @printf(ptr @str_1)
  ret i32 1

mod_cont:                                         ; preds = %mod
  %mod_result = phi i64 [ %mod11, %mod ]
  store i64 %mod_result, ptr %remainder, align 4
  %malloc_call = call ptr @malloc(i64 6)
  %char_ptr_0 = getelementptr i8, ptr %malloc_call, i32 0
  store i8 83, ptr %char_ptr_0, align 1
  %char_ptr_1 = getelementptr i8, ptr %malloc_call, i32 1
  store i8 117, ptr %char_ptr_1, align 1
  %char_ptr_2 = getelementptr i8, ptr %malloc_call, i32 2
  store i8 109, ptr %char_ptr_2, align 1
  %char_ptr_3 = getelementptr i8, ptr %malloc_call, i32 3
  store i8 58, ptr %char_ptr_3, align 1
  %char_ptr_4 = getelementptr i8, ptr %malloc_call, i32 4
  store i8 32, ptr %char_ptr_4, align 1
  %null_ptr = getelementptr i8, ptr %malloc_call, i32 5
  store i8 0, ptr %null_ptr, align 1
  %printf_call_2 = call i32 (ptr, ...) @printf(ptr @str_2, ptr %malloc_call)
  %sum_int = load i64, ptr %sum, align 4
  %printf_call_3 = call i32 (ptr, ...) @printf(ptr @str_3, i64 %sum_int)
  %printf_call_5 = call i32 (ptr, ...) @printf(ptr @str_4)
  %malloc_call12 = call ptr @malloc(i64 13)
  %char_ptr_013 = getelementptr i8, ptr %malloc_call12, i32 0
  store i8 68, ptr %char_ptr_013, align 1
  %char_ptr_114 = getelementptr i8, ptr %malloc_call12, i32 1
  store i8 105, ptr %char_ptr_114, align 1
  %char_ptr_215 = getelementptr i8, ptr %malloc_call12, i32 2
  store i8 102, ptr %char_ptr_215, align 1
  %char_ptr_316 = getelementptr i8, ptr %malloc_call12, i32 3
  store i8 102, ptr %char_ptr_316, align 1
  %char_ptr_417 = getelementptr i8, ptr %malloc_call12, i32 4
  store i8 101, ptr %char_ptr_417, align 1
  %char_ptr_5 = getelementptr i8, ptr %malloc_call12, i32 5
  store i8 114, ptr %char_ptr_5, align 1
  %char_ptr_6 = getelementptr i8, ptr %malloc_call12, i32 6
  store i8 101, ptr %char_ptr_6, align 1
  %char_ptr_7 = getelementptr i8, ptr %malloc_call12, i32 7
  store i8 110, ptr %char_ptr_7, align 1
  %char_ptr_8 = getelementptr i8, ptr %malloc_call12, i32 8
  store i8 99, ptr %char_ptr_8, align 1
  %char_ptr_9 = getelementptr i8, ptr %malloc_call12, i32 9
  store i8 101, ptr %char_ptr_9, align 1
  %char_ptr_10 = getelementptr i8, ptr %malloc_call12, i32 10
  store i8 58, ptr %char_ptr_10, align 1
  %char_ptr_11 = getelementptr i8, ptr %malloc_call12, i32 11
  store i8 32, ptr %char_ptr_11, align 1
  %null_ptr18 = getelementptr i8, ptr %malloc_call12, i32 12
  store i8 0, ptr %null_ptr18, align 1
  %printf_call_519 = call i32 (ptr, ...) @printf(ptr @str_5, ptr %malloc_call12)
  %difference_int = load i64, ptr %difference, align 4
  %printf_call_6 = call i32 (ptr, ...) @printf(ptr @str_6, i64 %difference_int)
  %printf_call_8 = call i32 (ptr, ...) @printf(ptr @str_7)
  %malloc_call20 = call ptr @malloc(i64 10)
  %char_ptr_021 = getelementptr i8, ptr %malloc_call20, i32 0
  store i8 80, ptr %char_ptr_021, align 1
  %char_ptr_122 = getelementptr i8, ptr %malloc_call20, i32 1
  store i8 114, ptr %char_ptr_122, align 1
  %char_ptr_223 = getelementptr i8, ptr %malloc_call20, i32 2
  store i8 111, ptr %char_ptr_223, align 1
  %char_ptr_324 = getelementptr i8, ptr %malloc_call20, i32 3
  store i8 100, ptr %char_ptr_324, align 1
  %char_ptr_425 = getelementptr i8, ptr %malloc_call20, i32 4
  store i8 117, ptr %char_ptr_425, align 1
  %char_ptr_526 = getelementptr i8, ptr %malloc_call20, i32 5
  store i8 99, ptr %char_ptr_526, align 1
  %char_ptr_627 = getelementptr i8, ptr %malloc_call20, i32 6
  store i8 116, ptr %char_ptr_627, align 1
  %char_ptr_728 = getelementptr i8, ptr %malloc_call20, i32 7
  store i8 58, ptr %char_ptr_728, align 1
  %char_ptr_829 = getelementptr i8, ptr %malloc_call20, i32 8
  store i8 32, ptr %char_ptr_829, align 1
  %null_ptr30 = getelementptr i8, ptr %malloc_call20, i32 9
  store i8 0, ptr %null_ptr30, align 1
  %printf_call_831 = call i32 (ptr, ...) @printf(ptr @str_8, ptr %malloc_call20)
  %product_int = load i64, ptr %product, align 4
  %printf_call_9 = call i32 (ptr, ...) @printf(ptr @str_9, i64 %product_int)
  %printf_call_11 = call i32 (ptr, ...) @printf(ptr @str_10)
  %malloc_call32 = call ptr @malloc(i64 11)
  %char_ptr_033 = getelementptr i8, ptr %malloc_call32, i32 0
  store i8 81, ptr %char_ptr_033, align 1
  %char_ptr_134 = getelementptr i8, ptr %malloc_call32, i32 1
  store i8 117, ptr %char_ptr_134, align 1
  %char_ptr_235 = getelementptr i8, ptr %malloc_call32, i32 2
  store i8 111, ptr %char_ptr_235, align 1
  %char_ptr_336 = getelementptr i8, ptr %malloc_call32, i32 3
  store i8 116, ptr %char_ptr_336, align 1
  %char_ptr_437 = getelementptr i8, ptr %malloc_call32, i32 4
  store i8 105, ptr %char_ptr_437, align 1
  %char_ptr_538 = getelementptr i8, ptr %malloc_call32, i32 5
  store i8 101, ptr %char_ptr_538, align 1
  %char_ptr_639 = getelementptr i8, ptr %malloc_call32, i32 6
  store i8 110, ptr %char_ptr_639, align 1
  %char_ptr_740 = getelementptr i8, ptr %malloc_call32, i32 7
  store i8 116, ptr %char_ptr_740, align 1
  %char_ptr_841 = getelementptr i8, ptr %malloc_call32, i32 8
  store i8 58, ptr %char_ptr_841, align 1
  %char_ptr_942 = getelementptr i8, ptr %malloc_call32, i32 9
  store i8 32, ptr %char_ptr_942, align 1
  %null_ptr43 = getelementptr i8, ptr %malloc_call32, i32 10
  store i8 0, ptr %null_ptr43, align 1
  %printf_call_1144 = call i32 (ptr, ...) @printf(ptr @str_11, ptr %malloc_call32)
  %quotient_int = load i64, ptr %quotient, align 4
  %printf_call_12 = call i32 (ptr, ...) @printf(ptr @str_12, i64 %quotient_int)
  %printf_call_14 = call i32 (ptr, ...) @printf(ptr @str_13)
  %malloc_call45 = call ptr @malloc(i64 12)
  %char_ptr_046 = getelementptr i8, ptr %malloc_call45, i32 0
  store i8 82, ptr %char_ptr_046, align 1
  %char_ptr_147 = getelementptr i8, ptr %malloc_call45, i32 1
  store i8 101, ptr %char_ptr_147, align 1
  %char_ptr_248 = getelementptr i8, ptr %malloc_call45, i32 2
  store i8 109, ptr %char_ptr_248, align 1
  %char_ptr_349 = getelementptr i8, ptr %malloc_call45, i32 3
  store i8 97, ptr %char_ptr_349, align 1
  %char_ptr_450 = getelementptr i8, ptr %malloc_call45, i32 4
  store i8 105, ptr %char_ptr_450, align 1
  %char_ptr_551 = getelementptr i8, ptr %malloc_call45, i32 5
  store i8 110, ptr %char_ptr_551, align 1
  %char_ptr_652 = getelementptr i8, ptr %malloc_call45, i32 6
  store i8 100, ptr %char_ptr_652, align 1
  %char_ptr_753 = getelementptr i8, ptr %malloc_call45, i32 7
  store i8 101, ptr %char_ptr_753, align 1
  %char_ptr_854 = getelementptr i8, ptr %malloc_call45, i32 8
  store i8 114, ptr %char_ptr_854, align 1
  %char_ptr_955 = getelementptr i8, ptr %malloc_call45, i32 9
  store i8 58, ptr %char_ptr_955, align 1
  %char_ptr_1056 = getelementptr i8, ptr %malloc_call45, i32 10
  store i8 32, ptr %char_ptr_1056, align 1
  %null_ptr57 = getelementptr i8, ptr %malloc_call45, i32 11
  store i8 0, ptr %null_ptr57, align 1
  %printf_call_1458 = call i32 (ptr, ...) @printf(ptr @str_14, ptr %malloc_call45)
  %remainder_int = load i64, ptr %remainder, align 4
  %printf_call_15 = call i32 (ptr, ...) @printf(ptr @str_15, i64 %remainder_int)
  %printf_call_17 = call i32 (ptr, ...) @printf(ptr @str_16)
  %a_int59 = load i64, ptr %a, align 4
  %b_int60 = load i64, ptr %b, align 4
  %mul61 = mul i64 %b_int60, 2
  %add62 = add i64 %a_int59, %mul61
  store i64 %add62, ptr %compound1, align 4
  %malloc_call63 = call ptr @malloc(i64 13)
  %char_ptr_064 = getelementptr i8, ptr %malloc_call63, i32 0
  store i8 67, ptr %char_ptr_064, align 1
  %char_ptr_165 = getelementptr i8, ptr %malloc_call63, i32 1
  store i8 111, ptr %char_ptr_165, align 1
  %char_ptr_266 = getelementptr i8, ptr %malloc_call63, i32 2
  store i8 109, ptr %char_ptr_266, align 1
  %char_ptr_367 = getelementptr i8, ptr %malloc_call63, i32 3
  store i8 112, ptr %char_ptr_367, align 1
  %char_ptr_468 = getelementptr i8, ptr %malloc_call63, i32 4
  store i8 111, ptr %char_ptr_468, align 1
  %char_ptr_569 = getelementptr i8, ptr %malloc_call63, i32 5
  store i8 117, ptr %char_ptr_569, align 1
  %char_ptr_670 = getelementptr i8, ptr %malloc_call63, i32 6
  store i8 110, ptr %char_ptr_670, align 1
  %char_ptr_771 = getelementptr i8, ptr %malloc_call63, i32 7
  store i8 100, ptr %char_ptr_771, align 1
  %char_ptr_872 = getelementptr i8, ptr %malloc_call63, i32 8
  store i8 32, ptr %char_ptr_872, align 1
  %char_ptr_973 = getelementptr i8, ptr %malloc_call63, i32 9
  store i8 49, ptr %char_ptr_973, align 1
  %char_ptr_1074 = getelementptr i8, ptr %malloc_call63, i32 10
  store i8 58, ptr %char_ptr_1074, align 1
  %char_ptr_1175 = getelementptr i8, ptr %malloc_call63, i32 11
  store i8 32, ptr %char_ptr_1175, align 1
  %null_ptr76 = getelementptr i8, ptr %malloc_call63, i32 12
  store i8 0, ptr %null_ptr76, align 1
  %printf_call_1777 = call i32 (ptr, ...) @printf(ptr @str_17, ptr %malloc_call63)
  %a_int78 = load i64, ptr %a, align 4
  %printf_call_18 = call i32 (ptr, ...) @printf(ptr @str_18, i64 %a_int78)
  %malloc_call79 = call ptr @malloc(i64 5)
  %char_ptr_080 = getelementptr i8, ptr %malloc_call79, i32 0
  store i8 32, ptr %char_ptr_080, align 1
  %char_ptr_181 = getelementptr i8, ptr %malloc_call79, i32 1
  store i8 43, ptr %char_ptr_181, align 1
  %char_ptr_282 = getelementptr i8, ptr %malloc_call79, i32 2
  store i8 32, ptr %char_ptr_282, align 1
  %char_ptr_383 = getelementptr i8, ptr %malloc_call79, i32 3
  store i8 40, ptr %char_ptr_383, align 1
  %null_ptr84 = getelementptr i8, ptr %malloc_call79, i32 4
  store i8 0, ptr %null_ptr84, align 1
  %printf_call_19 = call i32 (ptr, ...) @printf(ptr @str_19, ptr %malloc_call79)
  %b_int85 = load i64, ptr %b, align 4
  %printf_call_20 = call i32 (ptr, ...) @printf(ptr @str_20, i64 %b_int85)
  %malloc_call86 = call ptr @malloc(i64 9)
  %char_ptr_087 = getelementptr i8, ptr %malloc_call86, i32 0
  store i8 32, ptr %char_ptr_087, align 1
  %char_ptr_188 = getelementptr i8, ptr %malloc_call86, i32 1
  store i8 42, ptr %char_ptr_188, align 1
  %char_ptr_289 = getelementptr i8, ptr %malloc_call86, i32 2
  store i8 32, ptr %char_ptr_289, align 1
  %char_ptr_390 = getelementptr i8, ptr %malloc_call86, i32 3
  store i8 50, ptr %char_ptr_390, align 1
  %char_ptr_491 = getelementptr i8, ptr %malloc_call86, i32 4
  store i8 41, ptr %char_ptr_491, align 1
  %char_ptr_592 = getelementptr i8, ptr %malloc_call86, i32 5
  store i8 32, ptr %char_ptr_592, align 1
  %char_ptr_693 = getelementptr i8, ptr %malloc_call86, i32 6
  store i8 61, ptr %char_ptr_693, align 1
  %char_ptr_794 = getelementptr i8, ptr %malloc_call86, i32 7
  store i8 32, ptr %char_ptr_794, align 1
  %null_ptr95 = getelementptr i8, ptr %malloc_call86, i32 8
  store i8 0, ptr %null_ptr95, align 1
  %printf_call_21 = call i32 (ptr, ...) @printf(ptr @str_21, ptr %malloc_call86)
  %compound1_int = load i64, ptr %compound1, align 4
  %printf_call_22 = call i32 (ptr, ...) @printf(ptr @str_22, i64 %compound1_int)
  %printf_call_24 = call i32 (ptr, ...) @printf(ptr @str_23)
  %sum_int96 = load i64, ptr %sum, align 4
  %difference_int97 = load i64, ptr %difference, align 4
  %add98 = add i64 %sum_int96, %difference_int97
  store i64 %add98, ptr %compound2, align 4
  %malloc_call99 = call ptr @malloc(i64 13)
  %char_ptr_0100 = getelementptr i8, ptr %malloc_call99, i32 0
  store i8 67, ptr %char_ptr_0100, align 1
  %char_ptr_1101 = getelementptr i8, ptr %malloc_call99, i32 1
  store i8 111, ptr %char_ptr_1101, align 1
  %char_ptr_2102 = getelementptr i8, ptr %malloc_call99, i32 2
  store i8 109, ptr %char_ptr_2102, align 1
  %char_ptr_3103 = getelementptr i8, ptr %malloc_call99, i32 3
  store i8 112, ptr %char_ptr_3103, align 1
  %char_ptr_4104 = getelementptr i8, ptr %malloc_call99, i32 4
  store i8 111, ptr %char_ptr_4104, align 1
  %char_ptr_5105 = getelementptr i8, ptr %malloc_call99, i32 5
  store i8 117, ptr %char_ptr_5105, align 1
  %char_ptr_6106 = getelementptr i8, ptr %malloc_call99, i32 6
  store i8 110, ptr %char_ptr_6106, align 1
  %char_ptr_7107 = getelementptr i8, ptr %malloc_call99, i32 7
  store i8 100, ptr %char_ptr_7107, align 1
  %char_ptr_8108 = getelementptr i8, ptr %malloc_call99, i32 8
  store i8 32, ptr %char_ptr_8108, align 1
  %char_ptr_9109 = getelementptr i8, ptr %malloc_call99, i32 9
  store i8 50, ptr %char_ptr_9109, align 1
  %char_ptr_10110 = getelementptr i8, ptr %malloc_call99, i32 10
  store i8 58, ptr %char_ptr_10110, align 1
  %char_ptr_11111 = getelementptr i8, ptr %malloc_call99, i32 11
  store i8 32, ptr %char_ptr_11111, align 1
  %null_ptr112 = getelementptr i8, ptr %malloc_call99, i32 12
  store i8 0, ptr %null_ptr112, align 1
  %printf_call_24113 = call i32 (ptr, ...) @printf(ptr @str_24, ptr %malloc_call99)
  %sum_int114 = load i64, ptr %sum, align 4
  %printf_call_25 = call i32 (ptr, ...) @printf(ptr @str_25, i64 %sum_int114)
  %malloc_call115 = call ptr @malloc(i64 4)
  %char_ptr_0116 = getelementptr i8, ptr %malloc_call115, i32 0
  store i8 32, ptr %char_ptr_0116, align 1
  %char_ptr_1117 = getelementptr i8, ptr %malloc_call115, i32 1
  store i8 43, ptr %char_ptr_1117, align 1
  %char_ptr_2118 = getelementptr i8, ptr %malloc_call115, i32 2
  store i8 32, ptr %char_ptr_2118, align 1
  %null_ptr119 = getelementptr i8, ptr %malloc_call115, i32 3
  store i8 0, ptr %null_ptr119, align 1
  %printf_call_26 = call i32 (ptr, ...) @printf(ptr @str_26, ptr %malloc_call115)
  %difference_int120 = load i64, ptr %difference, align 4
  %printf_call_27 = call i32 (ptr, ...) @printf(ptr @str_27, i64 %difference_int120)
  %malloc_call121 = call ptr @malloc(i64 4)
  %char_ptr_0122 = getelementptr i8, ptr %malloc_call121, i32 0
  store i8 32, ptr %char_ptr_0122, align 1
  %char_ptr_1123 = getelementptr i8, ptr %malloc_call121, i32 1
  store i8 61, ptr %char_ptr_1123, align 1
  %char_ptr_2124 = getelementptr i8, ptr %malloc_call121, i32 2
  store i8 32, ptr %char_ptr_2124, align 1
  %null_ptr125 = getelementptr i8, ptr %malloc_call121, i32 3
  store i8 0, ptr %null_ptr125, align 1
  %printf_call_28 = call i32 (ptr, ...) @printf(ptr @str_28, ptr %malloc_call121)
  %compound2_int = load i64, ptr %compound2, align 4
  %printf_call_29 = call i32 (ptr, ...) @printf(ptr @str_29, i64 %compound2_int)
  %printf_call_31 = call i32 (ptr, ...) @printf(ptr @str_30)
  %a_int126 = load i64, ptr %a, align 4
  %malloc_call127 = call ptr @malloc(i64 20)
  %sprintf_call = call i32 (ptr, ptr, ...) @sprintf(ptr %malloc_call127, ptr @str_31, i64 %a_int126)
  store ptr %malloc_call127, ptr %text_a, align 8
  %b_int128 = load i64, ptr %b, align 4
  %malloc_call129 = call ptr @malloc(i64 20)
  %sprintf_call130 = call i32 (ptr, ptr, ...) @sprintf(ptr %malloc_call129, ptr @str_32, i64 %b_int128)
  store ptr %malloc_call129, ptr %text_b, align 8
  %malloc_call131 = call ptr @malloc(i64 18)
  %char_ptr_0132 = getelementptr i8, ptr %malloc_call131, i32 0
  store i8 84, ptr %char_ptr_0132, align 1
  %char_ptr_1133 = getelementptr i8, ptr %malloc_call131, i32 1
  store i8 101, ptr %char_ptr_1133, align 1
  %char_ptr_2134 = getelementptr i8, ptr %malloc_call131, i32 2
  store i8 120, ptr %char_ptr_2134, align 1
  %char_ptr_3135 = getelementptr i8, ptr %malloc_call131, i32 3
  store i8 116, ptr %char_ptr_3135, align 1
  %char_ptr_4136 = getelementptr i8, ptr %malloc_call131, i32 4
  store i8 32, ptr %char_ptr_4136, align 1
  %char_ptr_5137 = getelementptr i8, ptr %malloc_call131, i32 5
  store i8 97, ptr %char_ptr_5137, align 1
  %char_ptr_6138 = getelementptr i8, ptr %malloc_call131, i32 6
  store i8 32, ptr %char_ptr_6138, align 1
  %char_ptr_7139 = getelementptr i8, ptr %malloc_call131, i32 7
  store i8 43, ptr %char_ptr_7139, align 1
  %char_ptr_8140 = getelementptr i8, ptr %malloc_call131, i32 8
  store i8 32, ptr %char_ptr_8140, align 1
  %char_ptr_9141 = getelementptr i8, ptr %malloc_call131, i32 9
  store i8 84, ptr %char_ptr_9141, align 1
  %char_ptr_10142 = getelementptr i8, ptr %malloc_call131, i32 10
  store i8 101, ptr %char_ptr_10142, align 1
  %char_ptr_11143 = getelementptr i8, ptr %malloc_call131, i32 11
  store i8 120, ptr %char_ptr_11143, align 1
  %char_ptr_12 = getelementptr i8, ptr %malloc_call131, i32 12
  store i8 116, ptr %char_ptr_12, align 1
  %char_ptr_13 = getelementptr i8, ptr %malloc_call131, i32 13
  store i8 32, ptr %char_ptr_13, align 1
  %char_ptr_14 = getelementptr i8, ptr %malloc_call131, i32 14
  store i8 98, ptr %char_ptr_14, align 1
  %char_ptr_15 = getelementptr i8, ptr %malloc_call131, i32 15
  store i8 58, ptr %char_ptr_15, align 1
  %char_ptr_16 = getelementptr i8, ptr %malloc_call131, i32 16
  store i8 32, ptr %char_ptr_16, align 1
  %null_ptr144 = getelementptr i8, ptr %malloc_call131, i32 17
  store i8 0, ptr %null_ptr144, align 1
  %printf_call_33 = call i32 (ptr, ...) @printf(ptr @str_33, ptr %malloc_call131)
  %text_a_ptr = load ptr, ptr %text_a, align 8
  %printf_call_34 = call i32 (ptr, ...) @printf(ptr @str_34, ptr %text_a_ptr)
  %malloc_call145 = call ptr @malloc(i64 4)
  %char_ptr_0146 = getelementptr i8, ptr %malloc_call145, i32 0
  store i8 32, ptr %char_ptr_0146, align 1
  %char_ptr_1147 = getelementptr i8, ptr %malloc_call145, i32 1
  store i8 43, ptr %char_ptr_1147, align 1
  %char_ptr_2148 = getelementptr i8, ptr %malloc_call145, i32 2
  store i8 32, ptr %char_ptr_2148, align 1
  %null_ptr149 = getelementptr i8, ptr %malloc_call145, i32 3
  store i8 0, ptr %null_ptr149, align 1
  %printf_call_35 = call i32 (ptr, ...) @printf(ptr @str_35, ptr %malloc_call145)
  %text_b_ptr = load ptr, ptr %text_b, align 8
  %printf_call_36 = call i32 (ptr, ...) @printf(ptr @str_36, ptr %text_b_ptr)
  %printf_call_38 = call i32 (ptr, ...) @printf(ptr @str_37)
  %malloc_call150 = call ptr @malloc(i64 19)
  %char_ptr_0151 = getelementptr i8, ptr %malloc_call150, i32 0
  store i8 68, ptr %char_ptr_0151, align 1
  %char_ptr_1152 = getelementptr i8, ptr %malloc_call150, i32 1
  store i8 105, ptr %char_ptr_1152, align 1
  %char_ptr_2153 = getelementptr i8, ptr %malloc_call150, i32 2
  store i8 114, ptr %char_ptr_2153, align 1
  %char_ptr_3154 = getelementptr i8, ptr %malloc_call150, i32 3
  store i8 101, ptr %char_ptr_3154, align 1
  %char_ptr_4155 = getelementptr i8, ptr %malloc_call150, i32 4
  store i8 99, ptr %char_ptr_4155, align 1
  %char_ptr_5156 = getelementptr i8, ptr %malloc_call150, i32 5
  store i8 116, ptr %char_ptr_5156, align 1
  %char_ptr_6157 = getelementptr i8, ptr %malloc_call150, i32 6
  store i8 58, ptr %char_ptr_6157, align 1
  %char_ptr_7158 = getelementptr i8, ptr %malloc_call150, i32 7
  store i8 32, ptr %char_ptr_7158, align 1
  %char_ptr_8159 = getelementptr i8, ptr %malloc_call150, i32 8
  store i8 50, ptr %char_ptr_8159, align 1
  %char_ptr_9160 = getelementptr i8, ptr %malloc_call150, i32 9
  store i8 48, ptr %char_ptr_9160, align 1
  %char_ptr_10161 = getelementptr i8, ptr %malloc_call150, i32 10
  store i8 32, ptr %char_ptr_10161, align 1
  %char_ptr_11162 = getelementptr i8, ptr %malloc_call150, i32 11
  store i8 43, ptr %char_ptr_11162, align 1
  %char_ptr_12163 = getelementptr i8, ptr %malloc_call150, i32 12
  store i8 32, ptr %char_ptr_12163, align 1
  %char_ptr_13164 = getelementptr i8, ptr %malloc_call150, i32 13
  store i8 51, ptr %char_ptr_13164, align 1
  %char_ptr_14165 = getelementptr i8, ptr %malloc_call150, i32 14
  store i8 48, ptr %char_ptr_14165, align 1
  %char_ptr_15166 = getelementptr i8, ptr %malloc_call150, i32 15
  store i8 32, ptr %char_ptr_15166, align 1
  %char_ptr_16167 = getelementptr i8, ptr %malloc_call150, i32 16
  store i8 61, ptr %char_ptr_16167, align 1
  %char_ptr_17 = getelementptr i8, ptr %malloc_call150, i32 17
  store i8 32, ptr %char_ptr_17, align 1
  %null_ptr168 = getelementptr i8, ptr %malloc_call150, i32 18
  store i8 0, ptr %null_ptr168, align 1
  %printf_call_38169 = call i32 (ptr, ...) @printf(ptr @str_38, ptr %malloc_call150)
  %printf_call_39 = call i32 (ptr, ...) @printf(ptr @str_39, i64 50)
  %printf_call_41 = call i32 (ptr, ...) @printf(ptr @str_40)
  ret i32 0
}
