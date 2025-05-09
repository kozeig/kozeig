; ModuleID = 'arithmetic'
source_filename = "arithmetic"

@str_0 = private constant [3 x i8] c"%s\00"
@str_1 = private constant [5 x i8] c"%lld\00"
@str_2 = private constant [3 x i8] c"%s\00"
@str_3 = private constant [5 x i8] c"%lld\00"
@str_4 = private constant [3 x i8] c"%s\00"
@str_5 = private constant [5 x i8] c"%lld\00"
@str_6 = private constant [3 x i8] c"\\n\00"
@str_7 = private constant [3 x i8] c"%s\00"
@str_8 = private constant [5 x i8] c"%lld\00"
@str_9 = private constant [3 x i8] c"%s\00"
@str_10 = private constant [5 x i8] c"%lld\00"
@str_11 = private constant [3 x i8] c"%s\00"
@str_12 = private constant [5 x i8] c"%lld\00"
@str_13 = private constant [3 x i8] c"\\n\00"
@str_14 = private constant [3 x i8] c"%s\00"
@str_15 = private constant [5 x i8] c"%lld\00"
@str_16 = private constant [3 x i8] c"%s\00"
@str_17 = private constant [5 x i8] c"%lld\00"
@str_18 = private constant [3 x i8] c"%s\00"
@str_19 = private constant [5 x i8] c"%lld\00"
@str_20 = private constant [3 x i8] c"\\n\00"
@str_21 = private constant [33 x i8] c"Runtime error: Division by zero\0A\00"
@str_22 = private constant [3 x i8] c"%s\00"
@str_23 = private constant [5 x i8] c"%lld\00"
@str_24 = private constant [3 x i8] c"%s\00"
@str_25 = private constant [5 x i8] c"%lld\00"
@str_26 = private constant [3 x i8] c"%s\00"
@str_27 = private constant [5 x i8] c"%lld\00"
@str_28 = private constant [3 x i8] c"\\n\00"
@str_29 = private constant [31 x i8] c"Runtime error: Modulo by zero\0A\00"
@str_30 = private constant [3 x i8] c"%s\00"
@str_31 = private constant [5 x i8] c"%lld\00"
@str_32 = private constant [3 x i8] c"%s\00"
@str_33 = private constant [5 x i8] c"%lld\00"
@str_34 = private constant [3 x i8] c"%s\00"
@str_35 = private constant [5 x i8] c"%lld\00"
@str_36 = private constant [3 x i8] c"\\n\00"
@str_37 = private constant [3 x i8] c"%s\00"
@str_38 = private constant [5 x i8] c"%lld\00"
@str_39 = private constant [3 x i8] c"\\n\00"
@str_40 = private constant [3 x i8] c"%s\00"
@str_41 = private constant [5 x i8] c"%lld\00"
@str_42 = private constant [3 x i8] c"\\n\00"
@str_43 = private constant [33 x i8] c"Runtime error: Division by zero\0A\00"
@str_44 = private constant [31 x i8] c"Runtime error: Modulo by zero\0A\00"
@str_45 = private constant [3 x i8] c"%s\00"
@str_46 = private constant [5 x i8] c"%lld\00"
@str_47 = private constant [3 x i8] c"\\n\00"

declare i32 @printf(ptr, ...)

declare i32 @sprintf(ptr, ptr, ...)

declare i64 @atoll(ptr)

declare ptr @malloc(i64)

declare i64 @strlen(ptr)

