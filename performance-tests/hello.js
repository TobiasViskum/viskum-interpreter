function fib(n) {
    if (n < 2) {
        return n
    }
    return fib(n - 2) + fib(n - 1)
}

let start = performance.now();
let result = fib(40);
let end = performance.now();
console.log(end - start);