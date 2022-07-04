; ModuleID = 'probe5.5f75b08d-cgu.0'
source_filename = "probe5.5f75b08d-cgu.0"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx11.0.0"

%"core::panic::location::Location" = type { { [0 x i8]*, i64 }, i32, i32 }

@alloc3 = private unnamed_addr constant <{ [77 x i8] }> <{ [77 x i8] c"/rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/core/src/ops/arith.rs" }>, align 1
@alloc4 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [77 x i8] }>, <{ [77 x i8] }>* @alloc3, i32 0, i32 0, i32 0), [16 x i8] c"M\00\00\00\00\00\00\00\12\03\00\00\01\00\00\00" }>, align 8
@str.0 = internal constant [28 x i8] c"attempt to add with overflow"
@alloc2 = private unnamed_addr constant <{ [4 x i8] }> <{ [4 x i8] c"\02\00\00\00" }>, align 4
@__covrec_5FAD1E99F7968B02u = linkonce_odr hidden constant <{ i64, i32, i64, i64, [9 x i8] }> <{ i64 6894200251206765314, i32 9, i64 -1755868853587487849, i64 -3421096316512267351, [9 x i8] c"\01\01\00\01\01\01\01\008" }>, section "__LLVM_COV,__llvm_covfun", align 8
@__llvm_coverage_mapping = private constant { { i32, i32, i32, i32 }, [90 x i8] } { { i32, i32, i32, i32 } { i32 0, i32 90, i32 0, i32 5 }, [90 x i8] c"\02W\00O/Users/subham/.cargo/registry/src/github.com-1ecc6299db9ec823/num-traits-0.2.15\06<anon>" }, section "__LLVM_COV,__llvm_covmap", align 8
@__llvm_profile_runtime = external global i32
@__profc__RNvCs6iPIrE58Vo7_6probe55probe = private global [2 x i64] zeroinitializer, section "__DATA,__llvm_prf_cnts", align 8
@__profd__RNvCs6iPIrE58Vo7_6probe55probe = private global { i64, i64, i64, i8*, i8*, i32, [2 x i16] } { i64 6894200251206765314, i64 -1755868853587487849, i64 sub (i64 ptrtoint ([2 x i64]* @__profc__RNvCs6iPIrE58Vo7_6probe55probe to i64), i64 ptrtoint ({ i64, i64, i64, i8*, i8*, i32, [2 x i16] }* @__profd__RNvCs6iPIrE58Vo7_6probe55probe to i64)), i8* null, i8* null, i32 2, [2 x i16] zeroinitializer }, section "__DATA,__llvm_prf_data,regular,live_support", align 8
@__llvm_prf_nm = private constant [33 x i8] c"\1F\00_RNvCs6iPIrE58Vo7_6probe55probe", section "__DATA,__llvm_prf_names", align 1
@llvm.compiler.used = appending global [2 x i8*] [i8* bitcast (i32 ()* @__llvm_profile_runtime_user to i8*), i8* bitcast ({ i64, i64, i64, i8*, i8*, i32, [2 x i16] }* @__profd__RNvCs6iPIrE58Vo7_6probe55probe to i8*)], section "llvm.metadata"
@llvm.used = appending global [3 x i8*] [i8* bitcast (<{ i64, i32, i64, i64, [9 x i8] }>* @__covrec_5FAD1E99F7968B02u to i8*), i8* bitcast ({ { i32, i32, i32, i32 }, [90 x i8] }* @__llvm_coverage_mapping to i8*), i8* getelementptr inbounds ([33 x i8], [33 x i8]* @__llvm_prf_nm, i32 0, i32 0)], section "llvm.metadata"

; <i32 as core::ops::arith::AddAssign>::add_assign
; Function Attrs: inlinehint uwtable
define internal void @_RNvXs4T_NtNtCs9RxZUtHUtzz_4core3ops5arithlNtB6_9AddAssign10add_assignCs6iPIrE58Vo7_6probe5(i32* align 4 %self, i32 %other) unnamed_addr #0 {
start:
  %0 = load i32, i32* %self, align 4
  %1 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %0, i32 %other)
  %_4.0 = extractvalue { i32, i1 } %1, 0
  %_4.1 = extractvalue { i32, i1 } %1, 1
  %2 = call i1 @llvm.expect.i1(i1 %_4.1, i1 false)
  br i1 %2, label %panic, label %bb1

bb1:                                              ; preds = %start
  store i32 %_4.0, i32* %self, align 4
  ret void

panic:                                            ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17ha94a579c0309d35aE([0 x i8]* align 1 bitcast ([28 x i8]* @str.0 to [0 x i8]*), i64 28, %"core::panic::location::Location"* align 8 bitcast (<{ i8*, [16 x i8] }>* @alloc4 to %"core::panic::location::Location"*)) #7
  unreachable
}

; <i32 as core::ops::arith::AddAssign<&i32>>::add_assign
; Function Attrs: inlinehint uwtable
define internal void @_RNvXs57_NtNtCs9RxZUtHUtzz_4core3ops5arithlINtB6_9AddAssignRlE10add_assignCs6iPIrE58Vo7_6probe5(i32* align 4 %self, i32* align 4 %other) unnamed_addr #0 {
start:
  %_5 = load i32, i32* %other, align 4
; call <i32 as core::ops::arith::AddAssign>::add_assign
  call void @_RNvXs4T_NtNtCs9RxZUtHUtzz_4core3ops5arithlNtB6_9AddAssign10add_assignCs6iPIrE58Vo7_6probe5(i32* align 4 %self, i32 %_5)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; probe5::probe
; Function Attrs: uwtable
define void @_RNvCs6iPIrE58Vo7_6probe55probe() unnamed_addr #1 {
start:
  %x = alloca i32, align 4
  %pgocount = load i64, i64* getelementptr inbounds ([2 x i64], [2 x i64]* @__profc__RNvCs6iPIrE58Vo7_6probe55probe, i32 0, i32 0), align 8
  %0 = add i64 %pgocount, 1
  store i64 %0, i64* getelementptr inbounds ([2 x i64], [2 x i64]* @__profc__RNvCs6iPIrE58Vo7_6probe55probe, i32 0, i32 0), align 8
  store i32 1, i32* %x, align 4
; call <i32 as core::ops::arith::AddAssign<&i32>>::add_assign
  call void @_RNvXs57_NtNtCs9RxZUtHUtzz_4core3ops5arithlINtB6_9AddAssignRlE10add_assignCs6iPIrE58Vo7_6probe5(i32* align 4 %x, i32* align 4 bitcast (<{ [4 x i8] }>* @alloc2 to i32*))
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; Function Attrs: nofree nosync nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #2

; Function Attrs: nofree nosync nounwind readnone willreturn
declare i1 @llvm.expect.i1(i1, i1) #3

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17ha94a579c0309d35aE([0 x i8]* align 1, i64, %"core::panic::location::Location"* align 8) unnamed_addr #4

; Function Attrs: nounwind
declare void @llvm.instrprof.increment(i8*, i64, i32, i32) #5

; Function Attrs: noinline
define linkonce_odr hidden i32 @__llvm_profile_runtime_user() #6 {
  %1 = load i32, i32* @__llvm_profile_runtime, align 4
  ret i32 %1
}

attributes #0 = { inlinehint uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }
attributes #1 = { uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }
attributes #2 = { nofree nosync nounwind readnone speculatable willreturn }
attributes #3 = { nofree nosync nounwind readnone willreturn }
attributes #4 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }
attributes #5 = { nounwind }
attributes #6 = { noinline }
attributes #7 = { noreturn }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}
