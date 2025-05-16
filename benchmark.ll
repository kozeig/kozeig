; ModuleID = 'benchmark'
source_filename = "benchmark"

@str_0 = private constant [20 x i8] c"Factorial Benchmark\00"
@str_1 = private constant [3 x i8] c"%s\00"
@str_2 = private constant [3 x i8] c"\\n\00"
@str_3 = private constant [18 x i8] c"-----------------\00"
@str_4 = private constant [16 x i8] c"factorial(1) = \00"
@str_5 = private constant [5 x i8] c"%lld\00"
@str_6 = private constant [16 x i8] c"factorial(5) = \00"
@str_7 = private constant [17 x i8] c"factorial(10) = \00"
@str_8 = private constant [17 x i8] c"factorial(12) = \00"
@str_9 = private constant [17 x i8] c"factorial(15) = \00"
@str_10 = private constant [11 x i8] c"Completed \00"
@str_11 = private constant [31 x i8] c" calculations of factorial(20)\00"
@str_12 = private constant [17 x i8] c"Sum of results: \00"
@str_13 = private constant [3 x i8] c"ok\00"

declare i32 @printf(ptr, ...)

declare i32 @sprintf(ptr, ptr, ...)

declare i64 @atoll(ptr)

declare ptr @malloc(i64)

declare i64 @strlen(ptr)

define i64 @factorial(i64 %0) {
entry:
  %n.1 = alloca i64, align 8
  store i64 %0, ptr %n.1, align 4
  %n_int = load i64, ptr %n.1, align 4
  %le = icmp sle i64 %n_int, 1
  %zext = zext i1 %le to i64
  %ifcond = icmp ne i64 %zext, 0
  br i1 %ifcond, label %then, label %else

then:                                             ; preds = %entry
  br label %ifcont

else:                                             ; preds = %entry
  %n_int1 = load i64, ptr %n.1, align 4
  %n_int2 = load i64, ptr %n.1, align 4
  %sub = sub i64 %n_int2, 1
  %call_0 = call i64 @factorial(i64 %sub)
  %mul = mul i64 %n_int1, %call_0
  br label %ifcont

ifcont:                                           ; preds = %else, %then
  %ifresult = phi i64 [ 1, %then ], [ %mul, %else ]
  ret i64 %ifresult
}

