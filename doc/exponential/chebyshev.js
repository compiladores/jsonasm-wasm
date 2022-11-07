// Reference: https://en.wikipedia.org/wiki/Chebyshev_polynomials#Recurrence_definition
let chebyshev_table = [
  [1], // T_0(x) = 1
  [0, 1] // T_1(x) = x
]

// T_n(x) = 2 * x * T_{n-1}(x) - T_{n-2}(x)
function get_chebyshev_polynomial(n) {
  if (chebyshev_table[n]) return chebyshev_table[n];
  let p0 = [0].concat(get_chebyshev_polynomial(n-1)).map(v=>2*v) // 2 * x * T_{n-1}(x)
  let p1 = get_chebyshev_polynomial(n-2).map((v,i)=> -v); // - T_{n-2}(x)
  let p = p0.map((v, i) => v + (p1[i] ?? 0));
  chebyshev_table[n] = p; // Memoize
  return p;
}

function kronecker_delta(a, b) {
  return a == b ? 1 : 0;
}

// Reference: https://en.wikipedia.org/wiki/Chebyshev_polynomials#Example_1
function calculate_chebyshev_approximation_coefficients(f, N) {
  let coeffs = [];
  for (let n = 0; n < N; n++) {
    let sum = 0;
    for (let k = 0; k < N; k++) {
      let pos = Math.PI * (k + 0.5) / N;
      sum += Math.cos(n*pos) * f(Math.cos(pos));
    }
    coeffs[n] = (2 - kronecker_delta(n, 0)) * sum / N;
  }
  return coeffs;
}

function calculate_chebyshev_approximation_polynomial(f, N) {
  return calculate_chebyshev_approximation_coefficients(f, N).
    map((a_n, n) => get_chebyshev_polynomial(n).map(v=>a_n*v)).
    reduce((a, p) => a.map((v,i)=>v+(p[i]??0)), Array(N).fill(0));
}

console.log(calculate_chebyshev_approximation_polynomial(x => Math.log(1+x), 5))