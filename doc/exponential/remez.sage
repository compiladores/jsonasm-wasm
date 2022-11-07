from numpy import linalg, matrix, array
import bisect

# References:
# https://en.wikipedia.org/wiki/Remez_algorithm
# https://dl.acm.org/doi/pdf/10.1145/321281.321282
# https://abiy-tasissa.github.io/remez.pdf
# https://ask.sagemath.org/question/7823/numerically-find-all-roots-in-an-interval/

RF = RealField(256)

def abserr(a, b):
  return abs(abs(a) - abs(b))

def cull_roots(L, N):
  initlength = len(L)
  while len(L) > N:
    mindiff = 1e10
    minindex = 0
    for i in range(0, len(L)-1):
      diff = abs(L[i] - L[i+1])
      if mindiff > diff:
        mindiff = diff
        minindex = i
    L.pop(minindex)

def checked_insert(L, f, elem, eps):
  start, end = elem
  if start >= end:
    return
  if end - start < eps:
    return
  if sign(f(start)) == sign(f(end)) and eps*RF(1000) > end - start:
    return
  L.append(elem)

def find_potential_extrema(f, a, b, N, eps=1e-6):
    eps = RF(eps * (b - a) / N)
    roots = [a, b]
    intervals_to_check = [(a,b)]
    fabs(x) = abs(f(x))
    fdiff = f.diff()
    fddiff = fdiff.diff()
    while intervals_to_check:
        start, end = intervals_to_check.pop()
        try:
            root = find_root(f, start, end, xtol=RF(1e-50), maxiter=200)
            root_improved = fabs.find_local_minimum(root-eps, root+eps, tol=1e-20)[1]
            if fabs(root) > fabs(root_improved):
              root = root_improved
            root = RF(root)
            while fabs(root) > RF(1e-50): # improve with halley's method
              root = root - 2*f(root)*fdiff(root)/(2*(fdiff(root)^2)-f(root)*fddiff(root))
        except RuntimeError:
            continue
        bisect.insort(roots, root)
        checked_insert(intervals_to_check, f, (start, root-eps), eps)
        checked_insert(intervals_to_check, f, (root+eps, end), eps)
        if len(roots) > 1000*N:
          cull_roots(roots, N)
    cull_roots(roots, N)
    roots.sort()
    return roots

def find_maxima(f, a, b, N):
  points = find_potential_extrema(f, a, b, N+3)
  intervals = [(points[i], points[i+1]) for i in range(N + 2)]
  fabs(x) = abs(f(x))
  maxima = [fabs.find_local_maximum(start, end)[1] for start, end in intervals]
  maxima.sort()
  return maxima

def errdiff(errors):
  return abserr(max(errors), min(errors))

def remez(f, N, a, b):
  assert(b > a)
  # Chebyshev nodes
  X = [ RF((a + b)/2 + ((b - a)/2) * cos(pi*(n+0.5)/(N+1)))
    for n in range(N + 2)]
  # R = floating point polynomial ring of variable x
  R.<x> = PolynomialRing(RF)
  error = new_error = 1 # dummy
  new_coeffs = None
  improvement = 1 # dummy
  coeffs = None
  while improvement > 0:
    # update variables
    error = new_error
    coeffs = new_coeffs
    # find coefficients
    new_coeffs = solve_for_coeffs(f, X, N)
    # construct polynomial
    poly = R(new_coeffs)
    # error function
    err(p) = poly(p) - f(p)
    # find points of maximal error, then use as new sample points
    X = find_maxima(err, a, b, N)
    # there should be N+2 points found
    assert(len(X) == N + 2)
    # find associated errors
    new_error = max([abs(err(p)) for p in X])
    # calculate improvement of maximal error
    improvement = error - new_error
    #print(N, coeffs, error, improvement)
    #print("+", end="", flush=True)
  return coeffs, error.numerical_approx()


def solve_for_coeffs(f, X, N):
  A = matrix([
    [
      X[i-1] ** n
      for n in range(0, N + 1)
    ] + [(-1) ** i]
    for i in range(1, N + 3)
  ], dtype='float64')
  b = array([f(X[i-1]) for i in range(1, N + 3)], dtype='float64')
  sol = linalg.solve(A, b)
  coeffs = sol[:N+1]
  E = sol[N+1]
  return coeffs


start, end = (1, 2)
f(x) = log(x)

_, last_err = remez(f, 0, start, end)
improvement = 1
deg = 1
print(f"Searching polynomials for {f} in range ({start}, {end})")
while improvement > 1 or deg < 3:
  coeffs, err = remez(f, deg, start, end)
  R.<x> = PolynomialRing(RR)
  improvement = last_err / err
  last_err = err
  print(f"Degree {deg} polynomial is {R(coeffs)} with maximal error {err} with a improvement factor of {improvement} over degree {deg-1}")
  print("")
  deg += 1
print(f"Degree {deg-1} polynomial did not improve on {deg-2}'s error. Halting search.")
