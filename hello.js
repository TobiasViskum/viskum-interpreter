function cmp(n) {
  if (n < 1) return n;
  return cmp(n - 1)
}
let start = performance.now();
let result = cmp(5000);

console.log(performance.now() - start);

// function fib(n) {
//     if (n < 2) return n;
//     return fib(n - 2) + fib(n - 1);
//   }
  
// let start = performance.now();
// console.log(fib(35));
// console.log(performance.now() - start);