define i64 @main() {
entry:
  %f20.4 = alloca i64, align 8
  %total.1 = alloca i64, align 8
  %target.1 = alloca i64, align 8
  %count.1 = alloca i64, align 8
  %malloc_call = call ptr @malloc(i64 20)
  call void @llvm.memcpy.p0.p0.i64(ptr %malloc_call, ptr @str_0, i64 20, i1 false)
  %printf_call_1 = call i32 (ptr, ...) @printf(ptr @str_1, ptr %malloc_call)
  %printf_call_3 = call i32 (ptr, ...) @printf(ptr @str_2)
  %malloc_call1 = call ptr @malloc(i64 18)
  call void @llvm.memcpy.p0.p0.i64(ptr %malloc_call1, ptr @str_3, i64 18, i1 false)
  %printf_call_4 = call i32 (ptr, ...) @printf(ptr @str_1, ptr %malloc_call1)
  %printf_call_42 = call i32 (ptr, ...) @printf(ptr @str_2)
  %malloc_call3 = call ptr @malloc(i64 16)
  call void @llvm.memcpy.p0.p0.i64(ptr %malloc_call3, ptr @str_4, i64 16, i1 false)
  %printf_call_5 = call i32 (ptr, ...) @printf(ptr @str_1, ptr %malloc_call3)
  %call_5 = call i64 @factorial(i64 1)
  %printf_call_54 = call i32 (ptr, ...) @printf(ptr @str_5, i64 %call_5)
  %printf_call_6 = call i32 (ptr, ...) @printf(ptr @str_2)
  %malloc_call5 = call ptr @malloc(i64 16)
  call void @llvm.memcpy.p0.p0.i64(ptr %malloc_call5, ptr @str_6, i64 16, i1 false)
  %printf_call_7 = call i32 (ptr, ...) @printf(ptr @str_1, ptr %malloc_call5)
  %call_7 = call i64 @factorial(i64 5)
  %printf_call_76 = call i32 (ptr, ...) @printf(ptr @str_5, i64 %call_7)
  %printf_call_77 = call i32 (ptr, ...) @printf(ptr @str_2)
  %malloc_call8 = call ptr @malloc(i64 17)
  call void @llvm.memcpy.p0.p0.i64(ptr %malloc_call8, ptr @str_7, i64 17, i1 false)
  %printf_call_8 = call i32 (ptr, ...) @printf(ptr @str_1, ptr %malloc_call8)
  %call_8 = call i64 @factorial(i64 10)
  %printf_call_89 = call i32 (ptr, ...) @printf(ptr @str_5, i64 %call_8)
  %printf_call_810 = call i32 (ptr, ...) @printf(ptr @str_2)
  %malloc_call11 = call ptr @malloc(i64 17)
  call void @llvm.memcpy.p0.p0.i64(ptr %malloc_call11, ptr @str_8, i64 17, i1 false)
  %printf_call_9 = call i32 (ptr, ...) @printf(ptr @str_1, ptr %malloc_call11)
  %call_9 = call i64 @factorial(i64 12)
  %printf_call_912 = call i32 (ptr, ...) @printf(ptr @str_5, i64 %call_9)
  %printf_call_913 = call i32 (ptr, ...) @printf(ptr @str_2)
  %malloc_call14 = call ptr @malloc(i64 17)
  call void @llvm.memcpy.p0.p0.i64(ptr %malloc_call14, ptr @str_9, i64 17, i1 false)
  %printf_call_10 = call i32 (ptr, ...) @printf(ptr @str_1, ptr %malloc_call14)
  %call_10 = call i64 @factorial(i64 15)
  %printf_call_1015 = call i32 (ptr, ...) @printf(ptr @str_5, i64 %call_10)
  %printf_call_1016 = call i32 (ptr, ...) @printf(ptr @str_2)
  %malloc_call17 = call ptr @malloc(i64 35)
  %char_ptr_0 = getelementptr i8, ptr %malloc_call17, i32 0
  store i8 92, ptr %char_ptr_0, align 1
  %char_ptr_1 = getelementptr i8, ptr %malloc_call17, i32 1
  store i8 110, ptr %char_ptr_1, align 1
  %char_ptr_2 = getelementptr i8, ptr %malloc_call17, i32 2
  store i8 82, ptr %char_ptr_2, align 1
  %char_ptr_3 = getelementptr i8, ptr %malloc_call17, i32 3
  store i8 117, ptr %char_ptr_3, align 1
  %char_ptr_4 = getelementptr i8, ptr %malloc_call17, i32 4
  store i8 110, ptr %char_ptr_4, align 1
  %char_ptr_5 = getelementptr i8, ptr %malloc_call17, i32 5
  store i8 110, ptr %char_ptr_5, align 1
  %char_ptr_6 = getelementptr i8, ptr %malloc_call17, i32 6
  store i8 105, ptr %char_ptr_6, align 1
  %char_ptr_7 = getelementptr i8, ptr %malloc_call17, i32 7
  store i8 110, ptr %char_ptr_7, align 1
  %char_ptr_8 = getelementptr i8, ptr %malloc_call17, i32 8
  store i8 103, ptr %char_ptr_8, align 1
  %char_ptr_9 = getelementptr i8, ptr %malloc_call17, i32 9
  store i8 32, ptr %char_ptr_9, align 1
  %char_ptr_10 = getelementptr i8, ptr %malloc_call17, i32 10
  store i8 105, ptr %char_ptr_10, align 1
  %char_ptr_11 = getelementptr i8, ptr %malloc_call17, i32 11
  store i8 110, ptr %char_ptr_11, align 1
  %char_ptr_12 = getelementptr i8, ptr %malloc_call17, i32 12
  store i8 116, ptr %char_ptr_12, align 1
  %char_ptr_13 = getelementptr i8, ptr %malloc_call17, i32 13
  store i8 101, ptr %char_ptr_13, align 1
  %char_ptr_14 = getelementptr i8, ptr %malloc_call17, i32 14
  store i8 110, ptr %char_ptr_14, align 1
  %char_ptr_15 = getelementptr i8, ptr %malloc_call17, i32 15
  store i8 115, ptr %char_ptr_15, align 1
  %char_ptr_16 = getelementptr i8, ptr %malloc_call17, i32 16
  store i8 105, ptr %char_ptr_16, align 1
  %char_ptr_17 = getelementptr i8, ptr %malloc_call17, i32 17
  store i8 118, ptr %char_ptr_17, align 1
  %char_ptr_18 = getelementptr i8, ptr %malloc_call17, i32 18
  store i8 101, ptr %char_ptr_18, align 1
  %char_ptr_19 = getelementptr i8, ptr %malloc_call17, i32 19
  store i8 32, ptr %char_ptr_19, align 1
  %char_ptr_20 = getelementptr i8, ptr %malloc_call17, i32 20
  store i8 99, ptr %char_ptr_20, align 1
  %char_ptr_21 = getelementptr i8, ptr %malloc_call17, i32 21
  store i8 97, ptr %char_ptr_21, align 1
  %char_ptr_22 = getelementptr i8, ptr %malloc_call17, i32 22
  store i8 108, ptr %char_ptr_22, align 1
  %char_ptr_23 = getelementptr i8, ptr %malloc_call17, i32 23
  store i8 99, ptr %char_ptr_23, align 1
  %char_ptr_24 = getelementptr i8, ptr %malloc_call17, i32 24
  store i8 117, ptr %char_ptr_24, align 1
  %char_ptr_25 = getelementptr i8, ptr %malloc_call17, i32 25
  store i8 108, ptr %char_ptr_25, align 1
  %char_ptr_26 = getelementptr i8, ptr %malloc_call17, i32 26
  store i8 97, ptr %char_ptr_26, align 1
  %char_ptr_27 = getelementptr i8, ptr %malloc_call17, i32 27
  store i8 116, ptr %char_ptr_27, align 1
  %char_ptr_28 = getelementptr i8, ptr %malloc_call17, i32 28
  store i8 105, ptr %char_ptr_28, align 1
  %char_ptr_29 = getelementptr i8, ptr %malloc_call17, i32 29
  store i8 111, ptr %char_ptr_29, align 1
  %char_ptr_30 = getelementptr i8, ptr %malloc_call17, i32 30
  store i8 110, ptr %char_ptr_30, align 1
  %char_ptr_31 = getelementptr i8, ptr %malloc_call17, i32 31
  store i8 46, ptr %char_ptr_31, align 1
  %char_ptr_32 = getelementptr i8, ptr %malloc_call17, i32 32
  store i8 46, ptr %char_ptr_32, align 1
  %char_ptr_33 = getelementptr i8, ptr %malloc_call17, i32 33
  store i8 46, ptr %char_ptr_33, align 1
  %null_ptr = getelementptr i8, ptr %malloc_call17, i32 34
  store i8 0, ptr %null_ptr, align 1
  %printf_call_1018 = call i32 (ptr, ...) @printf(ptr @str_1, ptr %malloc_call17)
  %printf_call_1019 = call i32 (ptr, ...) @printf(ptr @str_2)
  store i64 0, ptr %count.1, align 4
  store i64 500000, ptr %target.1, align 4
  store i64 0, ptr %total.1, align 4
  br label %while_cond

while_cond:                                       ; preds = %while_body, %entry
  %count_int = load i64, ptr %count.1, align 4
  %target_int = load i64, ptr %target.1, align 4
  %lt = icmp slt i64 %count_int, %target_int
  %zext = zext i1 %lt to i64
  %while_cond20 = icmp ne i64 %zext, 0
  br i1 %while_cond20, label %while_body, label %while_exit

while_body:                                       ; preds = %while_cond
  %call_1021 = call i64 @factorial(i64 20)
  store i64 %call_1021, ptr %f20.4, align 4
  %total_int = load i64, ptr %total.1, align 4
  %f20_int = load i64, ptr %f20.4, align 4
  %add = add i64 %total_int, %f20_int
  store i64 %add, ptr %total.1, align 4
  %count_int22 = load i64, ptr %count.1, align 4
  %add23 = add i64 %count_int22, 1
  store i64 %add23, ptr %count.1, align 4
  br label %while_cond

while_exit:                                       ; preds = %while_cond
  %malloc_call24 = call ptr @malloc(i64 11)
  call void @llvm.memcpy.p0.p0.i64(ptr %malloc_call24, ptr @str_10, i64 11, i1 false)
  %printf_call_11 = call i32 (ptr, ...) @printf(ptr @str_1, ptr %malloc_call24)
  %target_int25 = load i64, ptr %target.1, align 4
  %printf_call_1126 = call i32 (ptr, ...) @printf(ptr @str_5, i64 %target_int25)
  %malloc_call27 = call ptr @malloc(i64 31)
  call void @llvm.memcpy.p0.p0.i64(ptr %malloc_call27, ptr @str_11, i64 31, i1 false)
  %printf_call_12 = call i32 (ptr, ...) @printf(ptr @str_1, ptr %malloc_call27)
  %printf_call_1228 = call i32 (ptr, ...) @printf(ptr @str_2)
  %malloc_call29 = call ptr @malloc(i64 17)
  call void @llvm.memcpy.p0.p0.i64(ptr %malloc_call29, ptr @str_12, i64 17, i1 false)
  %printf_call_13 = call i32 (ptr, ...) @printf(ptr @str_1, ptr %malloc_call29)
  %total_int30 = load i64, ptr %total.1, align 4
  %printf_call_1331 = call i32 (ptr, ...) @printf(ptr @str_5, i64 %total_int30)
  %printf_call_1332 = call i32 (ptr, ...) @printf(ptr @str_2)
  %malloc_call33 = call ptr @malloc(i64 3)
  call void @llvm.memcpy.p0.p0.i64(ptr %malloc_call33, ptr @str_13, i64 3, i1 false)
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
