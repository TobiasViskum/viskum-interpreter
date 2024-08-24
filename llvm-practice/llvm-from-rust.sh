rustc src/main.rs --emit=llvm-ir -O -C no-prepopulate-passes -C codegen-units=1 &&
code --reuse-window main.ll