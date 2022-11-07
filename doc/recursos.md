


Spec: https://www.w3.org/TR/wasm-core-1/

## Libraries
* https://docs.rs/walrus/latest/walrus/
(I dislike the model, having to store locals at a global level to then have them [filtered](https://github.com/rustwasm/walrus/blob/75d4728cf27ef73c0105b476a99561b78797f200/src/module/functions/local_function/mod.rs#L170) later)
* https://github.com/GoogleChromeLabs/wasmbin

## Exponential implementation
WASM has no native exponential instruction and thus it has to be implemented manually.
For that I considered multiple methods, first CORDIC then Chevysheb polynomials and finally minmax polynomials with the Remez algorithm.
The minmax approach has smaller maximal error, but Chebyshev has better average error.