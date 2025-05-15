; ModuleID = 'manual_tailcall'
source_filename = "manual_tailcall"

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
  %ge = icmp sge i64 %n1, 1
  %zext = zext i1 %ge to i64
  %ifcond = icmp ne i64 %zext, 0
  br i1 %ifcond, label %then, label %else

then:                                             ; preds = %entry
  %n2 = load i64, ptr %n, align 4
  store i64 %n2, ptr %result, align 4
  br label %ifcont

else:                                             ; preds = %entry
  br label %ifcont

ifcont:                                           ; preds = %else, %then
  %n6 = load i64, ptr %n, align 4
  %ge7 = icmp sge i64 %n6, 2
  %zext8 = zext i1 %ge7 to i64
  %ifcond9 = icmp ne i64 %zext8, 0
  br i1 %ifcond9, label %then3, label %else4

then3:                                            ; preds = %ifcont
  %result10 = load i64, ptr %result, align 4
  %n11 = load i64, ptr %n, align 4
  %sub = sub i64 %n11, 1
  %mul = mul i64 %result10, %sub
  store i64 %mul, ptr %result, align 4
  br label %ifcont5

else4:                                            ; preds = %ifcont
  br label %ifcont5

ifcont5:                                          ; preds = %else4, %then3
  %n15 = load i64, ptr %n, align 4
  %ge16 = icmp sge i64 %n15, 3
  %zext17 = zext i1 %ge16 to i64
  %ifcond18 = icmp ne i64 %zext17, 0
  br i1 %ifcond18, label %then12, label %else13

then12:                                           ; preds = %ifcont5
  %result19 = load i64, ptr %result, align 4
  %n20 = load i64, ptr %n, align 4
  %sub21 = sub i64 %n20, 2
  %mul22 = mul i64 %result19, %sub21
  store i64 %mul22, ptr %result, align 4
  br label %ifcont14

else13:                                           ; preds = %ifcont5
  br label %ifcont14

ifcont14:                                         ; preds = %else13, %then12
  %n26 = load i64, ptr %n, align 4
  %ge27 = icmp sge i64 %n26, 4
  %zext28 = zext i1 %ge27 to i64
  %ifcond29 = icmp ne i64 %zext28, 0
  br i1 %ifcond29, label %then23, label %else24

then23:                                           ; preds = %ifcont14
  %result30 = load i64, ptr %result, align 4
  %n31 = load i64, ptr %n, align 4
  %sub32 = sub i64 %n31, 3
  %mul33 = mul i64 %result30, %sub32
  store i64 %mul33, ptr %result, align 4
  br label %ifcont25

else24:                                           ; preds = %ifcont14
  br label %ifcont25

ifcont25:                                         ; preds = %else24, %then23
  %n37 = load i64, ptr %n, align 4
  %ge38 = icmp sge i64 %n37, 5
  %zext39 = zext i1 %ge38 to i64
  %ifcond40 = icmp ne i64 %zext39, 0
  br i1 %ifcond40, label %then34, label %else35

then34:                                           ; preds = %ifcont25
  %result41 = load i64, ptr %result, align 4
  %n42 = load i64, ptr %n, align 4
  %sub43 = sub i64 %n42, 4
  %mul44 = mul i64 %result41, %sub43
  store i64 %mul44, ptr %result, align 4
  br label %ifcont36

else35:                                           ; preds = %ifcont25
  br label %ifcont36

ifcont36:                                         ; preds = %else35, %then34
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
  %n45 = load i64, ptr %n, align 4
  %printf_call_1 = call i32 (ptr, ...) @printf(ptr @str_1, i64 %n45)
  %malloc_call46 = call ptr @malloc(i64 6)
  %char_ptr_047 = getelementptr i8, ptr %malloc_call46, i32 0
  store i8 32, ptr %char_ptr_047, align 1
  %char_ptr_148 = getelementptr i8, ptr %malloc_call46, i32 1
  store i8 105, ptr %char_ptr_148, align 1
  %char_ptr_249 = getelementptr i8, ptr %malloc_call46, i32 2
  store i8 115, ptr %char_ptr_249, align 1
  %char_ptr_350 = getelementptr i8, ptr %malloc_call46, i32 3
  store i8 58, ptr %char_ptr_350, align 1
  %char_ptr_451 = getelementptr i8, ptr %malloc_call46, i32 4
  store i8 32, ptr %char_ptr_451, align 1
  %null_ptr52 = getelementptr i8, ptr %malloc_call46, i32 5
  store i8 0, ptr %null_ptr52, align 1
  %printf_call_2 = call i32 (ptr, ...) @printf(ptr @str_0, ptr %malloc_call46)
  %result53 = load i64, ptr %result, align 4
  %printf_call_254 = call i32 (ptr, ...) @printf(ptr @str_1, i64 %result53)
  %printf_call_3 = call i32 (ptr, ...) @printf(ptr @str_2)
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
