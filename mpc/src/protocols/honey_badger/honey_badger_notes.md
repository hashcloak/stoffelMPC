# Honey Badger MPC Overview

This document serves as a summary to extract the relevant parts of the [HoneyBadgerMPC] paper.
We will leave out the implementation of AsynchroMix as this part is not relevant for us.

[HoneyBadgerMPC]: https://eprint.iacr.org/2019/883

### Arithmetic in FFT-Friendly Finite Fields
HoneyBadgerMPC operates on the finite field behind the BL12-381 curve. This means that field
elements are a multiple of some $2^k$-root of unity $\omega$. We should check how these
operations are supported in the ark-ff crate.

### Batch Reconstruction
This is the [algorithm] \(page 13, second box\) used to batch-reconstruct shares, which makes use
of RobustInterpolate twice. The key idea is to extend $t + 1$ sharings to $n$ sharings, by 
switching the roles of coefficients and points, so $M(x_{ij}) \alpha_j = y_i$ becomes
$M(\alpha_{ij}) x_j = y_i$, with $i = (1...n)$ and $j = (1...t+1)$. Then every player sends his
row $j$ to player $j$, who robust-interpolates all the equations he received this way and hence
reconstucts $y_j$. This process is repeated once more and we end up with $t +1$ opened values
$x_1, x_2, ..., x_{t + 1}$.

[algorithm]: https://www.iacr.org/archive/crypto2007/46220565/46220565.pdf

### Robust Interpolation

Honey Badger MPC is based on a robust form of Shamir Secret Sharing. We assume that $t < n/3$ 
parties can act maliciously. **RobustInterpolate** tries to interpolate the polynomial with 
the first $t + 1$ shares received. After that a party waits until $2t + 1$ shares are received.
If all shares mtach with each other we can directly interpolate the polynomial. If not we
have to use a form of [online error correction].

[online error correction]: https://eprint.iacr.org/2012/517

### RSDecode

Used to correct errors if RobustInterpolate receives a set of shares, which do not agree. This
can either use ordinary matrix multiplication, in which case the [Berlekamp-Welch] algorithm is used, 
or an FFT-based algorithm ([Soro-Lacan]). It seems that the
FFT-based approach is faster for Vandermonde matrices, which have more than 10k columns.

[Berlekamp-Welch]: https://jeremykun.com/2015/09/07/welch-berlekamp
[Soro-Lacan]: https://sci-hub.mksa.top/10.1109/ccnc.2010.5421749

### TODO:

- Preprocessing for Beaver triples with RanDouSha
