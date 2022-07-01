; ModuleID = 'probe1.e5d11323-cgu.0'
source_filename = "probe1.e5d11323-cgu.0"
target datalayout = "e-m:o-i64:64-i128:128-n32:64-S128"
target triple = "arm64-apple-macosx11.0.0"

@__llvm_profile_runtime = external global i32

; Function Attrs: noinline
define linkonce_odr hidden i32 @__llvm_profile_runtime_user() #0 {
  %1 = load i32, i32* @__llvm_profile_runtime, align 4
  ret i32 %1
}

attributes #0 = { noinline }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}
