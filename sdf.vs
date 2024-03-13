state<i8> count = state(0)

reIncr:

count++

if count < 10 { goto reIncr }

count = 0

stdout.println("Hello")