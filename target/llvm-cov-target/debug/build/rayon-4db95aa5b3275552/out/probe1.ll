; ModuleID = 'probe1.e5d11323-cgu.0'
source_filename = "probe1.e5d11323-cgu.0"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx11.0.0"

%"core::iter::adapters::rev::Rev<core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>>" = type { %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>" }
%"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>" = type { i64, { i32, i32 }, i8, [7 x i8] }
%"core::panic::location::Location" = type { { [0 x i8]*, i64 }, i32, i32 }
%"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [2 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

@alloc20 = private unnamed_addr constant <{ [27 x i8] }> <{ [27 x i8] c"assertion failed: step != 0" }>, align 1
@alloc21 = private unnamed_addr constant <{ [89 x i8] }> <{ [89 x i8] c"/rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/core/src/iter/adapters/step_by.rs" }>, align 1
@alloc22 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [89 x i8] }>, <{ [89 x i8] }>* @alloc21, i32 0, i32 0, i32 0), [16 x i8] c"Y\00\00\00\00\00\00\00\15\00\00\00\09\00\00\00" }>, align 8
@__covrec_D023ABF014852D9Cu = linkonce_odr hidden constant <{ i64, i32, i64, i64, [9 x i8] }> <{ i64 -3448723842010894948, i32 9, i64 -301039726912181883, i64 967489048203684622, [9 x i8] c"\01\01\00\01\01\01\01\005" }>, section "__LLVM_COV,__llvm_covfun", align 8
@__llvm_coverage_mapping = private constant { { i32, i32, i32, i32 }, [84 x i8] } { { i32, i32, i32, i32 } { i32 0, i32 84, i32 0, i32 5 }, [84 x i8] c"\02Q\00I/Users/subham/.cargo/registry/src/github.com-1ecc6299db9ec823/rayon-1.5.3\06<anon>" }, section "__LLVM_COV,__llvm_covmap", align 8
@__llvm_profile_runtime = external global i32
@__profc__RNvCs1hfoinfez1T_6probe15probe = private global [2 x i64] zeroinitializer, section "__DATA,__llvm_prf_cnts", align 8
@__profd__RNvCs1hfoinfez1T_6probe15probe = private global { i64, i64, i64, i8*, i8*, i32, [2 x i16] } { i64 -3448723842010894948, i64 -301039726912181883, i64 sub (i64 ptrtoint ([2 x i64]* @__profc__RNvCs1hfoinfez1T_6probe15probe to i64), i64 ptrtoint ({ i64, i64, i64, i8*, i8*, i32, [2 x i16] }* @__profd__RNvCs1hfoinfez1T_6probe15probe to i64)), i8* null, i8* null, i32 2, [2 x i16] zeroinitializer }, section "__DATA,__llvm_prf_data,regular,live_support", align 8
@__llvm_prf_nm = private constant [33 x i8] c"\1F\00_RNvCs1hfoinfez1T_6probe15probe", section "__DATA,__llvm_prf_names", align 1
@llvm.compiler.used = appending global [2 x i8*] [i8* bitcast (i32 ()* @__llvm_profile_runtime_user to i8*), i8* bitcast ({ i64, i64, i64, i8*, i8*, i32, [2 x i16] }* @__profd__RNvCs1hfoinfez1T_6probe15probe to i8*)], section "llvm.metadata"
@llvm.used = appending global [3 x i8*] [i8* bitcast (<{ i64, i32, i64, i64, [9 x i8] }>* @__covrec_D023ABF014852D9Cu to i8*), i8* bitcast ({ { i32, i32, i32, i32 }, [84 x i8] }* @__llvm_coverage_mapping to i8*), i8* getelementptr inbounds ([33 x i8], [33 x i8]* @__llvm_prf_nm, i32 0, i32 0)], section "llvm.metadata"

