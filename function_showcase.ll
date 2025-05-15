; ModuleID = 'function_showcase'
source_filename = "function_showcase"

@str_0 = private constant [3 x i8] c"%s\00"
@str_1 = private constant [3 x i8] c"\\n\00"
@str_2 = private constant [5 x i8] c"%lld\00"

declare i32 @printf(ptr, ...)

declare i32 @sprintf(ptr, ptr, ...)

declare i64 @atoll(ptr)

declare ptr @malloc(i64)

declare i64 @strlen(ptr)

define i64 @hello() {
entry:
  %malloc_call = call ptr @malloc(i64 14)
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
  store i8 44, ptr %char_ptr_5, align 1
  %char_ptr_6 = getelementptr i8, ptr %malloc_call, i32 6
  store i8 32, ptr %char_ptr_6, align 1
  %char_ptr_7 = getelementptr i8, ptr %malloc_call, i32 7
  store i8 119, ptr %char_ptr_7, align 1
  %char_ptr_8 = getelementptr i8, ptr %malloc_call, i32 8
  store i8 111, ptr %char_ptr_8, align 1
  %char_ptr_9 = getelementptr i8, ptr %malloc_call, i32 9
  store i8 114, ptr %char_ptr_9, align 1
  %char_ptr_10 = getelementptr i8, ptr %malloc_call, i32 10
  store i8 108, ptr %char_ptr_10, align 1
  %char_ptr_11 = getelementptr i8, ptr %malloc_call, i32 11
  store i8 100, ptr %char_ptr_11, align 1
  %char_ptr_12 = getelementptr i8, ptr %malloc_call, i32 12
  store i8 33, ptr %char_ptr_12, align 1
  %null_ptr = getelementptr i8, ptr %malloc_call, i32 13
  store i8 0, ptr %null_ptr, align 1
  %printf_call_0 = call i32 (ptr, ...) @printf(ptr @str_0, ptr %malloc_call)
  %printf_call_2 = call i32 (ptr, ...) @printf(ptr @str_1)
  %malloc_call1 = call ptr @malloc(i64 6)
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
  %null_ptr7 = getelementptr i8, ptr %malloc_call1, i32 5
  store i8 0, ptr %null_ptr7, align 1
  ret i64 0
}

define i64 @add(i64 %0, i64 %1) {
entry:
  %b = alloca i64, align 8
  %a = alloca i64, align 8
  store i64 %0, ptr %a, align 4
  store i64 %1, ptr %b, align 4
  %a1 = load i64, ptr %a, align 4
  %b2 = load i64, ptr %b, align 4
  %add = add i64 %a1, %b2
  ret i64 0
}

define i64 @get_greeting() {
entry:
  %malloc_call = call ptr @malloc(i64 12)
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
  store i8 116, ptr %char_ptr_6, align 1
  %char_ptr_7 = getelementptr i8, ptr %malloc_call, i32 7
  store i8 104, ptr %char_ptr_7, align 1
  %char_ptr_8 = getelementptr i8, ptr %malloc_call, i32 8
  store i8 101, ptr %char_ptr_8, align 1
  %char_ptr_9 = getelementptr i8, ptr %malloc_call, i32 9
  store i8 114, ptr %char_ptr_9, align 1
  %char_ptr_10 = getelementptr i8, ptr %malloc_call, i32 10
  store i8 101, ptr %char_ptr_10, align 1
  %null_ptr = getelementptr i8, ptr %malloc_call, i32 11
  store i8 0, ptr %null_ptr, align 1
  ret i64 0
}

define i64 @fibonacci(i64 %0) {
entry:
  %n = alloca i64, align 8
  store i64 %0, ptr %n, align 4
  %n1 = load i64, ptr %n, align 4
  %le = icmp sle i64 %n1, 0
  %zext = zext i1 %le to i64
  %ifcond = icmp ne i64 %zext, 0
  br i1 %ifcond, label %then, label %else

then:                                             ; preds = %entry
  br label %ifcont

else:                                             ; preds = %entry
  %n5 = load i64, ptr %n, align 4
  %eq = icmp eq i64 %n5, 1
  %zext6 = zext i1 %eq to i64
  %ifcond7 = icmp ne i64 %zext6, 0
  br i1 %ifcond7, label %then2, label %else3

ifcont:                                           ; preds = %ifcont4, %then
  ret i64 0

then2:                                            ; preds = %else
  br label %ifcont4

else3:                                            ; preds = %else
  %n8 = load i64, ptr %n, align 4
  %sub = sub i64 %n8, 1
  %call_2 = tail call i64 @fibonacci(i64 %sub)
  %n9 = load i64, ptr %n, align 4
  %sub10 = sub i64 %n9, 2
  %call_211 = tail call i64 @fibonacci(i64 %sub10)
  %add = add i64 %call_2, %call_211
  br label %ifcont4

ifcont4:                                          ; preds = %else3, %then2
  br label %ifcont
}

