; ModuleID = 'probe2.7e2101f7-cgu.0'
source_filename = "probe2.7e2101f7-cgu.0"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx11.0.0"

@__covrec_12423233AC2D4D79u = linkonce_odr hidden constant <{ i64, i32, i64, i64, [9 x i8] }> <{ i64 1315669238658977145, i32 9, i64 -1094251526817366627, i64 967489048203684622, [9 x i8] c"\01\01\00\01\01\01\01\00=" }>, section "__LLVM_COV,__llvm_covfun", align 8
@__covrec_A29E317535057F4E = linkonce_odr hidden constant <{ i64, i32, i64, i64, [9 x i8] }> <{ i64 -6728886413774454962, i32 9, i64 0, i64 967489048203684622, [9 x i8] c"\01\01\00\01\01\01\1C\008" }>, section "__LLVM_COV,__llvm_covfun", align 8
@__llvm_coverage_mapping = private constant { { i32, i32, i32, i32 }, [84 x i8] } { { i32, i32, i32, i32 } { i32 0, i32 84, i32 0, i32 5 }, [84 x i8] c"\02Q\00I/Users/subham/.cargo/registry/src/github.com-1ecc6299db9ec823/rayon-1.5.3\06<anon>" }, section "__LLVM_COV,__llvm_covmap", align 8
@__llvm_profile_runtime = external global i32
@__profc__RNvCsoUZuEGWCPP_6probe25probe = private global [2 x i64] zeroinitializer, section "__DATA,__llvm_prf_cnts", align 8
@__profd__RNvCsoUZuEGWCPP_6probe25probe = private global { i64, i64, i64, i8*, i8*, i32, [2 x i16] } { i64 1315669238658977145, i64 -1094251526817366627, i64 sub (i64 ptrtoint ([2 x i64]* @__profc__RNvCsoUZuEGWCPP_6probe25probe to i64), i64 ptrtoint ({ i64, i64, i64, i8*, i8*, i32, [2 x i16] }* @__profd__RNvCsoUZuEGWCPP_6probe25probe to i64)), i8* null, i8* null, i32 2, [2 x i16] zeroinitializer }, section "__DATA,__llvm_prf_data,regular,live_support", align 8
@__profc__RINvNvCsoUZuEGWCPP_6probe25probe4__fooKpEB4_ = private global [1 x i64] zeroinitializer, section "__DATA,__llvm_prf_cnts", align 8
@__profd__RINvNvCsoUZuEGWCPP_6probe25probe4__fooKpEB4_ = private global { i64, i64, i64, i8*, i8*, i32, [2 x i16] } { i64 -6728886413774454962, i64 0, i64 sub (i64 ptrtoint ([1 x i64]* @__profc__RINvNvCsoUZuEGWCPP_6probe25probe4__fooKpEB4_ to i64), i64 ptrtoint ({ i64, i64, i64, i8*, i8*, i32, [2 x i16] }* @__profd__RINvNvCsoUZuEGWCPP_6probe25probe4__fooKpEB4_ to i64)), i8* null, i8* null, i32 1, [2 x i16] zeroinitializer }, section "__DATA,__llvm_prf_data,regular,live_support", align 8
@__llvm_prf_nm = private constant [78 x i8] c"L\00_RNvCsoUZuEGWCPP_6probe25probe\01_RINvNvCsoUZuEGWCPP_6probe25probe4__fooKpEB4_", section "__DATA,__llvm_prf_names", align 1
@llvm.compiler.used = appending global [3 x i8*] [i8* bitcast (i32 ()* @__llvm_profile_runtime_user to i8*), i8* bitcast ({ i64, i64, i64, i8*, i8*, i32, [2 x i16] }* @__profd__RNvCsoUZuEGWCPP_6probe25probe to i8*), i8* bitcast ({ i64, i64, i64, i8*, i8*, i32, [2 x i16] }* @__profd__RINvNvCsoUZuEGWCPP_6probe25probe4__fooKpEB4_ to i8*)], section "llvm.metadata"
@llvm.used = appending global [4 x i8*] [i8* bitcast (<{ i64, i32, i64, i64, [9 x i8] }>* @__covrec_12423233AC2D4D79u to i8*), i8* bitcast (<{ i64, i32, i64, i64, [9 x i8] }>* @__covrec_A29E317535057F4E to i8*), i8* bitcast ({ { i32, i32, i32, i32 }, [84 x i8] }* @__llvm_coverage_mapping to i8*), i8* getelementptr inbounds ([78 x i8], [78 x i8]* @__llvm_prf_nm, i32 0, i32 0)], section "llvm.metadata"

; probe2::probe
; Function Attrs: uwtable
define void @_RNvCsoUZuEGWCPP_6probe25probe() unnamed_addr #0 {
start:
  %pgocount = load i64, i64* getelementptr inbounds ([2 x i64], [2 x i64]* @__profc__RNvCsoUZuEGWCPP_6probe25probe, i32 0, i32 0), align 8
  %0 = add i64 %pgocount, 1
  store i64 %0, i64* getelementptr inbounds ([2 x i64], [2 x i64]* @__profc__RNvCsoUZuEGWCPP_6probe25probe, i32 0, i32 0), align 8
  ret void
}

; Function Attrs: nounwind
declare void @llvm.instrprof.increment(i8*, i64, i32, i32) #1

; probe2::probe::_foo::<_>
define private void @_RINvNvCsoUZuEGWCPP_6probe25probe4__fooKpEB4_() unnamed_addr {
unused_function:
  %pgocount = load i64, i64* getelementptr inbounds ([1 x i64], [1 x i64]* @__profc__RINvNvCsoUZuEGWCPP_6probe25probe4__fooKpEB4_, i64 1, i32 0), align 8
  %0 = add i64 %pgocount, 1
  store i64 %0, i64* getelementptr inbounds ([1 x i64], [1 x i64]* @__profc__RINvNvCsoUZuEGWCPP_6probe25probe4__fooKpEB4_, i64 1, i32 0), align 8
  ret void
}

; Function Attrs: noinline
define linkonce_odr hidden i32 @__llvm_profile_runtime_user() #2 {
  %1 = load i32, i32* @__llvm_profile_runtime, align 4
  ret i32 %1
}

attributes #0 = { uwtable "frame-pointer"="non-leaf" "target-cpu"="apple-a14" }
attributes #1 = { nounwind }
attributes #2 = { noinline }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}