; <core::iter::adapters::rev::Rev<core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>>>::new
; Function Attrs: uwtable
define void @_RNvMNtNtNtCs9RxZUtHUtzz_4core4iter8adapters3revINtB2_3RevINtNtB4_7step_by6StepByINtNtNtB8_3ops5range5RangelEEE3newCs1hfoinfez1T_6probe1(%"core::iter::adapters::rev::Rev<core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>>"* sret(%"core::iter::adapters::rev::Rev<core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>>") %0, %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>"* %iter) unnamed_addr #0 {
start:
  %_2 = alloca %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>", align 8
  %1 = bitcast %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>"* %_2 to i8*
  %2 = bitcast %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>"* %iter to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %1, i8* align 8 %2, i64 24, i1 false)
  %3 = bitcast %"core::iter::adapters::rev::Rev<core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>>"* %0 to %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>"*
  %4 = bitcast %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>"* %3 to i8*
  %5 = bitcast %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>"* %_2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %4, i8* align 8 %5, i64 24, i1 false)
  ret void
}

; <core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>>::new
; Function Attrs: uwtable
define void @_RNvMNtNtNtCs9RxZUtHUtzz_4core4iter8adapters7step_byINtB2_6StepByINtNtNtB8_3ops5range5RangelEE3newCs1hfoinfez1T_6probe1(%"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>"* sret(%"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>") %0, i32 %iter.0, i32 %iter.1, i64 %step) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %1 = alloca { i8*, i32 }, align 8
  %_4 = icmp ne i64 %step, 0
  %_3 = xor i1 %_4, true
  br i1 %_3, label %bb1, label %bb2

bb2:                                              ; preds = %start
  %_7 = sub i64 %step, 1
  %2 = getelementptr inbounds %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>", %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>"* %0, i32 0, i32 1
  %3 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %2, i32 0, i32 0
  store i32 %iter.0, i32* %3, align 8
  %4 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %2, i32 0, i32 1
  store i32 %iter.1, i32* %4, align 4
  %5 = bitcast %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>"* %0 to i64*
  store i64 %_7, i64* %5, align 8
  %6 = getelementptr inbounds %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>", %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>"* %0, i32 0, i32 2
  store i8 1, i8* %6, align 8
  ret void

bb1:                                              ; preds = %start
; invoke core::panicking::panic
  invoke void @_ZN4core9panicking5panic17ha94a579c0309d35aE([0 x i8]* align 1 bitcast (<{ [27 x i8] }>* @alloc20 to [0 x i8]*), i64 27, %"core::panic::location::Location"* align 8 bitcast (<{ i8*, [16 x i8] }>* @alloc22 to %"core::panic::location::Location"*)) #6
          to label %unreachable unwind label %cleanup

bb3:                                              ; preds = %cleanup
  br label %bb4

cleanup:                                          ; preds = %bb1
  %7 = landingpad { i8*, i32 }
          cleanup
  %8 = extractvalue { i8*, i32 } %7, 0
  %9 = extractvalue { i8*, i32 } %7, 1
  %10 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 0
  store i8* %8, i8** %10, align 8
  %11 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
  store i32 %9, i32* %11, align 8
  br label %bb3

unreachable:                                      ; preds = %bb1
  unreachable

bb4:                                              ; preds = %bb3
  %12 = bitcast { i8*, i32 }* %1 to i8**
  %13 = load i8*, i8** %12, align 8
  %14 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
  %15 = load i32, i32* %14, align 8
  %16 = insertvalue { i8*, i32 } undef, i8* %13, 0
  %17 = insertvalue { i8*, i32 } %16, i32 %15, 1
  resume { i8*, i32 } %17
}

; <core::ops::range::Range<i32> as core::iter::traits::iterator::Iterator>::step_by
; Function Attrs: inlinehint uwtable
define void @_RNvYINtNtNtCs9RxZUtHUtzz_4core3ops5range5RangelENtNtNtNtB9_4iter6traits8iterator8Iterator7step_byCs1hfoinfez1T_6probe1(%"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>"* sret(%"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>") %0, i32 %self.0, i32 %self.1, i64 %step) unnamed_addr #1 {
start:
; call <core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>>::new
  call void @_RNvMNtNtNtCs9RxZUtHUtzz_4core4iter8adapters7step_byINtB2_6StepByINtNtNtB8_3ops5range5RangelEE3newCs1hfoinfez1T_6probe1(%"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>"* sret(%"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>") %0, i32 %self.0, i32 %self.1, i64 %step)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; <core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>> as core::iter::traits::iterator::Iterator>::rev
