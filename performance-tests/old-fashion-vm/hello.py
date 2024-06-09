import time

def fib(n):
    if (n < 2):
        return n
    return fib(n - 2) + fib(n - 1)


before = round(time.time() * 1000)
fib(35)
after = round(time.time() * 1000)
print(after - before)