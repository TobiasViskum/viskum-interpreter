clang gen.c -o gen && ./gen && clang -S -emit-llvm gen.c &&
code --reuse-window gen.ll