let start = performance.now();

let d = 0;
let sum = 0;

while (true) {
  d = d + 1
  if (d == 700000) {
    break;
  } else {
    let a = 0

    while (a <= 15) {
      sum = sum  +a;
      a  = a + 1
    }
  }
} 

console.log(performance.now() - start);

// function fib(n) {
//     if (n < 2) return n;
//     return fib(n - 2) + fib(n - 1);
//   }

// let start = performance.now();
// console.log(fib(35));
// console.log(performance.now() - start);