; Function Attrs: inlinehint uwtable
define void @_RNvYINtNtNtNtCs9RxZUtHUtzz_4core4iter8adapters7step_by6StepByINtNtNtBb_3ops5range5RangelEENtNtNtB9_6traits8iterator8Iterator3revCs1hfoinfez1T_6probe1(%"core::iter::adapters::rev::Rev<core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>>"* sret(%"core::iter::adapters::rev::Rev<core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>>") %0, %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>"* %self) unnamed_addr #1 {
start:
  %_2 = alloca %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>", align 8
  %1 = bitcast %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>"* %_2 to i8*
  %2 = bitcast %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>"* %self to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %1, i8* align 8 %2, i64 24, i1 false)
; call <core::iter::adapters::rev::Rev<core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>>>::new
  call void @_RNvMNtNtNtCs9RxZUtHUtzz_4core4iter8adapters3revINtB2_3RevINtNtB4_7step_by6StepByINtNtNtB8_3ops5range5RangelEEE3newCs1hfoinfez1T_6probe1(%"core::iter::adapters::rev::Rev<core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>>"* sret(%"core::iter::adapters::rev::Rev<core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>>") %0, %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>"* %_2)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; probe1::probe
; Function Attrs: uwtable
define void @_RNvCs1hfoinfez1T_6probe15probe() unnamed_addr #0 {
start:
  %_3 = alloca { i32, i32 }, align 4
  %_2 = alloca %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>", align 8
  %_1 = alloca %"core::iter::adapters::rev::Rev<core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>>", align 8
  %pgocount = load i64, i64* getelementptr inbounds ([2 x i64], [2 x i64]* @__profc__RNvCs1hfoinfez1T_6probe15probe, i32 0, i32 0), align 8
  %0 = add i64 %pgocount, 1
  store i64 %0, i64* getelementptr inbounds ([2 x i64], [2 x i64]* @__profc__RNvCs1hfoinfez1T_6probe15probe, i32 0, i32 0), align 8
  %1 = bitcast { i32, i32 }* %_3 to i32*
  store i32 0, i32* %1, align 4
  %2 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %_3, i32 0, i32 1
  store i32 10, i32* %2, align 4
  %3 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %_3, i32 0, i32 0
  %4 = load i32, i32* %3, align 4
  %5 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %_3, i32 0, i32 1
  %6 = load i32, i32* %5, align 4
; call <core::ops::range::Range<i32> as core::iter::traits::iterator::Iterator>::step_by
  call void @_RNvYINtNtNtCs9RxZUtHUtzz_4core3ops5range5RangelENtNtNtNtB9_4iter6traits8iterator8Iterator7step_byCs1hfoinfez1T_6probe1(%"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>"* sret(%"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>") %_2, i32 %4, i32 %6, i64 2)
  br label %bb1

bb1:                                              ; preds = %start
; call <core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>> as core::iter::traits::iterator::Iterator>::rev
  call void @_RNvYINtNtNtNtCs9RxZUtHUtzz_4core4iter8adapters7step_by6StepByINtNtNtBb_3ops5range5RangelEENtNtNtB9_6traits8iterator8Iterator3revCs1hfoinfez1T_6probe1(%"core::iter::adapters::rev::Rev<core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>>"* sret(%"core::iter::adapters::rev::Rev<core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>>") %_1, %"core::iter::adapters::step_by::StepBy<core::ops::range::Range<i32>>"* %_2)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret void
}

; Function Attrs: argmemonly nofree nounwind willreturn
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #2

; Function Attrs: uwtable
declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #0

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17ha94a579c0309d35aE([0 x i8]* align 1, i64, %"core::panic::location::Location"* align 8) unnamed_addr #3

; Function Attrs: nounwind
declare void @llvm.instrprof.increment(i8*, i64, i32, i32) #4

; Function Attrs: noinline
define linkonce_odr hidden i32 @__llvm_profile_runtime_user() #5 {
  %1 = load i32, i32* @__llvm_profile_runtime, align 4
  ret i32 %1
}

attributes #0 = { uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }
attributes #1 = { inlinehint uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }
attributes #2 = { argmemonly nofree nounwind willreturn }
attributes #3 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }
attributes #4 = { nounwind }
attributes #5 = { noinline }
attributes #6 = { noreturn }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}
