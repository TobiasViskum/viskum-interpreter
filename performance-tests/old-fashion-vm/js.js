function fib(n) {
    if (n < 2) {
        return n
    }
    return fib(n - 2) + fib(n - 1)
}
const now = performance.now();
fib(47);
console.log(performance.now() - now);       