; ModuleID = 'unified_syntax'
source_filename = "unified_syntax"

@str_0 = private constant [3 x i8] c"%s\00"
@str_1 = private constant [3 x i8] c"%s\00"
@str_2 = private constant [3 x i8] c"%s\00"
@str_3 = private constant [3 x i8] c"\\n\00"
@str_4 = private constant [5 x i8] c"%lld\00"
@str_5 = private constant [3 x i8] c"%s\00"
@str_6 = private constant [5 x i8] c"%lld\00"
@str_7 = private constant [3 x i8] c"\\n\00"
@str_8 = private constant [3 x i8] c"%s\00"
@str_9 = private constant [5 x i8] c"%lld\00"
@str_10 = private constant [3 x i8] c"\\n\00"
@str_11 = private constant [3 x i8] c"%s\00"
@str_12 = private constant [3 x i8] c"%s\00"
@str_13 = private constant [3 x i8] c"\\n\00"
@str_14 = private constant [5 x i8] c"%lld\00"
@str_15 = private constant [3 x i8] c"%s\00"
@str_16 = private constant [5 x i8] c"%lld\00"
@str_17 = private constant [3 x i8] c"\\n\00"
@str_18 = private constant [3 x i8] c"%s\00"
@str_19 = private constant [3 x i8] c"\\n\00"
@str_20 = private constant [3 x i8] c"%s\00"
@str_21 = private constant [3 x i8] c"\\n\00"
@str_22 = private constant [5 x i8] c"%lld\00"
@str_23 = private constant [3 x i8] c"%s\00"
@str_24 = private constant [5 x i8] c"%lld\00"
@str_25 = private constant [3 x i8] c"\\n\00"
@str_26 = private constant [3 x i8] c"%s\00"
@str_27 = private constant [3 x i8] c"%s\00"
@str_28 = private constant [3 x i8] c"\\n\00"
@str_29 = private constant [3 x i8] c"%s\00"
@str_30 = private constant [5 x i8] c"%lld\00"
@str_31 = private constant [3 x i8] c"\\n\00"

declare i32 @printf(ptr, ...)

declare i32 @sprintf(ptr, ptr, ...)

declare i64 @atoll(ptr)

declare ptr @malloc(i64)

declare i64 @strlen(ptr)

