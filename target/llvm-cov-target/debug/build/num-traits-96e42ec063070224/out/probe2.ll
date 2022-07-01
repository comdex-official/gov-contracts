; ModuleID = 'probe2.7e2101f7-cgu.0'
source_filename = "probe2.7e2101f7-cgu.0"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx11.0.0"

@__covrec_12423233AC2D4D79u = linkonce_odr hidden constant <{ i64, i32, i64, i64, [9 x i8] }> <{ i64 1315669238658977145, i32 9, i64 -6112113144305393894, i64 -3421096316512267351, [9 x i8] c"\01\01\00\01\01\01\01\00F" }>, section "__LLVM_COV,__llvm_covfun", align 8
@__llvm_coverage_mapping = private constant { { i32, i32, i32, i32 }, [90 x i8] } { { i32, i32, i32, i32 } { i32 0, i32 90, i32 0, i32 5 }, [90 x i8] c"\02W\00O/Users/subham/.cargo/registry/src/github.com-1ecc6299db9ec823/num-traits-0.2.15\06<anon>" }, section "__LLVM_COV,__llvm_covmap", align 8
@__llvm_profile_runtime = external global i32
@__profc__RNvCsoUZuEGWCPP_6probe25probe = private global [2 x i64] zeroinitializer, section "__DATA,__llvm_prf_cnts", align 8
@__profd__RNvCsoUZuEGWCPP_6probe25probe = private global { i64, i64, i64, i8*, i8*, i32, [2 x i16] } { i64 1315669238658977145, i64 -6112113144305393894, i64 sub (i64 ptrtoint ([2 x i64]* @__profc__RNvCsoUZuEGWCPP_6probe25probe to i64), i64 ptrtoint ({ i64, i64, i64, i8*, i8*, i32, [2 x i16] }* @__profd__RNvCsoUZuEGWCPP_6probe25probe to i64)), i8* null, i8* null, i32 2, [2 x i16] zeroinitializer }, section "__DATA,__llvm_prf_data,regular,live_support", align 8
@__llvm_prf_nm = private constant [32 x i8] c"\1E\00_RNvCsoUZuEGWCPP_6probe25probe", section "__DATA,__llvm_prf_names", align 1
@llvm.compiler.used = appending global [2 x i8*] [i8* bitcast (i32 ()* @__llvm_profile_runtime_user to i8*), i8* bitcast ({ i64, i64, i64, i8*, i8*, i32, [2 x i16] }* @__profd__RNvCsoUZuEGWCPP_6probe25probe to i8*)], section "llvm.metadata"
@llvm.used = appending global [3 x i8*] [i8* bitcast (<{ i64, i32, i64, i64, [9 x i8] }>* @__covrec_12423233AC2D4D79u to i8*), i8* bitcast ({ { i32, i32, i32, i32 }, [90 x i8] }* @__llvm_coverage_mapping to i8*), i8* getelementptr inbounds ([32 x i8], [32 x i8]* @__llvm_prf_nm, i32 0, i32 0)], section "llvm.metadata"

; <f64>::to_int_unchecked::<i32>
; Function Attrs: inlinehint uwtable
define i32 @_RINvMNtCs9RxZUtHUtzz_4core3f64d16to_int_uncheckedlECsoUZuEGWCPP_6probe2(double %self) unnamed_addr #0 {
start:
; call <f64 as core::convert::num::FloatToInt<i32>>::to_int_unchecked
  %0 = call i32 @_RNvXsk_NtNtCs9RxZUtHUtzz_4core7convert3numdINtB5_10FloatToIntlE16to_int_uncheckedCsoUZuEGWCPP_6probe2(double %self)
  br label %bb1

bb1:                                              ; preds = %start
  ret i32 %0
}

; <f64 as core::convert::num::FloatToInt<i32>>::to_int_unchecked
; Function Attrs: inlinehint uwtable
define internal i32 @_RNvXsk_NtNtCs9RxZUtHUtzz_4core7convert3numdINtB5_10FloatToIntlE16to_int_uncheckedCsoUZuEGWCPP_6probe2(double %self) unnamed_addr #0 {
start:
  %0 = alloca i32, align 4
  %1 = fptosi double %self to i32
  store i32 %1, i32* %0, align 4
  %2 = load i32, i32* %0, align 4
  br label %bb1

bb1:                                              ; preds = %start
  ret i32 %2
}

; probe2::probe
; Function Attrs: uwtable
define void @_RNvCsoUZuEGWCPP_6probe25probe() unnamed_addr #1 {
start:
  %pgocount = load i64, i64* getelementptr inbounds ([2 x i64], [2 x i64]* @__profc__RNvCsoUZuEGWCPP_6probe25probe, i32 0, i32 0), align 8
  %0 = add i64 %pgocount, 1
  store i64 %0, i64* getelementptr inbounds ([2 x i64], [2 x i64]* @__profc__RNvCsoUZuEGWCPP_6probe25probe, i32 0, i32 0), align 8
; call <f64>::to_int_unchecked::<i32>
  %_1 = call i32 @_RINvMNtCs9RxZUtHUtzz_4core3f64d16to_int_uncheckedlECsoUZuEGWCPP_6probe2(double 1.000000e+00)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; Function Attrs: nounwind
declare void @llvm.instrprof.increment(i8*, i64, i32, i32) #2

; Function Attrs: noinline
define linkonce_odr hidden i32 @__llvm_profile_runtime_user() #3 {
  %1 = load i32, i32* @__llvm_profile_runtime, align 4
  ret i32 %1
}

attributes #0 = { inlinehint uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }
attributes #1 = { uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }
attributes #2 = { nounwind }
attributes #3 = { noinline }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}