define i32 @main() {
entry:
  %compound_text3 = alloca ptr, align 8
  %compound3 = alloca i64, align 8
  %compound_text2 = alloca ptr, align 8
  %compound2 = alloca i64, align 8
  %compound_text1 = alloca ptr, align 8
  %compound1 = alloca i64, align 8
  %modulo = alloca ptr, align 8
  %result_mod = alloca i64, align 8
  %division = alloca ptr, align 8
  %result_div = alloca i64, align 8
  %multiplication = alloca ptr, align 8
  %result_mul = alloca i64, align 8
  %subtraction = alloca ptr, align 8
  %result_sub = alloca i64, align 8
  %equals = alloca ptr, align 8
  %addition = alloca ptr, align 8
  %result_add = alloca i64, align 8
  %b = alloca i64, align 8
  %a = alloca i64, align 8
  store i64 42, ptr %a, align 4
  store i64 7, ptr %b, align 4
  %a_int = load i64, ptr %a, align 4
  %b_int = load i64, ptr %b, align 4
  %add = add i64 %a_int, %b_int
  store i64 %add, ptr %result_add, align 4
  %malloc_call = call ptr @malloc(i64 11)
  %char_ptr_0 = getelementptr i8, ptr %malloc_call, i32 0
  store i8 65, ptr %char_ptr_0, align 1
  %char_ptr_1 = getelementptr i8, ptr %malloc_call, i32 1
  store i8 100, ptr %char_ptr_1, align 1
  %char_ptr_2 = getelementptr i8, ptr %malloc_call, i32 2
  store i8 100, ptr %char_ptr_2, align 1
  %char_ptr_3 = getelementptr i8, ptr %malloc_call, i32 3
  store i8 105, ptr %char_ptr_3, align 1
  %char_ptr_4 = getelementptr i8, ptr %malloc_call, i32 4
  store i8 116, ptr %char_ptr_4, align 1
  %char_ptr_5 = getelementptr i8, ptr %malloc_call, i32 5
  store i8 105, ptr %char_ptr_5, align 1
  %char_ptr_6 = getelementptr i8, ptr %malloc_call, i32 6
  store i8 111, ptr %char_ptr_6, align 1
  %char_ptr_7 = getelementptr i8, ptr %malloc_call, i32 7
  store i8 110, ptr %char_ptr_7, align 1
  %char_ptr_8 = getelementptr i8, ptr %malloc_call, i32 8
  store i8 58, ptr %char_ptr_8, align 1
  %char_ptr_9 = getelementptr i8, ptr %malloc_call, i32 9
  store i8 32, ptr %char_ptr_9, align 1
  %null_ptr = getelementptr i8, ptr %malloc_call, i32 10
  store i8 0, ptr %null_ptr, align 1
  store ptr %malloc_call, ptr %addition, align 8
  %malloc_call1 = call ptr @malloc(i64 4)
  %char_ptr_02 = getelementptr i8, ptr %malloc_call1, i32 0
  store i8 32, ptr %char_ptr_02, align 1
  %char_ptr_13 = getelementptr i8, ptr %malloc_call1, i32 1
  store i8 61, ptr %char_ptr_13, align 1
  %char_ptr_24 = getelementptr i8, ptr %malloc_call1, i32 2
  store i8 32, ptr %char_ptr_24, align 1
  %null_ptr5 = getelementptr i8, ptr %malloc_call1, i32 3
  store i8 0, ptr %null_ptr5, align 1
  store ptr %malloc_call1, ptr %equals, align 8
  %addition_ptr = load ptr, ptr %addition, align 8
  %printf_call_0 = call i32 (ptr, ...) @printf(ptr @str_0, ptr %addition_ptr)
  %a_int6 = load i64, ptr %a, align 4
  %printf_call_1 = call i32 (ptr, ...) @printf(ptr @str_1, i64 %a_int6)
  %malloc_call7 = call ptr @malloc(i64 4)
  %char_ptr_08 = getelementptr i8, ptr %malloc_call7, i32 0
  store i8 32, ptr %char_ptr_08, align 1
  %char_ptr_19 = getelementptr i8, ptr %malloc_call7, i32 1
  store i8 43, ptr %char_ptr_19, align 1
  %char_ptr_210 = getelementptr i8, ptr %malloc_call7, i32 2
  store i8 32, ptr %char_ptr_210, align 1
  %null_ptr11 = getelementptr i8, ptr %malloc_call7, i32 3
  store i8 0, ptr %null_ptr11, align 1
  %printf_call_2 = call i32 (ptr, ...) @printf(ptr @str_2, ptr %malloc_call7)
  %b_int12 = load i64, ptr %b, align 4
  %printf_call_3 = call i32 (ptr, ...) @printf(ptr @str_3, i64 %b_int12)
  %equals_ptr = load ptr, ptr %equals, align 8
  %printf_call_4 = call i32 (ptr, ...) @printf(ptr @str_4, ptr %equals_ptr)
  %result_add_int = load i64, ptr %result_add, align 4
  %printf_call_5 = call i32 (ptr, ...) @printf(ptr @str_5, i64 %result_add_int)
  %printf_call_7 = call i32 (ptr, ...) @printf(ptr @str_6)
  %a_int13 = load i64, ptr %a, align 4
  %b_int14 = load i64, ptr %b, align 4
  %sub = sub i64 %a_int13, %b_int14
  store i64 %sub, ptr %result_sub, align 4
  %malloc_call15 = call ptr @malloc(i64 14)
  %char_ptr_016 = getelementptr i8, ptr %malloc_call15, i32 0
  store i8 83, ptr %char_ptr_016, align 1
  %char_ptr_117 = getelementptr i8, ptr %malloc_call15, i32 1
  store i8 117, ptr %char_ptr_117, align 1
  %char_ptr_218 = getelementptr i8, ptr %malloc_call15, i32 2
  store i8 98, ptr %char_ptr_218, align 1
  %char_ptr_319 = getelementptr i8, ptr %malloc_call15, i32 3
  store i8 116, ptr %char_ptr_319, align 1
  %char_ptr_420 = getelementptr i8, ptr %malloc_call15, i32 4
  store i8 114, ptr %char_ptr_420, align 1
  %char_ptr_521 = getelementptr i8, ptr %malloc_call15, i32 5
  store i8 97, ptr %char_ptr_521, align 1
  %char_ptr_622 = getelementptr i8, ptr %malloc_call15, i32 6
  store i8 99, ptr %char_ptr_622, align 1
  %char_ptr_723 = getelementptr i8, ptr %malloc_call15, i32 7
  store i8 116, ptr %char_ptr_723, align 1
  %char_ptr_824 = getelementptr i8, ptr %malloc_call15, i32 8
  store i8 105, ptr %char_ptr_824, align 1
  %char_ptr_925 = getelementptr i8, ptr %malloc_call15, i32 9
  store i8 111, ptr %char_ptr_925, align 1
  %char_ptr_10 = getelementptr i8, ptr %malloc_call15, i32 10
  store i8 110, ptr %char_ptr_10, align 1
  %char_ptr_11 = getelementptr i8, ptr %malloc_call15, i32 11
  store i8 58, ptr %char_ptr_11, align 1
  %char_ptr_12 = getelementptr i8, ptr %malloc_call15, i32 12
  store i8 32, ptr %char_ptr_12, align 1
  %null_ptr26 = getelementptr i8, ptr %malloc_call15, i32 13
  store i8 0, ptr %null_ptr26, align 1
  store ptr %malloc_call15, ptr %subtraction, align 8
  %subtraction_ptr = load ptr, ptr %subtraction, align 8
  %printf_call_727 = call i32 (ptr, ...) @printf(ptr @str_7, ptr %subtraction_ptr)
  %a_int28 = load i64, ptr %a, align 4
  %printf_call_8 = call i32 (ptr, ...) @printf(ptr @str_8, i64 %a_int28)
  %malloc_call29 = call ptr @malloc(i64 4)
  %char_ptr_030 = getelementptr i8, ptr %malloc_call29, i32 0
  store i8 32, ptr %char_ptr_030, align 1
  %char_ptr_131 = getelementptr i8, ptr %malloc_call29, i32 1
  store i8 45, ptr %char_ptr_131, align 1
  %char_ptr_232 = getelementptr i8, ptr %malloc_call29, i32 2
  store i8 32, ptr %char_ptr_232, align 1
  %null_ptr33 = getelementptr i8, ptr %malloc_call29, i32 3
  store i8 0, ptr %null_ptr33, align 1
  %printf_call_9 = call i32 (ptr, ...) @printf(ptr @str_9, ptr %malloc_call29)
  %b_int34 = load i64, ptr %b, align 4
  %printf_call_10 = call i32 (ptr, ...) @printf(ptr @str_10, i64 %b_int34)
  %equals_ptr35 = load ptr, ptr %equals, align 8
  %printf_call_11 = call i32 (ptr, ...) @printf(ptr @str_11, ptr %equals_ptr35)
  %result_sub_int = load i64, ptr %result_sub, align 4
  %printf_call_12 = call i32 (ptr, ...) @printf(ptr @str_12, i64 %result_sub_int)
  %printf_call_14 = call i32 (ptr, ...) @printf(ptr @str_13)
  %a_int36 = load i64, ptr %a, align 4
  %b_int37 = load i64, ptr %b, align 4
  %mul = mul i64 %a_int36, %b_int37
  store i64 %mul, ptr %result_mul, align 4
  %malloc_call38 = call ptr @malloc(i64 17)
  %char_ptr_039 = getelementptr i8, ptr %malloc_call38, i32 0
  store i8 77, ptr %char_ptr_039, align 1
  %char_ptr_140 = getelementptr i8, ptr %malloc_call38, i32 1
  store i8 117, ptr %char_ptr_140, align 1
  %char_ptr_241 = getelementptr i8, ptr %malloc_call38, i32 2
  store i8 108, ptr %char_ptr_241, align 1
  %char_ptr_342 = getelementptr i8, ptr %malloc_call38, i32 3
  store i8 116, ptr %char_ptr_342, align 1
  %char_ptr_443 = getelementptr i8, ptr %malloc_call38, i32 4
  store i8 105, ptr %char_ptr_443, align 1
  %char_ptr_544 = getelementptr i8, ptr %malloc_call38, i32 5
  store i8 112, ptr %char_ptr_544, align 1
  %char_ptr_645 = getelementptr i8, ptr %malloc_call38, i32 6
  store i8 108, ptr %char_ptr_645, align 1
  %char_ptr_746 = getelementptr i8, ptr %malloc_call38, i32 7
  store i8 105, ptr %char_ptr_746, align 1
  %char_ptr_847 = getelementptr i8, ptr %malloc_call38, i32 8
  store i8 99, ptr %char_ptr_847, align 1
  %char_ptr_948 = getelementptr i8, ptr %malloc_call38, i32 9
  store i8 97, ptr %char_ptr_948, align 1
  %char_ptr_1049 = getelementptr i8, ptr %malloc_call38, i32 10
  store i8 116, ptr %char_ptr_1049, align 1
  %char_ptr_1150 = getelementptr i8, ptr %malloc_call38, i32 11
  store i8 105, ptr %char_ptr_1150, align 1
  %char_ptr_1251 = getelementptr i8, ptr %malloc_call38, i32 12
  store i8 111, ptr %char_ptr_1251, align 1
  %char_ptr_1352 = getelementptr i8, ptr %malloc_call38, i32 13
  store i8 110, ptr %char_ptr_1352, align 1
  %char_ptr_14 = getelementptr i8, ptr %malloc_call38, i32 14
  store i8 58, ptr %char_ptr_14, align 1
  %char_ptr_15 = getelementptr i8, ptr %malloc_call38, i32 15
  store i8 32, ptr %char_ptr_15, align 1
  %null_ptr53 = getelementptr i8, ptr %malloc_call38, i32 16
  store i8 0, ptr %null_ptr53, align 1
  store ptr %malloc_call38, ptr %multiplication, align 8
  %multiplication_ptr = load ptr, ptr %multiplication, align 8
  %printf_call_1454 = call i32 (ptr, ...) @printf(ptr @str_14, ptr %multiplication_ptr)
  %a_int55 = load i64, ptr %a, align 4
  %printf_call_15 = call i32 (ptr, ...) @printf(ptr @str_15, i64 %a_int55)
  %malloc_call56 = call ptr @malloc(i64 4)
  %char_ptr_057 = getelementptr i8, ptr %malloc_call56, i32 0
  store i8 32, ptr %char_ptr_057, align 1
  %char_ptr_158 = getelementptr i8, ptr %malloc_call56, i32 1
  store i8 42, ptr %char_ptr_158, align 1
  %char_ptr_259 = getelementptr i8, ptr %malloc_call56, i32 2
  store i8 32, ptr %char_ptr_259, align 1
  %null_ptr60 = getelementptr i8, ptr %malloc_call56, i32 3
  store i8 0, ptr %null_ptr60, align 1
  %printf_call_16 = call i32 (ptr, ...) @printf(ptr @str_16, ptr %malloc_call56)
  %b_int61 = load i64, ptr %b, align 4
  %printf_call_17 = call i32 (ptr, ...) @printf(ptr @str_17, i64 %b_int61)
  %equals_ptr62 = load ptr, ptr %equals, align 8
  %printf_call_18 = call i32 (ptr, ...) @printf(ptr @str_18, ptr %equals_ptr62)
  %result_mul_int = load i64, ptr %result_mul, align 4
  %printf_call_19 = call i32 (ptr, ...) @printf(ptr @str_19, i64 %result_mul_int)
  %printf_call_21 = call i32 (ptr, ...) @printf(ptr @str_20)
  %a_int63 = load i64, ptr %a, align 4
  %b_int64 = load i64, ptr %b, align 4
  %is_zero = icmp eq i64 %b_int64, 0
  br i1 %is_zero, label %div_by_zero, label %div

div:                                              ; preds = %entry
  %div65 = sdiv i64 %a_int63, %b_int64
  br label %div_cont

div_by_zero:                                      ; preds = %entry
  %div_zero_error_msg = call i32 (ptr, ...) @printf(ptr @str_21)
  ret i32 1

div_cont:                                         ; preds = %div
  %div_result = phi i64 [ %div65, %div ]
  store i64 %div_result, ptr %result_div, align 4
  %malloc_call66 = call ptr @malloc(i64 11)
  %char_ptr_067 = getelementptr i8, ptr %malloc_call66, i32 0
  store i8 68, ptr %char_ptr_067, align 1
  %char_ptr_168 = getelementptr i8, ptr %malloc_call66, i32 1
  store i8 105, ptr %char_ptr_168, align 1
  %char_ptr_269 = getelementptr i8, ptr %malloc_call66, i32 2
  store i8 118, ptr %char_ptr_269, align 1
  %char_ptr_370 = getelementptr i8, ptr %malloc_call66, i32 3
  store i8 105, ptr %char_ptr_370, align 1
  %char_ptr_471 = getelementptr i8, ptr %malloc_call66, i32 4
  store i8 115, ptr %char_ptr_471, align 1
  %char_ptr_572 = getelementptr i8, ptr %malloc_call66, i32 5
  store i8 105, ptr %char_ptr_572, align 1
  %char_ptr_673 = getelementptr i8, ptr %malloc_call66, i32 6
  store i8 111, ptr %char_ptr_673, align 1
  %char_ptr_774 = getelementptr i8, ptr %malloc_call66, i32 7
  store i8 110, ptr %char_ptr_774, align 1
  %char_ptr_875 = getelementptr i8, ptr %malloc_call66, i32 8
  store i8 58, ptr %char_ptr_875, align 1
  %char_ptr_976 = getelementptr i8, ptr %malloc_call66, i32 9
  store i8 32, ptr %char_ptr_976, align 1
  %null_ptr77 = getelementptr i8, ptr %malloc_call66, i32 10
  store i8 0, ptr %null_ptr77, align 1
  store ptr %malloc_call66, ptr %division, align 8
  %division_ptr = load ptr, ptr %division, align 8
  %printf_call_22 = call i32 (ptr, ...) @printf(ptr @str_22, ptr %division_ptr)
  %a_int78 = load i64, ptr %a, align 4
  %printf_call_23 = call i32 (ptr, ...) @printf(ptr @str_23, i64 %a_int78)
  %malloc_call79 = call ptr @malloc(i64 4)
  %char_ptr_080 = getelementptr i8, ptr %malloc_call79, i32 0
  store i8 32, ptr %char_ptr_080, align 1
  %char_ptr_181 = getelementptr i8, ptr %malloc_call79, i32 1
  store i8 47, ptr %char_ptr_181, align 1
  %char_ptr_282 = getelementptr i8, ptr %malloc_call79, i32 2
  store i8 32, ptr %char_ptr_282, align 1
  %null_ptr83 = getelementptr i8, ptr %malloc_call79, i32 3
  store i8 0, ptr %null_ptr83, align 1
  %printf_call_24 = call i32 (ptr, ...) @printf(ptr @str_24, ptr %malloc_call79)
  %b_int84 = load i64, ptr %b, align 4
  %printf_call_25 = call i32 (ptr, ...) @printf(ptr @str_25, i64 %b_int84)
  %equals_ptr85 = load ptr, ptr %equals, align 8
  %printf_call_26 = call i32 (ptr, ...) @printf(ptr @str_26, ptr %equals_ptr85)
  %result_div_int = load i64, ptr %result_div, align 4
  %printf_call_27 = call i32 (ptr, ...) @printf(ptr @str_27, i64 %result_div_int)
  %printf_call_29 = call i32 (ptr, ...) @printf(ptr @str_28)
  %a_int86 = load i64, ptr %a, align 4
  %b_int87 = load i64, ptr %b, align 4
  %is_zero88 = icmp eq i64 %b_int87, 0
  br i1 %is_zero88, label %mod_by_zero, label %mod

mod:                                              ; preds = %div_cont
  %mod89 = srem i64 %a_int86, %b_int87
  br label %mod_cont

mod_by_zero:                                      ; preds = %div_cont
  %mod_zero_error_msg = call i32 (ptr, ...) @printf(ptr @str_29)
  ret i32 1

mod_cont:                                         ; preds = %mod
  %mod_result = phi i64 [ %mod89, %mod ]
  store i64 %mod_result, ptr %result_mod, align 4
  %malloc_call90 = call ptr @malloc(i64 9)
  %char_ptr_091 = getelementptr i8, ptr %malloc_call90, i32 0
  store i8 77, ptr %char_ptr_091, align 1
  %char_ptr_192 = getelementptr i8, ptr %malloc_call90, i32 1
  store i8 111, ptr %char_ptr_192, align 1
  %char_ptr_293 = getelementptr i8, ptr %malloc_call90, i32 2
  store i8 100, ptr %char_ptr_293, align 1
  %char_ptr_394 = getelementptr i8, ptr %malloc_call90, i32 3
  store i8 117, ptr %char_ptr_394, align 1
  %char_ptr_495 = getelementptr i8, ptr %malloc_call90, i32 4
  store i8 108, ptr %char_ptr_495, align 1
  %char_ptr_596 = getelementptr i8, ptr %malloc_call90, i32 5
  store i8 111, ptr %char_ptr_596, align 1
  %char_ptr_697 = getelementptr i8, ptr %malloc_call90, i32 6
  store i8 58, ptr %char_ptr_697, align 1
  %char_ptr_798 = getelementptr i8, ptr %malloc_call90, i32 7
  store i8 32, ptr %char_ptr_798, align 1
  %null_ptr99 = getelementptr i8, ptr %malloc_call90, i32 8
  store i8 0, ptr %null_ptr99, align 1
  store ptr %malloc_call90, ptr %modulo, align 8
  %modulo_ptr = load ptr, ptr %modulo, align 8
  %printf_call_30 = call i32 (ptr, ...) @printf(ptr @str_30, ptr %modulo_ptr)
  %a_int100 = load i64, ptr %a, align 4
  %printf_call_31 = call i32 (ptr, ...) @printf(ptr @str_31, i64 %a_int100)
  %malloc_call101 = call ptr @malloc(i64 4)
  %char_ptr_0102 = getelementptr i8, ptr %malloc_call101, i32 0
  store i8 32, ptr %char_ptr_0102, align 1
  %char_ptr_1103 = getelementptr i8, ptr %malloc_call101, i32 1
  store i8 37, ptr %char_ptr_1103, align 1
  %char_ptr_2104 = getelementptr i8, ptr %malloc_call101, i32 2
  store i8 32, ptr %char_ptr_2104, align 1
  %null_ptr105 = getelementptr i8, ptr %malloc_call101, i32 3
  store i8 0, ptr %null_ptr105, align 1
  %printf_call_32 = call i32 (ptr, ...) @printf(ptr @str_32, ptr %malloc_call101)
  %b_int106 = load i64, ptr %b, align 4
  %printf_call_33 = call i32 (ptr, ...) @printf(ptr @str_33, i64 %b_int106)
  %equals_ptr107 = load ptr, ptr %equals, align 8
  %printf_call_34 = call i32 (ptr, ...) @printf(ptr @str_34, ptr %equals_ptr107)
  %result_mod_int = load i64, ptr %result_mod, align 4
  %printf_call_35 = call i32 (ptr, ...) @printf(ptr @str_35, i64 %result_mod_int)
  %printf_call_37 = call i32 (ptr, ...) @printf(ptr @str_36)
  %a_int108 = load i64, ptr %a, align 4
  %b_int109 = load i64, ptr %b, align 4
  %mul110 = mul i64 %b_int109, 2
  %add111 = add i64 %a_int108, %mul110
  store i64 %add111, ptr %compound1, align 4
  %malloc_call112 = call ptr @malloc(i64 23)
  %char_ptr_0113 = getelementptr i8, ptr %malloc_call112, i32 0
  store i8 67, ptr %char_ptr_0113, align 1
  %char_ptr_1114 = getelementptr i8, ptr %malloc_call112, i32 1
  store i8 111, ptr %char_ptr_1114, align 1
  %char_ptr_2115 = getelementptr i8, ptr %malloc_call112, i32 2
  store i8 109, ptr %char_ptr_2115, align 1
  %char_ptr_3116 = getelementptr i8, ptr %malloc_call112, i32 3
  store i8 112, ptr %char_ptr_3116, align 1
  %char_ptr_4117 = getelementptr i8, ptr %malloc_call112, i32 4
  store i8 111, ptr %char_ptr_4117, align 1
  %char_ptr_5118 = getelementptr i8, ptr %malloc_call112, i32 5
  store i8 117, ptr %char_ptr_5118, align 1
  %char_ptr_6119 = getelementptr i8, ptr %malloc_call112, i32 6
  store i8 110, ptr %char_ptr_6119, align 1
  %char_ptr_7120 = getelementptr i8, ptr %malloc_call112, i32 7
  store i8 100, ptr %char_ptr_7120, align 1
  %char_ptr_8121 = getelementptr i8, ptr %malloc_call112, i32 8
  store i8 32, ptr %char_ptr_8121, align 1
  %char_ptr_9122 = getelementptr i8, ptr %malloc_call112, i32 9
  store i8 40, ptr %char_ptr_9122, align 1
  %char_ptr_10123 = getelementptr i8, ptr %malloc_call112, i32 10
  store i8 97, ptr %char_ptr_10123, align 1
  %char_ptr_11124 = getelementptr i8, ptr %malloc_call112, i32 11
  store i8 32, ptr %char_ptr_11124, align 1
  %char_ptr_12125 = getelementptr i8, ptr %malloc_call112, i32 12
  store i8 43, ptr %char_ptr_12125, align 1
  %char_ptr_13126 = getelementptr i8, ptr %malloc_call112, i32 13
  store i8 32, ptr %char_ptr_13126, align 1
  %char_ptr_14127 = getelementptr i8, ptr %malloc_call112, i32 14
  store i8 98, ptr %char_ptr_14127, align 1
  %char_ptr_15128 = getelementptr i8, ptr %malloc_call112, i32 15
  store i8 32, ptr %char_ptr_15128, align 1
  %char_ptr_16 = getelementptr i8, ptr %malloc_call112, i32 16
  store i8 42, ptr %char_ptr_16, align 1
  %char_ptr_17 = getelementptr i8, ptr %malloc_call112, i32 17
  store i8 32, ptr %char_ptr_17, align 1
  %char_ptr_18 = getelementptr i8, ptr %malloc_call112, i32 18
  store i8 50, ptr %char_ptr_18, align 1
  %char_ptr_19129 = getelementptr i8, ptr %malloc_call112, i32 19
  store i8 41, ptr %char_ptr_19129, align 1
  %char_ptr_20 = getelementptr i8, ptr %malloc_call112, i32 20
  store i8 58, ptr %char_ptr_20, align 1
  %char_ptr_21 = getelementptr i8, ptr %malloc_call112, i32 21
  store i8 32, ptr %char_ptr_21, align 1
  %null_ptr130 = getelementptr i8, ptr %malloc_call112, i32 22
  store i8 0, ptr %null_ptr130, align 1
  store ptr %malloc_call112, ptr %compound_text1, align 8
  %compound_text1_ptr = load ptr, ptr %compound_text1, align 8
  %printf_call_37131 = call i32 (ptr, ...) @printf(ptr @str_37, ptr %compound_text1_ptr)
  %compound1_int = load i64, ptr %compound1, align 4
  %printf_call_38 = call i32 (ptr, ...) @printf(ptr @str_38, i64 %compound1_int)
  %printf_call_40 = call i32 (ptr, ...) @printf(ptr @str_39)
  %a_int132 = load i64, ptr %a, align 4
  %b_int133 = load i64, ptr %b, align 4
  %add134 = add i64 %a_int132, %b_int133
  %mul135 = mul i64 %add134, 2
  store i64 %mul135, ptr %compound2, align 4
  %malloc_call136 = call ptr @malloc(i64 25)
  %char_ptr_0137 = getelementptr i8, ptr %malloc_call136, i32 0
  store i8 67, ptr %char_ptr_0137, align 1
  %char_ptr_1138 = getelementptr i8, ptr %malloc_call136, i32 1
  store i8 111, ptr %char_ptr_1138, align 1
  %char_ptr_2139 = getelementptr i8, ptr %malloc_call136, i32 2
  store i8 109, ptr %char_ptr_2139, align 1
  %char_ptr_3140 = getelementptr i8, ptr %malloc_call136, i32 3
  store i8 112, ptr %char_ptr_3140, align 1
  %char_ptr_4141 = getelementptr i8, ptr %malloc_call136, i32 4
  store i8 111, ptr %char_ptr_4141, align 1
  %char_ptr_5142 = getelementptr i8, ptr %malloc_call136, i32 5
  store i8 117, ptr %char_ptr_5142, align 1
  %char_ptr_6143 = getelementptr i8, ptr %malloc_call136, i32 6
  store i8 110, ptr %char_ptr_6143, align 1
  %char_ptr_7144 = getelementptr i8, ptr %malloc_call136, i32 7
  store i8 100, ptr %char_ptr_7144, align 1
  %char_ptr_8145 = getelementptr i8, ptr %malloc_call136, i32 8
  store i8 32, ptr %char_ptr_8145, align 1
  %char_ptr_9146 = getelementptr i8, ptr %malloc_call136, i32 9
  store i8 40, ptr %char_ptr_9146, align 1
  %char_ptr_10147 = getelementptr i8, ptr %malloc_call136, i32 10
  store i8 40, ptr %char_ptr_10147, align 1
  %char_ptr_11148 = getelementptr i8, ptr %malloc_call136, i32 11
  store i8 97, ptr %char_ptr_11148, align 1
  %char_ptr_12149 = getelementptr i8, ptr %malloc_call136, i32 12
  store i8 32, ptr %char_ptr_12149, align 1
  %char_ptr_13150 = getelementptr i8, ptr %malloc_call136, i32 13
  store i8 43, ptr %char_ptr_13150, align 1
  %char_ptr_14151 = getelementptr i8, ptr %malloc_call136, i32 14
  store i8 32, ptr %char_ptr_14151, align 1
  %char_ptr_15152 = getelementptr i8, ptr %malloc_call136, i32 15
  store i8 98, ptr %char_ptr_15152, align 1
  %char_ptr_16153 = getelementptr i8, ptr %malloc_call136, i32 16
  store i8 41, ptr %char_ptr_16153, align 1
  %char_ptr_17154 = getelementptr i8, ptr %malloc_call136, i32 17
  store i8 32, ptr %char_ptr_17154, align 1
  %char_ptr_18155 = getelementptr i8, ptr %malloc_call136, i32 18
  store i8 42, ptr %char_ptr_18155, align 1
  %char_ptr_19156 = getelementptr i8, ptr %malloc_call136, i32 19
  store i8 32, ptr %char_ptr_19156, align 1
  %char_ptr_20157 = getelementptr i8, ptr %malloc_call136, i32 20
  store i8 50, ptr %char_ptr_20157, align 1
  %char_ptr_21158 = getelementptr i8, ptr %malloc_call136, i32 21
  store i8 41, ptr %char_ptr_21158, align 1
  %char_ptr_22 = getelementptr i8, ptr %malloc_call136, i32 22
  store i8 58, ptr %char_ptr_22, align 1
  %char_ptr_23 = getelementptr i8, ptr %malloc_call136, i32 23
  store i8 32, ptr %char_ptr_23, align 1
  %null_ptr159 = getelementptr i8, ptr %malloc_call136, i32 24
  store i8 0, ptr %null_ptr159, align 1
  store ptr %malloc_call136, ptr %compound_text2, align 8
  %compound_text2_ptr = load ptr, ptr %compound_text2, align 8
  %printf_call_40160 = call i32 (ptr, ...) @printf(ptr @str_40, ptr %compound_text2_ptr)
  %compound2_int = load i64, ptr %compound2, align 4
  %printf_call_41 = call i32 (ptr, ...) @printf(ptr @str_41, i64 %compound2_int)
  %printf_call_43 = call i32 (ptr, ...) @printf(ptr @str_42)
  %a_int161 = load i64, ptr %a, align 4
  %b_int162 = load i64, ptr %b, align 4
  %mul163 = mul i64 %b_int162, 3
  %add164 = add i64 %a_int161, %mul163
  br i1 false, label %div_by_zero166, label %div165

div165:                                           ; preds = %mod_cont
  %div168 = sdiv i64 %add164, 1
  br label %div_cont167

div_by_zero166:                                   ; preds = %mod_cont
  %div_zero_error_msg169 = call i32 (ptr, ...) @printf(ptr @str_43)
  ret i32 1

div_cont167:                                      ; preds = %div165
  %div_result170 = phi i64 [ %div168, %div165 ]
  br i1 false, label %mod_by_zero172, label %mod171

mod171:                                           ; preds = %div_cont167
  %mod174 = srem i64 %div_result170, 10
  br label %mod_cont173

mod_by_zero172:                                   ; preds = %div_cont167
  %mod_zero_error_msg175 = call i32 (ptr, ...) @printf(ptr @str_44)
  ret i32 1

mod_cont173:                                      ; preds = %mod171
  %mod_result176 = phi i64 [ %mod174, %mod171 ]
  store i64 %mod_result176, ptr %compound3, align 4
  %malloc_call177 = call ptr @malloc(i64 39)
  %char_ptr_0178 = getelementptr i8, ptr %malloc_call177, i32 0
  store i8 67, ptr %char_ptr_0178, align 1
  %char_ptr_1179 = getelementptr i8, ptr %malloc_call177, i32 1
  store i8 111, ptr %char_ptr_1179, align 1
  %char_ptr_2180 = getelementptr i8, ptr %malloc_call177, i32 2
  store i8 109, ptr %char_ptr_2180, align 1
  %char_ptr_3181 = getelementptr i8, ptr %malloc_call177, i32 3
  store i8 112, ptr %char_ptr_3181, align 1
  %char_ptr_4182 = getelementptr i8, ptr %malloc_call177, i32 4
  store i8 108, ptr %char_ptr_4182, align 1
  %char_ptr_5183 = getelementptr i8, ptr %malloc_call177, i32 5
  store i8 101, ptr %char_ptr_5183, align 1
  %char_ptr_6184 = getelementptr i8, ptr %malloc_call177, i32 6
  store i8 120, ptr %char_ptr_6184, align 1
  %char_ptr_7185 = getelementptr i8, ptr %malloc_call177, i32 7
  store i8 58, ptr %char_ptr_7185, align 1
  %char_ptr_8186 = getelementptr i8, ptr %malloc_call177, i32 8
  store i8 32, ptr %char_ptr_8186, align 1
  %char_ptr_9187 = getelementptr i8, ptr %malloc_call177, i32 9
  store i8 40, ptr %char_ptr_9187, align 1
  %char_ptr_10188 = getelementptr i8, ptr %malloc_call177, i32 10
  store i8 97, ptr %char_ptr_10188, align 1
  %char_ptr_11189 = getelementptr i8, ptr %malloc_call177, i32 11
  store i8 32, ptr %char_ptr_11189, align 1
  %char_ptr_12190 = getelementptr i8, ptr %malloc_call177, i32 12
  store i8 43, ptr %char_ptr_12190, align 1
  %char_ptr_13191 = getelementptr i8, ptr %malloc_call177, i32 13
  store i8 32, ptr %char_ptr_13191, align 1
  %char_ptr_14192 = getelementptr i8, ptr %malloc_call177, i32 14
  store i8 98, ptr %char_ptr_14192, align 1
  %char_ptr_15193 = getelementptr i8, ptr %malloc_call177, i32 15
  store i8 32, ptr %char_ptr_15193, align 1
  %char_ptr_16194 = getelementptr i8, ptr %malloc_call177, i32 16
  store i8 42, ptr %char_ptr_16194, align 1
  %char_ptr_17195 = getelementptr i8, ptr %malloc_call177, i32 17
  store i8 32, ptr %char_ptr_17195, align 1
  %char_ptr_18196 = getelementptr i8, ptr %malloc_call177, i32 18
  store i8 51, ptr %char_ptr_18196, align 1
  %char_ptr_19197 = getelementptr i8, ptr %malloc_call177, i32 19
  store i8 41, ptr %char_ptr_19197, align 1
  %char_ptr_20198 = getelementptr i8, ptr %malloc_call177, i32 20
  store i8 32, ptr %char_ptr_20198, align 1
  %char_ptr_21199 = getelementptr i8, ptr %malloc_call177, i32 21
  store i8 47, ptr %char_ptr_21199, align 1
  %char_ptr_22200 = getelementptr i8, ptr %malloc_call177, i32 22
  store i8 32, ptr %char_ptr_22200, align 1
  %char_ptr_23201 = getelementptr i8, ptr %malloc_call177, i32 23
  store i8 40, ptr %char_ptr_23201, align 1
  %char_ptr_24202 = getelementptr i8, ptr %malloc_call177, i32 24
  store i8 50, ptr %char_ptr_24202, align 1
  %char_ptr_25 = getelementptr i8, ptr %malloc_call177, i32 25
  store i8 32, ptr %char_ptr_25, align 1
  %char_ptr_26 = getelementptr i8, ptr %malloc_call177, i32 26
  store i8 45, ptr %char_ptr_26, align 1
  %char_ptr_27 = getelementptr i8, ptr %malloc_call177, i32 27
  store i8 32, ptr %char_ptr_27, align 1
  %char_ptr_28 = getelementptr i8, ptr %malloc_call177, i32 28
  store i8 49, ptr %char_ptr_28, align 1
  %char_ptr_29 = getelementptr i8, ptr %malloc_call177, i32 29
  store i8 41, ptr %char_ptr_29, align 1
  %char_ptr_30 = getelementptr i8, ptr %malloc_call177, i32 30
  store i8 32, ptr %char_ptr_30, align 1
  %char_ptr_31 = getelementptr i8, ptr %malloc_call177, i32 31
  store i8 37, ptr %char_ptr_31, align 1
  %char_ptr_32 = getelementptr i8, ptr %malloc_call177, i32 32
  store i8 32, ptr %char_ptr_32, align 1
  %char_ptr_33 = getelementptr i8, ptr %malloc_call177, i32 33
  store i8 49, ptr %char_ptr_33, align 1
  %char_ptr_34 = getelementptr i8, ptr %malloc_call177, i32 34
  store i8 48, ptr %char_ptr_34, align 1
  %char_ptr_35 = getelementptr i8, ptr %malloc_call177, i32 35
  store i8 32, ptr %char_ptr_35, align 1
  %char_ptr_36 = getelementptr i8, ptr %malloc_call177, i32 36
  store i8 61, ptr %char_ptr_36, align 1
  %char_ptr_37 = getelementptr i8, ptr %malloc_call177, i32 37
  store i8 32, ptr %char_ptr_37, align 1
  %null_ptr203 = getelementptr i8, ptr %malloc_call177, i32 38
  store i8 0, ptr %null_ptr203, align 1
  store ptr %malloc_call177, ptr %compound_text3, align 8
  %compound_text3_ptr = load ptr, ptr %compound_text3, align 8
  %printf_call_45 = call i32 (ptr, ...) @printf(ptr @str_45, ptr %compound_text3_ptr)
  %compound3_int = load i64, ptr %compound3, align 4
  %printf_call_46 = call i32 (ptr, ...) @printf(ptr @str_46, i64 %compound3_int)
  %printf_call_48 = call i32 (ptr, ...) @printf(ptr @str_47)
  ret i32 0
}
