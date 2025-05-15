; ModuleID = 'simple_recursive'
source_filename = "simple_recursive"

@str_0 = private constant [3 x i8] c"%s\00"
@str_1 = private constant [5 x i8] c"%lld\00"
@str_2 = private constant [3 x i8] c"\\n\00"

declare i32 @printf(ptr, ...)

declare i32 @sprintf(ptr, ptr, ...)

declare i64 @atoll(ptr)

declare ptr @malloc(i64)

declare i64 @strlen(ptr)

define i64 @main() {
entry:
  %result = alloca i64, align 8
  %n = alloca i64, align 8
  store i64 5, ptr %n, align 4
  store i64 1, ptr %result, align 4
  %n1 = load i64, ptr %n, align 4
  %gt = icmp sgt i64 %n1, 1
  %zext = zext i1 %gt to i64
  %ifcond = icmp ne i64 %zext, 0
  br i1 %ifcond, label %then, label %else

then:                                             ; preds = %entry
  %n2 = load i64, ptr %n, align 4
  store i64 %n2, ptr %result, align 4
  %n6 = load i64, ptr %n, align 4
  %gt7 = icmp sgt i64 %n6, 1
  %zext8 = zext i1 %gt7 to i64
  %ifcond9 = icmp ne i64 %zext8, 0
  br i1 %ifcond9, label %then3, label %else4

else:                                             ; preds = %entry
  br label %ifcont

ifcont:                                           ; preds = %else, %ifcont5
  %malloc_call = call ptr @malloc(i64 14)
  %char_ptr_0 = getelementptr i8, ptr %malloc_call, i32 0
  store i8 70, ptr %char_ptr_0, align 1
  %char_ptr_1 = getelementptr i8, ptr %malloc_call, i32 1
  store i8 97, ptr %char_ptr_1, align 1
  %char_ptr_2 = getelementptr i8, ptr %malloc_call, i32 2
  store i8 99, ptr %char_ptr_2, align 1
  %char_ptr_3 = getelementptr i8, ptr %malloc_call, i32 3
  store i8 116, ptr %char_ptr_3, align 1
  %char_ptr_4 = getelementptr i8, ptr %malloc_call, i32 4
  store i8 111, ptr %char_ptr_4, align 1
  %char_ptr_5 = getelementptr i8, ptr %malloc_call, i32 5
  store i8 114, ptr %char_ptr_5, align 1
  %char_ptr_6 = getelementptr i8, ptr %malloc_call, i32 6
  store i8 105, ptr %char_ptr_6, align 1
  %char_ptr_7 = getelementptr i8, ptr %malloc_call, i32 7
  store i8 97, ptr %char_ptr_7, align 1
  %char_ptr_8 = getelementptr i8, ptr %malloc_call, i32 8
  store i8 108, ptr %char_ptr_8, align 1
  %char_ptr_9 = getelementptr i8, ptr %malloc_call, i32 9
  store i8 32, ptr %char_ptr_9, align 1
  %char_ptr_10 = getelementptr i8, ptr %malloc_call, i32 10
  store i8 111, ptr %char_ptr_10, align 1
  %char_ptr_11 = getelementptr i8, ptr %malloc_call, i32 11
  store i8 102, ptr %char_ptr_11, align 1
  %char_ptr_12 = getelementptr i8, ptr %malloc_call, i32 12
  store i8 32, ptr %char_ptr_12, align 1
  %null_ptr = getelementptr i8, ptr %malloc_call, i32 13
  store i8 0, ptr %null_ptr, align 1
  %printf_call_0 = call i32 (ptr, ...) @printf(ptr @str_0, ptr %malloc_call)
  %n48 = load i64, ptr %n, align 4
  %printf_call_1 = call i32 (ptr, ...) @printf(ptr @str_1, i64 %n48)
  %malloc_call49 = call ptr @malloc(i64 6)
  %char_ptr_050 = getelementptr i8, ptr %malloc_call49, i32 0
  store i8 32, ptr %char_ptr_050, align 1
  %char_ptr_151 = getelementptr i8, ptr %malloc_call49, i32 1
  store i8 105, ptr %char_ptr_151, align 1
  %char_ptr_252 = getelementptr i8, ptr %malloc_call49, i32 2
  store i8 115, ptr %char_ptr_252, align 1
  %char_ptr_353 = getelementptr i8, ptr %malloc_call49, i32 3
  store i8 58, ptr %char_ptr_353, align 1
  %char_ptr_454 = getelementptr i8, ptr %malloc_call49, i32 4
  store i8 32, ptr %char_ptr_454, align 1
  %null_ptr55 = getelementptr i8, ptr %malloc_call49, i32 5
  store i8 0, ptr %null_ptr55, align 1
  %printf_call_2 = call i32 (ptr, ...) @printf(ptr @str_0, ptr %malloc_call49)
  %result56 = load i64, ptr %result, align 4
  %printf_call_257 = call i32 (ptr, ...) @printf(ptr @str_1, i64 %result56)
  %printf_call_3 = call i32 (ptr, ...) @printf(ptr @str_2)
  ret i64 0

then3:                                            ; preds = %then
  %result10 = load i64, ptr %result, align 4
  %n11 = load i64, ptr %n, align 4
  %sub = sub i64 %n11, 1
  %mul = mul i64 %result10, %sub
  store i64 %mul, ptr %result, align 4
  %n15 = load i64, ptr %n, align 4
  %sub16 = sub i64 %n15, 1
  %gt17 = icmp sgt i64 %sub16, 1
  %zext18 = zext i1 %gt17 to i64
  %ifcond19 = icmp ne i64 %zext18, 0
  br i1 %ifcond19, label %then12, label %else13

else4:                                            ; preds = %then
  br label %ifcont5

ifcont5:                                          ; preds = %else4, %ifcont14
  br label %ifcont

then12:                                           ; preds = %then3
  %result20 = load i64, ptr %result, align 4
  %n21 = load i64, ptr %n, align 4
  %sub22 = sub i64 %n21, 2
  %mul23 = mul i64 %result20, %sub22
  store i64 %mul23, ptr %result, align 4
  %n27 = load i64, ptr %n, align 4
  %sub28 = sub i64 %n27, 2
  %gt29 = icmp sgt i64 %sub28, 1
  %zext30 = zext i1 %gt29 to i64
  %ifcond31 = icmp ne i64 %zext30, 0
  br i1 %ifcond31, label %then24, label %else25

else13:                                           ; preds = %then3
  br label %ifcont14

ifcont14:                                         ; preds = %else13, %ifcont26
  br label %ifcont5

then24:                                           ; preds = %then12
  %result32 = load i64, ptr %result, align 4
  %n33 = load i64, ptr %n, align 4
  %sub34 = sub i64 %n33, 3
  %mul35 = mul i64 %result32, %sub34
  store i64 %mul35, ptr %result, align 4
  %n39 = load i64, ptr %n, align 4
  %sub40 = sub i64 %n39, 3
  %gt41 = icmp sgt i64 %sub40, 1
  %zext42 = zext i1 %gt41 to i64
  %ifcond43 = icmp ne i64 %zext42, 0
  br i1 %ifcond43, label %then36, label %else37

else25:                                           ; preds = %then12
  br label %ifcont26

ifcont26:                                         ; preds = %else25, %ifcont38
  br label %ifcont14

then36:                                           ; preds = %then24
  %result44 = load i64, ptr %result, align 4
  %n45 = load i64, ptr %n, align 4
  %sub46 = sub i64 %n45, 4
  %mul47 = mul i64 %result44, %sub46
  store i64 %mul47, ptr %result, align 4
  br label %ifcont38

else37:                                           ; preds = %then24
  br label %ifcont38

ifcont38:                                         ; preds = %else37, %then36
  br label %ifcont26
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
