import time

start = time.time()

d = 0
sum = 0

while True:
    d = d + 1

    if d == 700000:
        break
    else:
        a = 0

        while a <= 15:
            sum = sum + a
            a = a + 1

print(time.time() - start)