define i32 @main() {
entry:
  %has_value = alloca i64, align 8
  %empty_text = alloca ptr, align 8
  %is_valid = alloca i64, align 8
  %product = alloca i64, align 8
  %sum = alloca i64, align 8
  %char_a = alloca ptr, align 8
  %num_back = alloca i64, align 8
  %text_num = alloca ptr, align 8
  %is_active = alloca i64, align 8
  %num2 = alloca i64, align 8
  %num1 = alloca i64, align 8
  %name = alloca ptr, align 8
  %malloc_call = call ptr @malloc(i64 6)
  %char_ptr_0 = getelementptr i8, ptr %malloc_call, i32 0
  store i8 87, ptr %char_ptr_0, align 1
  %char_ptr_1 = getelementptr i8, ptr %malloc_call, i32 1
  store i8 111, ptr %char_ptr_1, align 1
  %char_ptr_2 = getelementptr i8, ptr %malloc_call, i32 2
  store i8 114, ptr %char_ptr_2, align 1
  %char_ptr_3 = getelementptr i8, ptr %malloc_call, i32 3
  store i8 108, ptr %char_ptr_3, align 1
  %char_ptr_4 = getelementptr i8, ptr %malloc_call, i32 4
  store i8 100, ptr %char_ptr_4, align 1
  %null_ptr = getelementptr i8, ptr %malloc_call, i32 5
  store i8 0, ptr %null_ptr, align 1
  store ptr %malloc_call, ptr %name, align 8
  store i64 5, ptr %num1, align 4
  store i64 10, ptr %num2, align 4
  store i64 1, ptr %is_active, align 4
  %malloc_call1 = call ptr @malloc(i64 8)
  %char_ptr_02 = getelementptr i8, ptr %malloc_call1, i32 0
  store i8 72, ptr %char_ptr_02, align 1
  %char_ptr_13 = getelementptr i8, ptr %malloc_call1, i32 1
  store i8 101, ptr %char_ptr_13, align 1
  %char_ptr_24 = getelementptr i8, ptr %malloc_call1, i32 2
  store i8 108, ptr %char_ptr_24, align 1
  %char_ptr_35 = getelementptr i8, ptr %malloc_call1, i32 3
  store i8 108, ptr %char_ptr_35, align 1
  %char_ptr_46 = getelementptr i8, ptr %malloc_call1, i32 4
  store i8 111, ptr %char_ptr_46, align 1
  %char_ptr_5 = getelementptr i8, ptr %malloc_call1, i32 5
  store i8 44, ptr %char_ptr_5, align 1
  %char_ptr_6 = getelementptr i8, ptr %malloc_call1, i32 6
  store i8 32, ptr %char_ptr_6, align 1
  %null_ptr7 = getelementptr i8, ptr %malloc_call1, i32 7
  store i8 0, ptr %null_ptr7, align 1
  %printf_call_0 = call i32 (ptr, ...) @printf(ptr @str_0, ptr %malloc_call1)
  %name_ptr = load ptr, ptr %name, align 8
  %printf_call_1 = call i32 (ptr, ...) @printf(ptr @str_1, ptr %name_ptr)
  %malloc_call8 = call ptr @malloc(i64 2)
  %char_ptr_09 = getelementptr i8, ptr %malloc_call8, i32 0
  store i8 33, ptr %char_ptr_09, align 1
  %null_ptr10 = getelementptr i8, ptr %malloc_call8, i32 1
  store i8 0, ptr %null_ptr10, align 1
  %printf_call_2 = call i32 (ptr, ...) @printf(ptr @str_2, ptr %malloc_call8)
  %printf_call_4 = call i32 (ptr, ...) @printf(ptr @str_3)
  %num1_int = load i64, ptr %num1, align 4
  %malloc_call11 = call ptr @malloc(i64 20)
  %sprintf_call = call i32 (ptr, ptr, ...) @sprintf(ptr %malloc_call11, ptr @str_4, i64 %num1_int)
  store ptr %malloc_call11, ptr %text_num, align 8
  %text_num_ptr = load ptr, ptr %text_num, align 8
  %atoll_call = call i64 @atoll(ptr %text_num_ptr)
  store i64 %atoll_call, ptr %num_back, align 4
  %malloc_call12 = call ptr @malloc(i64 2)
  store i8 65, ptr %malloc_call12, align 1
  %buffer_plus_one = getelementptr i8, ptr %malloc_call12, i32 1
  store i8 0, ptr %buffer_plus_one, align 1
  store ptr %malloc_call12, ptr %char_a, align 8
  %num1_int13 = load i64, ptr %num1, align 4
  %num2_int = load i64, ptr %num2, align 4
  %add = add i64 %num1_int13, %num2_int
  store i64 %add, ptr %sum, align 4
  %num1_int14 = load i64, ptr %num1, align 4
  %num2_int15 = load i64, ptr %num2, align 4
  %mul = mul i64 %num1_int14, %num2_int15
  store i64 %mul, ptr %product, align 4
  %malloc_call16 = call ptr @malloc(i64 6)
  %char_ptr_017 = getelementptr i8, ptr %malloc_call16, i32 0
  store i8 83, ptr %char_ptr_017, align 1
  %char_ptr_118 = getelementptr i8, ptr %malloc_call16, i32 1
  store i8 117, ptr %char_ptr_118, align 1
  %char_ptr_219 = getelementptr i8, ptr %malloc_call16, i32 2
  store i8 109, ptr %char_ptr_219, align 1
  %char_ptr_320 = getelementptr i8, ptr %malloc_call16, i32 3
  store i8 58, ptr %char_ptr_320, align 1
  %char_ptr_421 = getelementptr i8, ptr %malloc_call16, i32 4
  store i8 32, ptr %char_ptr_421, align 1
  %null_ptr22 = getelementptr i8, ptr %malloc_call16, i32 5
  store i8 0, ptr %null_ptr22, align 1
  %printf_call_5 = call i32 (ptr, ...) @printf(ptr @str_5, ptr %malloc_call16)
  %sum_int = load i64, ptr %sum, align 4
  %printf_call_6 = call i32 (ptr, ...) @printf(ptr @str_6, i64 %sum_int)
  %printf_call_8 = call i32 (ptr, ...) @printf(ptr @str_7)
  %malloc_call23 = call ptr @malloc(i64 10)
  %char_ptr_024 = getelementptr i8, ptr %malloc_call23, i32 0
  store i8 80, ptr %char_ptr_024, align 1
  %char_ptr_125 = getelementptr i8, ptr %malloc_call23, i32 1
  store i8 114, ptr %char_ptr_125, align 1
  %char_ptr_226 = getelementptr i8, ptr %malloc_call23, i32 2
  store i8 111, ptr %char_ptr_226, align 1
  %char_ptr_327 = getelementptr i8, ptr %malloc_call23, i32 3
  store i8 100, ptr %char_ptr_327, align 1
  %char_ptr_428 = getelementptr i8, ptr %malloc_call23, i32 4
  store i8 117, ptr %char_ptr_428, align 1
  %char_ptr_529 = getelementptr i8, ptr %malloc_call23, i32 5
  store i8 99, ptr %char_ptr_529, align 1
  %char_ptr_630 = getelementptr i8, ptr %malloc_call23, i32 6
  store i8 116, ptr %char_ptr_630, align 1
  %char_ptr_7 = getelementptr i8, ptr %malloc_call23, i32 7
  store i8 58, ptr %char_ptr_7, align 1
  %char_ptr_8 = getelementptr i8, ptr %malloc_call23, i32 8
  store i8 32, ptr %char_ptr_8, align 1
  %null_ptr31 = getelementptr i8, ptr %malloc_call23, i32 9
  store i8 0, ptr %null_ptr31, align 1
  %printf_call_832 = call i32 (ptr, ...) @printf(ptr @str_8, ptr %malloc_call23)
  %product_int = load i64, ptr %product, align 4
  %printf_call_9 = call i32 (ptr, ...) @printf(ptr @str_9, i64 %product_int)
  %printf_call_11 = call i32 (ptr, ...) @printf(ptr @str_10)
  %malloc_call33 = call ptr @malloc(i64 14)
  %char_ptr_034 = getelementptr i8, ptr %malloc_call33, i32 0
  store i8 65, ptr %char_ptr_034, align 1
  %char_ptr_135 = getelementptr i8, ptr %malloc_call33, i32 1
  store i8 83, ptr %char_ptr_135, align 1
  %char_ptr_236 = getelementptr i8, ptr %malloc_call33, i32 2
  store i8 67, ptr %char_ptr_236, align 1
  %char_ptr_337 = getelementptr i8, ptr %malloc_call33, i32 3
  store i8 73, ptr %char_ptr_337, align 1
  %char_ptr_438 = getelementptr i8, ptr %malloc_call33, i32 4
  store i8 73, ptr %char_ptr_438, align 1
  %char_ptr_539 = getelementptr i8, ptr %malloc_call33, i32 5
  store i8 32, ptr %char_ptr_539, align 1
  %char_ptr_640 = getelementptr i8, ptr %malloc_call33, i32 6
  store i8 54, ptr %char_ptr_640, align 1
  %char_ptr_741 = getelementptr i8, ptr %malloc_call33, i32 7
  store i8 53, ptr %char_ptr_741, align 1
  %char_ptr_842 = getelementptr i8, ptr %malloc_call33, i32 8
  store i8 32, ptr %char_ptr_842, align 1
  %char_ptr_9 = getelementptr i8, ptr %malloc_call33, i32 9
  store i8 105, ptr %char_ptr_9, align 1
  %char_ptr_10 = getelementptr i8, ptr %malloc_call33, i32 10
  store i8 115, ptr %char_ptr_10, align 1
  %char_ptr_11 = getelementptr i8, ptr %malloc_call33, i32 11
  store i8 58, ptr %char_ptr_11, align 1
  %char_ptr_12 = getelementptr i8, ptr %malloc_call33, i32 12
  store i8 32, ptr %char_ptr_12, align 1
  %null_ptr43 = getelementptr i8, ptr %malloc_call33, i32 13
  store i8 0, ptr %null_ptr43, align 1
  %printf_call_1144 = call i32 (ptr, ...) @printf(ptr @str_11, ptr %malloc_call33)
  %char_a_ptr = load ptr, ptr %char_a, align 8
  %printf_call_12 = call i32 (ptr, ...) @printf(ptr @str_12, ptr %char_a_ptr)
  %printf_call_14 = call i32 (ptr, ...) @printf(ptr @str_13)
  %num1_int45 = load i64, ptr %num1, align 4
  %num2_int46 = load i64, ptr %num2, align 4
  %lt = icmp slt i64 %num1_int45, %num2_int46
  %zext = zext i1 %lt to i64
  %ifcond = icmp ne i64 %zext, 0
  br i1 %ifcond, label %then, label %else

then:                                             ; preds = %entry
  %num1_int47 = load i64, ptr %num1, align 4
  %printf_call_1448 = call i32 (ptr, ...) @printf(ptr @str_14, i64 %num1_int47)
  %malloc_call49 = call ptr @malloc(i64 15)
  %char_ptr_050 = getelementptr i8, ptr %malloc_call49, i32 0
  store i8 32, ptr %char_ptr_050, align 1
  %char_ptr_151 = getelementptr i8, ptr %malloc_call49, i32 1
  store i8 105, ptr %char_ptr_151, align 1
  %char_ptr_252 = getelementptr i8, ptr %malloc_call49, i32 2
  store i8 115, ptr %char_ptr_252, align 1
  %char_ptr_353 = getelementptr i8, ptr %malloc_call49, i32 3
  store i8 32, ptr %char_ptr_353, align 1
  %char_ptr_454 = getelementptr i8, ptr %malloc_call49, i32 4
  store i8 108, ptr %char_ptr_454, align 1
  %char_ptr_555 = getelementptr i8, ptr %malloc_call49, i32 5
  store i8 101, ptr %char_ptr_555, align 1
  %char_ptr_656 = getelementptr i8, ptr %malloc_call49, i32 6
  store i8 115, ptr %char_ptr_656, align 1
  %char_ptr_757 = getelementptr i8, ptr %malloc_call49, i32 7
  store i8 115, ptr %char_ptr_757, align 1
  %char_ptr_858 = getelementptr i8, ptr %malloc_call49, i32 8
  store i8 32, ptr %char_ptr_858, align 1
  %char_ptr_959 = getelementptr i8, ptr %malloc_call49, i32 9
  store i8 116, ptr %char_ptr_959, align 1
  %char_ptr_1060 = getelementptr i8, ptr %malloc_call49, i32 10
  store i8 104, ptr %char_ptr_1060, align 1
  %char_ptr_1161 = getelementptr i8, ptr %malloc_call49, i32 11
  store i8 97, ptr %char_ptr_1161, align 1
  %char_ptr_1262 = getelementptr i8, ptr %malloc_call49, i32 12
  store i8 110, ptr %char_ptr_1262, align 1
  %char_ptr_1363 = getelementptr i8, ptr %malloc_call49, i32 13
  store i8 32, ptr %char_ptr_1363, align 1
  %null_ptr64 = getelementptr i8, ptr %malloc_call49, i32 14
  store i8 0, ptr %null_ptr64, align 1
  %printf_call_15 = call i32 (ptr, ...) @printf(ptr @str_15, ptr %malloc_call49)
  %num2_int65 = load i64, ptr %num2, align 4
  %printf_call_16 = call i32 (ptr, ...) @printf(ptr @str_16, i64 %num2_int65)
  %printf_call_18 = call i32 (ptr, ...) @printf(ptr @str_17)
  %sum_int69 = load i64, ptr %sum, align 4
  %gt = icmp sgt i64 %sum_int69, 10
  %zext70 = zext i1 %gt to i64
  %ifcond71 = icmp ne i64 %zext70, 0
  br i1 %ifcond71, label %then66, label %else67

else:                                             ; preds = %entry
  %num1_int115 = load i64, ptr %num1, align 4
  %printf_call_22116 = call i32 (ptr, ...) @printf(ptr @str_22, i64 %num1_int115)
  %malloc_call117 = call ptr @malloc(i64 19)
  %char_ptr_0118 = getelementptr i8, ptr %malloc_call117, i32 0
  store i8 32, ptr %char_ptr_0118, align 1
  %char_ptr_1119 = getelementptr i8, ptr %malloc_call117, i32 1
  store i8 105, ptr %char_ptr_1119, align 1
  %char_ptr_2120 = getelementptr i8, ptr %malloc_call117, i32 2
  store i8 115, ptr %char_ptr_2120, align 1
  %char_ptr_3121 = getelementptr i8, ptr %malloc_call117, i32 3
  store i8 32, ptr %char_ptr_3121, align 1
  %char_ptr_4122 = getelementptr i8, ptr %malloc_call117, i32 4
  store i8 110, ptr %char_ptr_4122, align 1
  %char_ptr_5123 = getelementptr i8, ptr %malloc_call117, i32 5
  store i8 111, ptr %char_ptr_5123, align 1
  %char_ptr_6124 = getelementptr i8, ptr %malloc_call117, i32 6
  store i8 116, ptr %char_ptr_6124, align 1
  %char_ptr_7125 = getelementptr i8, ptr %malloc_call117, i32 7
  store i8 32, ptr %char_ptr_7125, align 1
  %char_ptr_8126 = getelementptr i8, ptr %malloc_call117, i32 8
  store i8 108, ptr %char_ptr_8126, align 1
  %char_ptr_9127 = getelementptr i8, ptr %malloc_call117, i32 9
  store i8 101, ptr %char_ptr_9127, align 1
  %char_ptr_10128 = getelementptr i8, ptr %malloc_call117, i32 10
  store i8 115, ptr %char_ptr_10128, align 1
  %char_ptr_11129 = getelementptr i8, ptr %malloc_call117, i32 11
  store i8 115, ptr %char_ptr_11129, align 1
  %char_ptr_12130 = getelementptr i8, ptr %malloc_call117, i32 12
  store i8 32, ptr %char_ptr_12130, align 1
  %char_ptr_13131 = getelementptr i8, ptr %malloc_call117, i32 13
  store i8 116, ptr %char_ptr_13131, align 1
  %char_ptr_14132 = getelementptr i8, ptr %malloc_call117, i32 14
  store i8 104, ptr %char_ptr_14132, align 1
  %char_ptr_15133 = getelementptr i8, ptr %malloc_call117, i32 15
  store i8 97, ptr %char_ptr_15133, align 1
  %char_ptr_16134 = getelementptr i8, ptr %malloc_call117, i32 16
  store i8 110, ptr %char_ptr_16134, align 1
  %char_ptr_17135 = getelementptr i8, ptr %malloc_call117, i32 17
  store i8 32, ptr %char_ptr_17135, align 1
  %null_ptr136 = getelementptr i8, ptr %malloc_call117, i32 18
  store i8 0, ptr %null_ptr136, align 1
  %printf_call_23 = call i32 (ptr, ...) @printf(ptr @str_23, ptr %malloc_call117)
  %num2_int137 = load i64, ptr %num2, align 4
  %printf_call_24 = call i32 (ptr, ...) @printf(ptr @str_24, i64 %num2_int137)
  %printf_call_26 = call i32 (ptr, ...) @printf(ptr @str_25)
  br label %ifcont

ifcont:                                           ; preds = %else, %ifcont68
  %num1_int138 = load i64, ptr %num1, align 4
  %gt139 = icmp sgt i64 %num1_int138, 0
  %zext140 = zext i1 %gt139 to i64
  %num2_int141 = load i64, ptr %num2, align 4
  %lt142 = icmp slt i64 %num2_int141, 20
  %zext143 = zext i1 %lt142 to i64
  %left_bool = icmp ne i64 %zext140, 0
  %right_bool = icmp ne i64 %zext143, 0
  %and = and i1 %left_bool, %right_bool
  %zext144 = zext i1 %and to i64
  store i64 %zext144, ptr %is_valid, align 4
  %malloc_call145 = call ptr @malloc(i64 11)
  %char_ptr_0146 = getelementptr i8, ptr %malloc_call145, i32 0
  store i8 73, ptr %char_ptr_0146, align 1
  %char_ptr_1147 = getelementptr i8, ptr %malloc_call145, i32 1
  store i8 115, ptr %char_ptr_1147, align 1
  %char_ptr_2148 = getelementptr i8, ptr %malloc_call145, i32 2
  store i8 32, ptr %char_ptr_2148, align 1
  %char_ptr_3149 = getelementptr i8, ptr %malloc_call145, i32 3
  store i8 118, ptr %char_ptr_3149, align 1
  %char_ptr_4150 = getelementptr i8, ptr %malloc_call145, i32 4
  store i8 97, ptr %char_ptr_4150, align 1
  %char_ptr_5151 = getelementptr i8, ptr %malloc_call145, i32 5
  store i8 108, ptr %char_ptr_5151, align 1
  %char_ptr_6152 = getelementptr i8, ptr %malloc_call145, i32 6
  store i8 105, ptr %char_ptr_6152, align 1
  %char_ptr_7153 = getelementptr i8, ptr %malloc_call145, i32 7
  store i8 100, ptr %char_ptr_7153, align 1
  %char_ptr_8154 = getelementptr i8, ptr %malloc_call145, i32 8
  store i8 58, ptr %char_ptr_8154, align 1
  %char_ptr_9155 = getelementptr i8, ptr %malloc_call145, i32 9
  store i8 32, ptr %char_ptr_9155, align 1
  %null_ptr156 = getelementptr i8, ptr %malloc_call145, i32 10
  store i8 0, ptr %null_ptr156, align 1
  %printf_call_26157 = call i32 (ptr, ...) @printf(ptr @str_26, ptr %malloc_call145)
  %is_valid_int = load i64, ptr %is_valid, align 4
  %ternary_cond = icmp ne i64 %is_valid_int, 0
  br i1 %ternary_cond, label %ternary_then, label %ternary_else

then66:                                           ; preds = %then
  %malloc_call72 = call ptr @malloc(i64 23)
  %char_ptr_073 = getelementptr i8, ptr %malloc_call72, i32 0
  store i8 83, ptr %char_ptr_073, align 1
  %char_ptr_174 = getelementptr i8, ptr %malloc_call72, i32 1
  store i8 117, ptr %char_ptr_174, align 1
  %char_ptr_275 = getelementptr i8, ptr %malloc_call72, i32 2
  store i8 109, ptr %char_ptr_275, align 1
  %char_ptr_376 = getelementptr i8, ptr %malloc_call72, i32 3
  store i8 32, ptr %char_ptr_376, align 1
  %char_ptr_477 = getelementptr i8, ptr %malloc_call72, i32 4
  store i8 105, ptr %char_ptr_477, align 1
  %char_ptr_578 = getelementptr i8, ptr %malloc_call72, i32 5
  store i8 115, ptr %char_ptr_578, align 1
  %char_ptr_679 = getelementptr i8, ptr %malloc_call72, i32 6
  store i8 32, ptr %char_ptr_679, align 1
  %char_ptr_780 = getelementptr i8, ptr %malloc_call72, i32 7
  store i8 103, ptr %char_ptr_780, align 1
  %char_ptr_881 = getelementptr i8, ptr %malloc_call72, i32 8
  store i8 114, ptr %char_ptr_881, align 1
  %char_ptr_982 = getelementptr i8, ptr %malloc_call72, i32 9
  store i8 101, ptr %char_ptr_982, align 1
  %char_ptr_1083 = getelementptr i8, ptr %malloc_call72, i32 10
  store i8 97, ptr %char_ptr_1083, align 1
  %char_ptr_1184 = getelementptr i8, ptr %malloc_call72, i32 11
  store i8 116, ptr %char_ptr_1184, align 1
  %char_ptr_1285 = getelementptr i8, ptr %malloc_call72, i32 12
  store i8 101, ptr %char_ptr_1285, align 1
  %char_ptr_1386 = getelementptr i8, ptr %malloc_call72, i32 13
  store i8 114, ptr %char_ptr_1386, align 1
  %char_ptr_14 = getelementptr i8, ptr %malloc_call72, i32 14
  store i8 32, ptr %char_ptr_14, align 1
  %char_ptr_15 = getelementptr i8, ptr %malloc_call72, i32 15
  store i8 116, ptr %char_ptr_15, align 1
  %char_ptr_16 = getelementptr i8, ptr %malloc_call72, i32 16
  store i8 104, ptr %char_ptr_16, align 1
  %char_ptr_17 = getelementptr i8, ptr %malloc_call72, i32 17
  store i8 97, ptr %char_ptr_17, align 1
  %char_ptr_18 = getelementptr i8, ptr %malloc_call72, i32 18
  store i8 110, ptr %char_ptr_18, align 1
  %char_ptr_19 = getelementptr i8, ptr %malloc_call72, i32 19
  store i8 32, ptr %char_ptr_19, align 1
  %char_ptr_20 = getelementptr i8, ptr %malloc_call72, i32 20
  store i8 49, ptr %char_ptr_20, align 1
  %char_ptr_21 = getelementptr i8, ptr %malloc_call72, i32 21
  store i8 48, ptr %char_ptr_21, align 1
  %null_ptr87 = getelementptr i8, ptr %malloc_call72, i32 22
  store i8 0, ptr %null_ptr87, align 1
  %printf_call_1888 = call i32 (ptr, ...) @printf(ptr @str_18, ptr %malloc_call72)
  %printf_call_20 = call i32 (ptr, ...) @printf(ptr @str_19)
  br label %ifcont68

else67:                                           ; preds = %then
  %malloc_call89 = call ptr @malloc(i64 27)
  %char_ptr_090 = getelementptr i8, ptr %malloc_call89, i32 0
  store i8 83, ptr %char_ptr_090, align 1
  %char_ptr_191 = getelementptr i8, ptr %malloc_call89, i32 1
  store i8 117, ptr %char_ptr_191, align 1
  %char_ptr_292 = getelementptr i8, ptr %malloc_call89, i32 2
  store i8 109, ptr %char_ptr_292, align 1
  %char_ptr_393 = getelementptr i8, ptr %malloc_call89, i32 3
  store i8 32, ptr %char_ptr_393, align 1
  %char_ptr_494 = getelementptr i8, ptr %malloc_call89, i32 4
  store i8 105, ptr %char_ptr_494, align 1
  %char_ptr_595 = getelementptr i8, ptr %malloc_call89, i32 5
  store i8 115, ptr %char_ptr_595, align 1
  %char_ptr_696 = getelementptr i8, ptr %malloc_call89, i32 6
  store i8 32, ptr %char_ptr_696, align 1
  %char_ptr_797 = getelementptr i8, ptr %malloc_call89, i32 7
  store i8 110, ptr %char_ptr_797, align 1
  %char_ptr_898 = getelementptr i8, ptr %malloc_call89, i32 8
  store i8 111, ptr %char_ptr_898, align 1
  %char_ptr_999 = getelementptr i8, ptr %malloc_call89, i32 9
  store i8 116, ptr %char_ptr_999, align 1
  %char_ptr_10100 = getelementptr i8, ptr %malloc_call89, i32 10
  store i8 32, ptr %char_ptr_10100, align 1
  %char_ptr_11101 = getelementptr i8, ptr %malloc_call89, i32 11
  store i8 103, ptr %char_ptr_11101, align 1
  %char_ptr_12102 = getelementptr i8, ptr %malloc_call89, i32 12
  store i8 114, ptr %char_ptr_12102, align 1
  %char_ptr_13103 = getelementptr i8, ptr %malloc_call89, i32 13
  store i8 101, ptr %char_ptr_13103, align 1
  %char_ptr_14104 = getelementptr i8, ptr %malloc_call89, i32 14
  store i8 97, ptr %char_ptr_14104, align 1
  %char_ptr_15105 = getelementptr i8, ptr %malloc_call89, i32 15
  store i8 116, ptr %char_ptr_15105, align 1
  %char_ptr_16106 = getelementptr i8, ptr %malloc_call89, i32 16
  store i8 101, ptr %char_ptr_16106, align 1
  %char_ptr_17107 = getelementptr i8, ptr %malloc_call89, i32 17
  store i8 114, ptr %char_ptr_17107, align 1
  %char_ptr_18108 = getelementptr i8, ptr %malloc_call89, i32 18
  store i8 32, ptr %char_ptr_18108, align 1
  %char_ptr_19109 = getelementptr i8, ptr %malloc_call89, i32 19
  store i8 116, ptr %char_ptr_19109, align 1
  %char_ptr_20110 = getelementptr i8, ptr %malloc_call89, i32 20
  store i8 104, ptr %char_ptr_20110, align 1
  %char_ptr_21111 = getelementptr i8, ptr %malloc_call89, i32 21
  store i8 97, ptr %char_ptr_21111, align 1
  %char_ptr_22 = getelementptr i8, ptr %malloc_call89, i32 22
  store i8 110, ptr %char_ptr_22, align 1
  %char_ptr_23 = getelementptr i8, ptr %malloc_call89, i32 23
  store i8 32, ptr %char_ptr_23, align 1
  %char_ptr_24112 = getelementptr i8, ptr %malloc_call89, i32 24
  store i8 49, ptr %char_ptr_24112, align 1
  %char_ptr_25 = getelementptr i8, ptr %malloc_call89, i32 25
  store i8 48, ptr %char_ptr_25, align 1
  %null_ptr113 = getelementptr i8, ptr %malloc_call89, i32 26
  store i8 0, ptr %null_ptr113, align 1
  %printf_call_20114 = call i32 (ptr, ...) @printf(ptr @str_20, ptr %malloc_call89)
  %printf_call_22 = call i32 (ptr, ...) @printf(ptr @str_21)
  br label %ifcont68

ifcont68:                                         ; preds = %else67, %then66
  br label %ifcont

ternary_then:                                     ; preds = %ifcont
  %malloc_call158 = call ptr @malloc(i64 4)
  %char_ptr_0159 = getelementptr i8, ptr %malloc_call158, i32 0
  store i8 89, ptr %char_ptr_0159, align 1
  %char_ptr_1160 = getelementptr i8, ptr %malloc_call158, i32 1
  store i8 101, ptr %char_ptr_1160, align 1
  %char_ptr_2161 = getelementptr i8, ptr %malloc_call158, i32 2
  store i8 115, ptr %char_ptr_2161, align 1
  %null_ptr162 = getelementptr i8, ptr %malloc_call158, i32 3
  store i8 0, ptr %null_ptr162, align 1
  br label %ternary_merge

ternary_else:                                     ; preds = %ifcont
  %malloc_call163 = call ptr @malloc(i64 3)
  %char_ptr_0164 = getelementptr i8, ptr %malloc_call163, i32 0
  store i8 78, ptr %char_ptr_0164, align 1
  %char_ptr_1165 = getelementptr i8, ptr %malloc_call163, i32 1
  store i8 111, ptr %char_ptr_1165, align 1
  %null_ptr166 = getelementptr i8, ptr %malloc_call163, i32 2
  store i8 0, ptr %null_ptr166, align 1
  br label %ternary_merge

ternary_merge:                                    ; preds = %ternary_else, %ternary_then
  %ternary_result = phi ptr [ %malloc_call158, %ternary_then ], [ %malloc_call163, %ternary_else ]
  %printf_call_27 = call i32 (ptr, ...) @printf(ptr @str_27, ptr %ternary_result)
  %printf_call_29 = call i32 (ptr, ...) @printf(ptr @str_28)
  %malloc_call167 = call ptr @malloc(i64 1)
  %null_ptr168 = getelementptr i8, ptr %malloc_call167, i32 0
  store i8 0, ptr %null_ptr168, align 1
  store ptr %malloc_call167, ptr %empty_text, align 8
  %empty_text_ptr = load ptr, ptr %empty_text, align 8
  %strlen_call = call i64 @strlen(ptr %empty_text_ptr)
  %str_nonempty = icmp ne i64 %strlen_call, 0
  %zext_bool = zext i1 %str_nonempty to i64
  store i64 %zext_bool, ptr %has_value, align 4
  %malloc_call169 = call ptr @malloc(i64 24)
  %char_ptr_0170 = getelementptr i8, ptr %malloc_call169, i32 0
  store i8 69, ptr %char_ptr_0170, align 1
  %char_ptr_1171 = getelementptr i8, ptr %malloc_call169, i32 1
  store i8 109, ptr %char_ptr_1171, align 1
  %char_ptr_2172 = getelementptr i8, ptr %malloc_call169, i32 2
  store i8 112, ptr %char_ptr_2172, align 1
  %char_ptr_3173 = getelementptr i8, ptr %malloc_call169, i32 3
  store i8 116, ptr %char_ptr_3173, align 1
  %char_ptr_4174 = getelementptr i8, ptr %malloc_call169, i32 4
  store i8 121, ptr %char_ptr_4174, align 1
  %char_ptr_5175 = getelementptr i8, ptr %malloc_call169, i32 5
  store i8 32, ptr %char_ptr_5175, align 1
  %char_ptr_6176 = getelementptr i8, ptr %malloc_call169, i32 6
  store i8 116, ptr %char_ptr_6176, align 1
  %char_ptr_7177 = getelementptr i8, ptr %malloc_call169, i32 7
  store i8 101, ptr %char_ptr_7177, align 1
  %char_ptr_8178 = getelementptr i8, ptr %malloc_call169, i32 8
  store i8 120, ptr %char_ptr_8178, align 1
  %char_ptr_9179 = getelementptr i8, ptr %malloc_call169, i32 9
  store i8 116, ptr %char_ptr_9179, align 1
  %char_ptr_10180 = getelementptr i8, ptr %malloc_call169, i32 10
  store i8 32, ptr %char_ptr_10180, align 1
  %char_ptr_11181 = getelementptr i8, ptr %malloc_call169, i32 11
  store i8 97, ptr %char_ptr_11181, align 1
  %char_ptr_12182 = getelementptr i8, ptr %malloc_call169, i32 12
  store i8 115, ptr %char_ptr_12182, align 1
  %char_ptr_13183 = getelementptr i8, ptr %malloc_call169, i32 13
  store i8 32, ptr %char_ptr_13183, align 1
  %char_ptr_14184 = getelementptr i8, ptr %malloc_call169, i32 14
  store i8 98, ptr %char_ptr_14184, align 1
  %char_ptr_15185 = getelementptr i8, ptr %malloc_call169, i32 15
  store i8 111, ptr %char_ptr_15185, align 1
  %char_ptr_16186 = getelementptr i8, ptr %malloc_call169, i32 16
  store i8 111, ptr %char_ptr_16186, align 1
  %char_ptr_17187 = getelementptr i8, ptr %malloc_call169, i32 17
  store i8 108, ptr %char_ptr_17187, align 1
  %char_ptr_18188 = getelementptr i8, ptr %malloc_call169, i32 18
  store i8 101, ptr %char_ptr_18188, align 1
  %char_ptr_19189 = getelementptr i8, ptr %malloc_call169, i32 19
  store i8 97, ptr %char_ptr_19189, align 1
  %char_ptr_20190 = getelementptr i8, ptr %malloc_call169, i32 20
  store i8 110, ptr %char_ptr_20190, align 1
  %char_ptr_21191 = getelementptr i8, ptr %malloc_call169, i32 21
  store i8 58, ptr %char_ptr_21191, align 1
  %char_ptr_22192 = getelementptr i8, ptr %malloc_call169, i32 22
  store i8 32, ptr %char_ptr_22192, align 1
  %null_ptr193 = getelementptr i8, ptr %malloc_call169, i32 23
  store i8 0, ptr %null_ptr193, align 1
  %printf_call_29194 = call i32 (ptr, ...) @printf(ptr @str_29, ptr %malloc_call169)
  %has_value_int = load i64, ptr %has_value, align 4
  %printf_call_30 = call i32 (ptr, ...) @printf(ptr @str_30, i64 %has_value_int)
  %printf_call_32 = call i32 (ptr, ...) @printf(ptr @str_31)
  ret i32 0
}
