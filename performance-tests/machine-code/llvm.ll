define i32 @main() {
entry:
  %1 = call i32 @add(i32 2, i32 3)
  ret i32 %1
}

define i32 @add(i32 %x, i32 %y) {
entry:
  %addtmp = add i32 %x, %y
  ret i32 %addtmp
}