define i64 @main() {
entry:
  %greeting = alloca i64, align 8
  %malloc_call = call ptr @malloc(i64 26)
  %char_ptr_0 = getelementptr i8, ptr %malloc_call, i32 0
  store i8 84, ptr %char_ptr_0, align 1
  %char_ptr_1 = getelementptr i8, ptr %malloc_call, i32 1
  store i8 101, ptr %char_ptr_1, align 1
  %char_ptr_2 = getelementptr i8, ptr %malloc_call, i32 2
  store i8 115, ptr %char_ptr_2, align 1
  %char_ptr_3 = getelementptr i8, ptr %malloc_call, i32 3
  store i8 116, ptr %char_ptr_3, align 1
  %char_ptr_4 = getelementptr i8, ptr %malloc_call, i32 4
  store i8 105, ptr %char_ptr_4, align 1
  %char_ptr_5 = getelementptr i8, ptr %malloc_call, i32 5
  store i8 110, ptr %char_ptr_5, align 1
  %char_ptr_6 = getelementptr i8, ptr %malloc_call, i32 6
  store i8 103, ptr %char_ptr_6, align 1
  %char_ptr_7 = getelementptr i8, ptr %malloc_call, i32 7
  store i8 32, ptr %char_ptr_7, align 1
  %char_ptr_8 = getelementptr i8, ptr %malloc_call, i32 8
  store i8 115, ptr %char_ptr_8, align 1
  %char_ptr_9 = getelementptr i8, ptr %malloc_call, i32 9
  store i8 105, ptr %char_ptr_9, align 1
  %char_ptr_10 = getelementptr i8, ptr %malloc_call, i32 10
  store i8 109, ptr %char_ptr_10, align 1
  %char_ptr_11 = getelementptr i8, ptr %malloc_call, i32 11
  store i8 112, ptr %char_ptr_11, align 1
  %char_ptr_12 = getelementptr i8, ptr %malloc_call, i32 12
  store i8 108, ptr %char_ptr_12, align 1
  %char_ptr_13 = getelementptr i8, ptr %malloc_call, i32 13
  store i8 101, ptr %char_ptr_13, align 1
  %char_ptr_14 = getelementptr i8, ptr %malloc_call, i32 14
  store i8 32, ptr %char_ptr_14, align 1
  %char_ptr_15 = getelementptr i8, ptr %malloc_call, i32 15
  store i8 102, ptr %char_ptr_15, align 1
  %char_ptr_16 = getelementptr i8, ptr %malloc_call, i32 16
  store i8 117, ptr %char_ptr_16, align 1
  %char_ptr_17 = getelementptr i8, ptr %malloc_call, i32 17
  store i8 110, ptr %char_ptr_17, align 1
  %char_ptr_18 = getelementptr i8, ptr %malloc_call, i32 18
  store i8 99, ptr %char_ptr_18, align 1
  %char_ptr_19 = getelementptr i8, ptr %malloc_call, i32 19
  store i8 116, ptr %char_ptr_19, align 1
  %char_ptr_20 = getelementptr i8, ptr %malloc_call, i32 20
  store i8 105, ptr %char_ptr_20, align 1
  %char_ptr_21 = getelementptr i8, ptr %malloc_call, i32 21
  store i8 111, ptr %char_ptr_21, align 1
  %char_ptr_22 = getelementptr i8, ptr %malloc_call, i32 22
  store i8 110, ptr %char_ptr_22, align 1
  %char_ptr_23 = getelementptr i8, ptr %malloc_call, i32 23
  store i8 58, ptr %char_ptr_23, align 1
  %char_ptr_24 = getelementptr i8, ptr %malloc_call, i32 24
  store i8 32, ptr %char_ptr_24, align 1
  %null_ptr = getelementptr i8, ptr %malloc_call, i32 25
  store i8 0, ptr %null_ptr, align 1
  %printf_call_2 = call i32 (ptr, ...) @printf(ptr @str_0, ptr %malloc_call)
  %call_2 = call i64 @hello()
  %printf_call_21 = call i32 (ptr, ...) @printf(ptr @str_2, i64 %call_2)
  %printf_call_3 = call i32 (ptr, ...) @printf(ptr @str_1)
  %malloc_call2 = call ptr @malloc(i64 17)
  %char_ptr_03 = getelementptr i8, ptr %malloc_call2, i32 0
  store i8 83, ptr %char_ptr_03, align 1
  %char_ptr_110 = getelementptr i8, ptr %malloc_call2, i32 1
  store i8 117, ptr %char_ptr_110, align 1
  %char_ptr_211 = getelementptr i8, ptr %malloc_call2, i32 2
  store i8 109, ptr %char_ptr_211, align 1
  %char_ptr_312 = getelementptr i8, ptr %malloc_call2, i32 3
  store i8 32, ptr %char_ptr_312, align 1
  %char_ptr_413 = getelementptr i8, ptr %malloc_call2, i32 4
  store i8 111, ptr %char_ptr_413, align 1
  %char_ptr_514 = getelementptr i8, ptr %malloc_call2, i32 5
  store i8 102, ptr %char_ptr_514, align 1
  %char_ptr_615 = getelementptr i8, ptr %malloc_call2, i32 6
  store i8 32, ptr %char_ptr_615, align 1
  %char_ptr_716 = getelementptr i8, ptr %malloc_call2, i32 7
  store i8 51, ptr %char_ptr_716, align 1
  %char_ptr_817 = getelementptr i8, ptr %malloc_call2, i32 8
  store i8 32, ptr %char_ptr_817, align 1
  %char_ptr_918 = getelementptr i8, ptr %malloc_call2, i32 9
  store i8 97, ptr %char_ptr_918, align 1
  %char_ptr_1019 = getelementptr i8, ptr %malloc_call2, i32 10
  store i8 110, ptr %char_ptr_1019, align 1
  %char_ptr_1120 = getelementptr i8, ptr %malloc_call2, i32 11
  store i8 100, ptr %char_ptr_1120, align 1
  %char_ptr_1221 = getelementptr i8, ptr %malloc_call2, i32 12
  store i8 32, ptr %char_ptr_1221, align 1
  %char_ptr_1322 = getelementptr i8, ptr %malloc_call2, i32 13
  store i8 53, ptr %char_ptr_1322, align 1
  %char_ptr_1423 = getelementptr i8, ptr %malloc_call2, i32 14
  store i8 58, ptr %char_ptr_1423, align 1
  %char_ptr_1524 = getelementptr i8, ptr %malloc_call2, i32 15
  store i8 32, ptr %char_ptr_1524, align 1
  %null_ptr25 = getelementptr i8, ptr %malloc_call2, i32 16
  store i8 0, ptr %null_ptr25, align 1
  %printf_call_326 = call i32 (ptr, ...) @printf(ptr @str_0, ptr %malloc_call2)
  %call_3 = call i64 @add(i64 3, i64 5)
  %printf_call_327 = call i32 (ptr, ...) @printf(ptr @str_2, i64 %call_3)
  %printf_call_328 = call i32 (ptr, ...) @printf(ptr @str_1)
  %call_329 = call i64 @get_greeting()
  store i64 %call_329, ptr %greeting, align 4
  %malloc_call30 = call ptr @malloc(i64 18)
  %char_ptr_031 = getelementptr i8, ptr %malloc_call30, i32 0
  store i8 84, ptr %char_ptr_031, align 1
  %char_ptr_132 = getelementptr i8, ptr %malloc_call30, i32 1
  store i8 104, ptr %char_ptr_132, align 1
  %char_ptr_233 = getelementptr i8, ptr %malloc_call30, i32 2
  store i8 101, ptr %char_ptr_233, align 1
  %char_ptr_334 = getelementptr i8, ptr %malloc_call30, i32 3
  store i8 32, ptr %char_ptr_334, align 1
  %char_ptr_435 = getelementptr i8, ptr %malloc_call30, i32 4
  store i8 103, ptr %char_ptr_435, align 1
  %char_ptr_536 = getelementptr i8, ptr %malloc_call30, i32 5
  store i8 114, ptr %char_ptr_536, align 1
  %char_ptr_637 = getelementptr i8, ptr %malloc_call30, i32 6
  store i8 101, ptr %char_ptr_637, align 1
  %char_ptr_738 = getelementptr i8, ptr %malloc_call30, i32 7
  store i8 101, ptr %char_ptr_738, align 1
  %char_ptr_839 = getelementptr i8, ptr %malloc_call30, i32 8
  store i8 116, ptr %char_ptr_839, align 1
  %char_ptr_940 = getelementptr i8, ptr %malloc_call30, i32 9
  store i8 105, ptr %char_ptr_940, align 1
  %char_ptr_1041 = getelementptr i8, ptr %malloc_call30, i32 10
  store i8 110, ptr %char_ptr_1041, align 1
  %char_ptr_1142 = getelementptr i8, ptr %malloc_call30, i32 11
  store i8 103, ptr %char_ptr_1142, align 1
  %char_ptr_1243 = getelementptr i8, ptr %malloc_call30, i32 12
  store i8 32, ptr %char_ptr_1243, align 1
  %char_ptr_1344 = getelementptr i8, ptr %malloc_call30, i32 13
  store i8 105, ptr %char_ptr_1344, align 1
  %char_ptr_1445 = getelementptr i8, ptr %malloc_call30, i32 14
  store i8 115, ptr %char_ptr_1445, align 1
  %char_ptr_1546 = getelementptr i8, ptr %malloc_call30, i32 15
  store i8 58, ptr %char_ptr_1546, align 1
  %char_ptr_1647 = getelementptr i8, ptr %malloc_call30, i32 16
  store i8 32, ptr %char_ptr_1647, align 1
  %null_ptr48 = getelementptr i8, ptr %malloc_call30, i32 17
  store i8 0, ptr %null_ptr48, align 1
  %printf_call_349 = call i32 (ptr, ...) @printf(ptr @str_0, ptr %malloc_call30)
  %greeting50 = load i64, ptr %greeting, align 4
  %printf_call_351 = call i32 (ptr, ...) @printf(ptr @str_2, i64 %greeting50)
  %printf_call_352 = call i32 (ptr, ...) @printf(ptr @str_1)
  %malloc_call53 = call ptr @malloc(i64 16)
  %char_ptr_054 = getelementptr i8, ptr %malloc_call53, i32 0
  store i8 70, ptr %char_ptr_054, align 1
  %char_ptr_155 = getelementptr i8, ptr %malloc_call53, i32 1
  store i8 105, ptr %char_ptr_155, align 1
  %char_ptr_256 = getelementptr i8, ptr %malloc_call53, i32 2
  store i8 98, ptr %char_ptr_256, align 1
  %char_ptr_357 = getelementptr i8, ptr %malloc_call53, i32 3
  store i8 111, ptr %char_ptr_357, align 1
  %char_ptr_458 = getelementptr i8, ptr %malloc_call53, i32 4
  store i8 110, ptr %char_ptr_458, align 1
  %char_ptr_559 = getelementptr i8, ptr %malloc_call53, i32 5
  store i8 97, ptr %char_ptr_559, align 1
  %char_ptr_660 = getelementptr i8, ptr %malloc_call53, i32 6
  store i8 99, ptr %char_ptr_660, align 1
  %char_ptr_761 = getelementptr i8, ptr %malloc_call53, i32 7
  store i8 99, ptr %char_ptr_761, align 1
  %char_ptr_862 = getelementptr i8, ptr %malloc_call53, i32 8
  store i8 105, ptr %char_ptr_862, align 1
  %char_ptr_963 = getelementptr i8, ptr %malloc_call53, i32 9
  store i8 40, ptr %char_ptr_963, align 1
  %char_ptr_1064 = getelementptr i8, ptr %malloc_call53, i32 10
  store i8 48, ptr %char_ptr_1064, align 1
  %char_ptr_1165 = getelementptr i8, ptr %malloc_call53, i32 11
  store i8 41, ptr %char_ptr_1165, align 1
  %char_ptr_1266 = getelementptr i8, ptr %malloc_call53, i32 12
  store i8 32, ptr %char_ptr_1266, align 1
  %char_ptr_1367 = getelementptr i8, ptr %malloc_call53, i32 13
  store i8 61, ptr %char_ptr_1367, align 1
  %char_ptr_1468 = getelementptr i8, ptr %malloc_call53, i32 14
  store i8 32, ptr %char_ptr_1468, align 1
  %null_ptr69 = getelementptr i8, ptr %malloc_call53, i32 15
  store i8 0, ptr %null_ptr69, align 1
  %printf_call_370 = call i32 (ptr, ...) @printf(ptr @str_0, ptr %malloc_call53)
  %call_371 = call i64 @fibonacci(i64 0)
  %printf_call_372 = call i32 (ptr, ...) @printf(ptr @str_2, i64 %call_371)
  %printf_call_373 = call i32 (ptr, ...) @printf(ptr @str_1)
  %malloc_call74 = call ptr @malloc(i64 16)
  %char_ptr_075 = getelementptr i8, ptr %malloc_call74, i32 0
  store i8 70, ptr %char_ptr_075, align 1
  %char_ptr_176 = getelementptr i8, ptr %malloc_call74, i32 1
  store i8 105, ptr %char_ptr_176, align 1
  %char_ptr_277 = getelementptr i8, ptr %malloc_call74, i32 2
  store i8 98, ptr %char_ptr_277, align 1
  %char_ptr_378 = getelementptr i8, ptr %malloc_call74, i32 3
  store i8 111, ptr %char_ptr_378, align 1
  %char_ptr_479 = getelementptr i8, ptr %malloc_call74, i32 4
  store i8 110, ptr %char_ptr_479, align 1
  %char_ptr_580 = getelementptr i8, ptr %malloc_call74, i32 5
  store i8 97, ptr %char_ptr_580, align 1
  %char_ptr_681 = getelementptr i8, ptr %malloc_call74, i32 6
  store i8 99, ptr %char_ptr_681, align 1
  %char_ptr_782 = getelementptr i8, ptr %malloc_call74, i32 7
  store i8 99, ptr %char_ptr_782, align 1
  %char_ptr_883 = getelementptr i8, ptr %malloc_call74, i32 8
  store i8 105, ptr %char_ptr_883, align 1
  %char_ptr_984 = getelementptr i8, ptr %malloc_call74, i32 9
  store i8 40, ptr %char_ptr_984, align 1
  %char_ptr_1085 = getelementptr i8, ptr %malloc_call74, i32 10
  store i8 49, ptr %char_ptr_1085, align 1
  %char_ptr_1186 = getelementptr i8, ptr %malloc_call74, i32 11
  store i8 41, ptr %char_ptr_1186, align 1
  %char_ptr_1287 = getelementptr i8, ptr %malloc_call74, i32 12
  store i8 32, ptr %char_ptr_1287, align 1
  %char_ptr_1388 = getelementptr i8, ptr %malloc_call74, i32 13
  store i8 61, ptr %char_ptr_1388, align 1
  %char_ptr_1489 = getelementptr i8, ptr %malloc_call74, i32 14
  store i8 32, ptr %char_ptr_1489, align 1
  %null_ptr90 = getelementptr i8, ptr %malloc_call74, i32 15
  store i8 0, ptr %null_ptr90, align 1
  %printf_call_391 = call i32 (ptr, ...) @printf(ptr @str_0, ptr %malloc_call74)
  %call_392 = call i64 @fibonacci(i64 1)
  %printf_call_393 = call i32 (ptr, ...) @printf(ptr @str_2, i64 %call_392)
  %printf_call_394 = call i32 (ptr, ...) @printf(ptr @str_1)
  %malloc_call95 = call ptr @malloc(i64 16)
  %char_ptr_096 = getelementptr i8, ptr %malloc_call95, i32 0
  store i8 70, ptr %char_ptr_096, align 1
  %char_ptr_197 = getelementptr i8, ptr %malloc_call95, i32 1
  store i8 105, ptr %char_ptr_197, align 1
  %char_ptr_298 = getelementptr i8, ptr %malloc_call95, i32 2
  store i8 98, ptr %char_ptr_298, align 1
  %char_ptr_399 = getelementptr i8, ptr %malloc_call95, i32 3
  store i8 111, ptr %char_ptr_399, align 1
  %char_ptr_4100 = getelementptr i8, ptr %malloc_call95, i32 4
  store i8 110, ptr %char_ptr_4100, align 1
  %char_ptr_5101 = getelementptr i8, ptr %malloc_call95, i32 5
  store i8 97, ptr %char_ptr_5101, align 1
  %char_ptr_6102 = getelementptr i8, ptr %malloc_call95, i32 6
  store i8 99, ptr %char_ptr_6102, align 1
  %char_ptr_7103 = getelementptr i8, ptr %malloc_call95, i32 7
  store i8 99, ptr %char_ptr_7103, align 1
  %char_ptr_8104 = getelementptr i8, ptr %malloc_call95, i32 8
  store i8 105, ptr %char_ptr_8104, align 1
  %char_ptr_9105 = getelementptr i8, ptr %malloc_call95, i32 9
  store i8 40, ptr %char_ptr_9105, align 1
  %char_ptr_10106 = getelementptr i8, ptr %malloc_call95, i32 10
  store i8 50, ptr %char_ptr_10106, align 1
  %char_ptr_11107 = getelementptr i8, ptr %malloc_call95, i32 11
  store i8 41, ptr %char_ptr_11107, align 1
  %char_ptr_12108 = getelementptr i8, ptr %malloc_call95, i32 12
  store i8 32, ptr %char_ptr_12108, align 1
  %char_ptr_13109 = getelementptr i8, ptr %malloc_call95, i32 13
  store i8 61, ptr %char_ptr_13109, align 1
  %char_ptr_14110 = getelementptr i8, ptr %malloc_call95, i32 14
  store i8 32, ptr %char_ptr_14110, align 1
  %null_ptr111 = getelementptr i8, ptr %malloc_call95, i32 15
  store i8 0, ptr %null_ptr111, align 1
  %printf_call_3112 = call i32 (ptr, ...) @printf(ptr @str_0, ptr %malloc_call95)
  %call_3113 = call i64 @fibonacci(i64 2)
  %printf_call_3114 = call i32 (ptr, ...) @printf(ptr @str_2, i64 %call_3113)
  %printf_call_3115 = call i32 (ptr, ...) @printf(ptr @str_1)
  %malloc_call116 = call ptr @malloc(i64 16)
  %char_ptr_0117 = getelementptr i8, ptr %malloc_call116, i32 0
  store i8 70, ptr %char_ptr_0117, align 1
  %char_ptr_1118 = getelementptr i8, ptr %malloc_call116, i32 1
  store i8 105, ptr %char_ptr_1118, align 1
  %char_ptr_2119 = getelementptr i8, ptr %malloc_call116, i32 2
  store i8 98, ptr %char_ptr_2119, align 1
  %char_ptr_3120 = getelementptr i8, ptr %malloc_call116, i32 3
  store i8 111, ptr %char_ptr_3120, align 1
  %char_ptr_4121 = getelementptr i8, ptr %malloc_call116, i32 4
  store i8 110, ptr %char_ptr_4121, align 1
  %char_ptr_5122 = getelementptr i8, ptr %malloc_call116, i32 5
  store i8 97, ptr %char_ptr_5122, align 1
  %char_ptr_6123 = getelementptr i8, ptr %malloc_call116, i32 6
  store i8 99, ptr %char_ptr_6123, align 1
  %char_ptr_7124 = getelementptr i8, ptr %malloc_call116, i32 7
  store i8 99, ptr %char_ptr_7124, align 1
  %char_ptr_8125 = getelementptr i8, ptr %malloc_call116, i32 8
  store i8 105, ptr %char_ptr_8125, align 1
  %char_ptr_9126 = getelementptr i8, ptr %malloc_call116, i32 9
  store i8 40, ptr %char_ptr_9126, align 1
  %char_ptr_10127 = getelementptr i8, ptr %malloc_call116, i32 10
  store i8 51, ptr %char_ptr_10127, align 1
  %char_ptr_11128 = getelementptr i8, ptr %malloc_call116, i32 11
  store i8 41, ptr %char_ptr_11128, align 1
  %char_ptr_12129 = getelementptr i8, ptr %malloc_call116, i32 12
  store i8 32, ptr %char_ptr_12129, align 1
  %char_ptr_13130 = getelementptr i8, ptr %malloc_call116, i32 13
  store i8 61, ptr %char_ptr_13130, align 1
  %char_ptr_14131 = getelementptr i8, ptr %malloc_call116, i32 14
  store i8 32, ptr %char_ptr_14131, align 1
  %null_ptr132 = getelementptr i8, ptr %malloc_call116, i32 15
  store i8 0, ptr %null_ptr132, align 1
  %printf_call_3133 = call i32 (ptr, ...) @printf(ptr @str_0, ptr %malloc_call116)
  %call_3134 = call i64 @fibonacci(i64 3)
  %printf_call_3135 = call i32 (ptr, ...) @printf(ptr @str_2, i64 %call_3134)
  %printf_call_3136 = call i32 (ptr, ...) @printf(ptr @str_1)
  %malloc_call137 = call ptr @malloc(i64 16)
  %char_ptr_0138 = getelementptr i8, ptr %malloc_call137, i32 0
  store i8 70, ptr %char_ptr_0138, align 1
  %char_ptr_1139 = getelementptr i8, ptr %malloc_call137, i32 1
  store i8 105, ptr %char_ptr_1139, align 1
  %char_ptr_2140 = getelementptr i8, ptr %malloc_call137, i32 2
  store i8 98, ptr %char_ptr_2140, align 1
  %char_ptr_3141 = getelementptr i8, ptr %malloc_call137, i32 3
  store i8 111, ptr %char_ptr_3141, align 1
  %char_ptr_4142 = getelementptr i8, ptr %malloc_call137, i32 4
  store i8 110, ptr %char_ptr_4142, align 1
  %char_ptr_5143 = getelementptr i8, ptr %malloc_call137, i32 5
  store i8 97, ptr %char_ptr_5143, align 1
  %char_ptr_6144 = getelementptr i8, ptr %malloc_call137, i32 6
  store i8 99, ptr %char_ptr_6144, align 1
  %char_ptr_7145 = getelementptr i8, ptr %malloc_call137, i32 7
  store i8 99, ptr %char_ptr_7145, align 1
  %char_ptr_8146 = getelementptr i8, ptr %malloc_call137, i32 8
  store i8 105, ptr %char_ptr_8146, align 1
  %char_ptr_9147 = getelementptr i8, ptr %malloc_call137, i32 9
  store i8 40, ptr %char_ptr_9147, align 1
  %char_ptr_10148 = getelementptr i8, ptr %malloc_call137, i32 10
  store i8 52, ptr %char_ptr_10148, align 1
  %char_ptr_11149 = getelementptr i8, ptr %malloc_call137, i32 11
  store i8 41, ptr %char_ptr_11149, align 1
  %char_ptr_12150 = getelementptr i8, ptr %malloc_call137, i32 12
  store i8 32, ptr %char_ptr_12150, align 1
  %char_ptr_13151 = getelementptr i8, ptr %malloc_call137, i32 13
  store i8 61, ptr %char_ptr_13151, align 1
  %char_ptr_14152 = getelementptr i8, ptr %malloc_call137, i32 14
  store i8 32, ptr %char_ptr_14152, align 1
  %null_ptr153 = getelementptr i8, ptr %malloc_call137, i32 15
  store i8 0, ptr %null_ptr153, align 1
  %printf_call_3154 = call i32 (ptr, ...) @printf(ptr @str_0, ptr %malloc_call137)
  %call_3155 = call i64 @fibonacci(i64 4)
  %printf_call_3156 = call i32 (ptr, ...) @printf(ptr @str_2, i64 %call_3155)
  %printf_call_3157 = call i32 (ptr, ...) @printf(ptr @str_1)
  %malloc_call158 = call ptr @malloc(i64 16)
  %char_ptr_0159 = getelementptr i8, ptr %malloc_call158, i32 0
  store i8 70, ptr %char_ptr_0159, align 1
  %char_ptr_1160 = getelementptr i8, ptr %malloc_call158, i32 1
  store i8 105, ptr %char_ptr_1160, align 1
  %char_ptr_2161 = getelementptr i8, ptr %malloc_call158, i32 2
  store i8 98, ptr %char_ptr_2161, align 1
  %char_ptr_3162 = getelementptr i8, ptr %malloc_call158, i32 3
  store i8 111, ptr %char_ptr_3162, align 1
  %char_ptr_4163 = getelementptr i8, ptr %malloc_call158, i32 4
  store i8 110, ptr %char_ptr_4163, align 1
  %char_ptr_5164 = getelementptr i8, ptr %malloc_call158, i32 5
  store i8 97, ptr %char_ptr_5164, align 1
  %char_ptr_6165 = getelementptr i8, ptr %malloc_call158, i32 6
  store i8 99, ptr %char_ptr_6165, align 1
  %char_ptr_7166 = getelementptr i8, ptr %malloc_call158, i32 7
  store i8 99, ptr %char_ptr_7166, align 1
  %char_ptr_8167 = getelementptr i8, ptr %malloc_call158, i32 8
  store i8 105, ptr %char_ptr_8167, align 1
  %char_ptr_9168 = getelementptr i8, ptr %malloc_call158, i32 9
  store i8 40, ptr %char_ptr_9168, align 1
  %char_ptr_10169 = getelementptr i8, ptr %malloc_call158, i32 10
  store i8 53, ptr %char_ptr_10169, align 1
  %char_ptr_11170 = getelementptr i8, ptr %malloc_call158, i32 11
  store i8 41, ptr %char_ptr_11170, align 1
  %char_ptr_12171 = getelementptr i8, ptr %malloc_call158, i32 12
  store i8 32, ptr %char_ptr_12171, align 1
  %char_ptr_13172 = getelementptr i8, ptr %malloc_call158, i32 13
  store i8 61, ptr %char_ptr_13172, align 1
  %char_ptr_14173 = getelementptr i8, ptr %malloc_call158, i32 14
  store i8 32, ptr %char_ptr_14173, align 1
  %null_ptr174 = getelementptr i8, ptr %malloc_call158, i32 15
  store i8 0, ptr %null_ptr174, align 1
  %printf_call_3175 = call i32 (ptr, ...) @printf(ptr @str_0, ptr %malloc_call158)
  %call_3176 = call i64 @fibonacci(i64 5)
  %printf_call_3177 = call i32 (ptr, ...) @printf(ptr @str_2, i64 %call_3176)
  %printf_call_3178 = call i32 (ptr, ...) @printf(ptr @str_1)
  %malloc_call179 = call ptr @malloc(i64 17)
  %char_ptr_0180 = getelementptr i8, ptr %malloc_call179, i32 0
  store i8 70, ptr %char_ptr_0180, align 1
  %char_ptr_1181 = getelementptr i8, ptr %malloc_call179, i32 1
  store i8 105, ptr %char_ptr_1181, align 1
  %char_ptr_2182 = getelementptr i8, ptr %malloc_call179, i32 2
  store i8 98, ptr %char_ptr_2182, align 1
  %char_ptr_3183 = getelementptr i8, ptr %malloc_call179, i32 3
  store i8 111, ptr %char_ptr_3183, align 1
  %char_ptr_4184 = getelementptr i8, ptr %malloc_call179, i32 4
  store i8 110, ptr %char_ptr_4184, align 1
  %char_ptr_5185 = getelementptr i8, ptr %malloc_call179, i32 5
  store i8 97, ptr %char_ptr_5185, align 1
  %char_ptr_6186 = getelementptr i8, ptr %malloc_call179, i32 6
  store i8 99, ptr %char_ptr_6186, align 1
  %char_ptr_7187 = getelementptr i8, ptr %malloc_call179, i32 7
  store i8 99, ptr %char_ptr_7187, align 1
  %char_ptr_8188 = getelementptr i8, ptr %malloc_call179, i32 8
  store i8 105, ptr %char_ptr_8188, align 1
  %char_ptr_9189 = getelementptr i8, ptr %malloc_call179, i32 9
  store i8 40, ptr %char_ptr_9189, align 1
  %char_ptr_10190 = getelementptr i8, ptr %malloc_call179, i32 10
  store i8 49, ptr %char_ptr_10190, align 1
  %char_ptr_11191 = getelementptr i8, ptr %malloc_call179, i32 11
  store i8 48, ptr %char_ptr_11191, align 1
  %char_ptr_12192 = getelementptr i8, ptr %malloc_call179, i32 12
  store i8 41, ptr %char_ptr_12192, align 1
  %char_ptr_13193 = getelementptr i8, ptr %malloc_call179, i32 13
  store i8 32, ptr %char_ptr_13193, align 1
  %char_ptr_14194 = getelementptr i8, ptr %malloc_call179, i32 14
  store i8 61, ptr %char_ptr_14194, align 1
  %char_ptr_15195 = getelementptr i8, ptr %malloc_call179, i32 15
  store i8 32, ptr %char_ptr_15195, align 1
  %null_ptr196 = getelementptr i8, ptr %malloc_call179, i32 16
  store i8 0, ptr %null_ptr196, align 1
  %printf_call_3197 = call i32 (ptr, ...) @printf(ptr @str_0, ptr %malloc_call179)
  %call_3198 = call i64 @fibonacci(i64 10)
  %printf_call_3199 = call i32 (ptr, ...) @printf(ptr @str_2, i64 %call_3198)
  %printf_call_3200 = call i32 (ptr, ...) @printf(ptr @str_1)
  ret i64 0
}

define i32 @main.1() {
entry:
  %main_call = call i64 @main()
  %is_ok = icmp eq i64 %main_call, 1
  br i1 %is_ok, label %ok_exit, label %error_exit

ok_exit:                                          ; preds = %entry
  ret i32 0

error_exit:                                       ; preds = %entry
  ret i32 1
}
