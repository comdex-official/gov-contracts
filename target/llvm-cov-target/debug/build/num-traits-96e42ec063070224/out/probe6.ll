; ModuleID = 'probe6.2f02fb0d-cgu.0'
source_filename = "probe6.2f02fb0d-cgu.0"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx11.0.0"

%"core::panic::location::Location" = type { { [0 x i8]*, i64 }, i32, i32 }

@alloc1 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/fe5b13d681f25ee6474be29d748c65adcd91f69e/library/core/src/num/mod.rs" }>, align 1
@alloc2 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [75 x i8] }>, <{ [75 x i8] }>* @alloc1, i32 0, i32 0, i32 0), [16 x i8] c"K\00\00\00\00\00\00\00U\03\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"
@__covrec_1C71A8325059836Bu = linkonce_odr hidden constant <{ i64, i32, i64, i64, [9 x i8] }> <{ i64 2049604239480161131, i32 9, i64 7030929516476072503, i64 -3421096316512267351, [9 x i8] c"\01\01\00\01\01\01\01\002" }>, section "__LLVM_COV,__llvm_covfun", align 8
@__llvm_coverage_mapping = private constant { { i32, i32, i32, i32 }, [90 x i8] } { { i32, i32, i32, i32 } { i32 0, i32 90, i32 0, i32 5 }, [90 x i8] c"\02W\00O/Users/subham/.cargo/registry/src/github.com-1ecc6299db9ec823/num-traits-0.2.15\06<anon>" }, section "__LLVM_COV,__llvm_covmap", align 8
@__llvm_profile_runtime = external global i32
@__profc__RNvCs8ZkI4e3ZCwH_6probe65probe = private global [2 x i64] zeroinitializer, section "__DATA,__llvm_prf_cnts", align 8
@__profd__RNvCs8ZkI4e3ZCwH_6probe65probe = private global { i64, i64, i64, i8*, i8*, i32, [2 x i16] } { i64 2049604239480161131, i64 7030929516476072503, i64 sub (i64 ptrtoint ([2 x i64]* @__profc__RNvCs8ZkI4e3ZCwH_6probe65probe to i64), i64 ptrtoint ({ i64, i64, i64, i8*, i8*, i32, [2 x i16] }* @__profd__RNvCs8ZkI4e3ZCwH_6probe65probe to i64)), i8* null, i8* null, i32 2, [2 x i16] zeroinitializer }, section "__DATA,__llvm_prf_data,regular,live_support", align 8
@__llvm_prf_nm = private constant [33 x i8] c"\1F\00_RNvCs8ZkI4e3ZCwH_6probe65probe", section "__DATA,__llvm_prf_names", align 1
@llvm.compiler.used = appending global [2 x i8*] [i8* bitcast (i32 ()* @__llvm_profile_runtime_user to i8*), i8* bitcast ({ i64, i64, i64, i8*, i8*, i32, [2 x i16] }* @__profd__RNvCs8ZkI4e3ZCwH_6probe65probe to i8*)], section "llvm.metadata"
@llvm.used = appending global [3 x i8*] [i8* bitcast (<{ i64, i32, i64, i64, [9 x i8] }>* @__covrec_1C71A8325059836Bu to i8*), i8* bitcast ({ { i32, i32, i32, i32 }, [90 x i8] }* @__llvm_coverage_mapping to i8*), i8* getelementptr inbounds ([33 x i8], [33 x i8]* @__llvm_prf_nm, i32 0, i32 0)], section "llvm.metadata"

; probe6::probe
; Function Attrs: uwtable
define void @_RNvCs8ZkI4e3ZCwH_6probe65probe() unnamed_addr #0 {
start:
  %pgocount = load i64, i64* getelementptr inbounds ([2 x i64], [2 x i64]* @__profc__RNvCs8ZkI4e3ZCwH_6probe65probe, i32 0, i32 0), align 8
  %0 = add i64 %pgocount, 1
  store i64 %0, i64* getelementptr inbounds ([2 x i64], [2 x i64]* @__profc__RNvCs8ZkI4e3ZCwH_6probe65probe, i32 0, i32 0), align 8
  %1 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %1, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h6e149b300bd4c287E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17ha94a579c0309d35aE([0 x i8]* align 1 bitcast ([25 x i8]* @str.0 to [0 x i8]*), i64 25, %"core::panic::location::Location"* align 8 bitcast (<{ i8*, [16 x i8] }>* @alloc2 to %"core::panic::location::Location"*)) #5
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h6e149b300bd4c287E.exit": ; preds = %start
  br label %bb1

bb1:                                              ; preds = %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h6e149b300bd4c287E.exit"
  ret void
}

; Function Attrs: nofree nosync nounwind readnone willreturn
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17ha94a579c0309d35aE([0 x i8]* align 1, i64, %"core::panic::location::Location"* align 8) unnamed_addr #2

; Function Attrs: nounwind
declare void @llvm.instrprof.increment(i8*, i64, i32, i32) #3

; Function Attrs: noinline
define linkonce_odr hidden i32 @__llvm_profile_runtime_user() #4 {
  %1 = load i32, i32* @__llvm_profile_runtime, align 4
  ret i32 %1
}

attributes #0 = { uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }
attributes #1 = { nofree nosync nounwind readnone willreturn }
attributes #2 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }
attributes #3 = { nounwind }
attributes #4 = { noinline }
attributes #5 = { noreturn }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}
