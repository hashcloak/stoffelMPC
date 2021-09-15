# Honey Badger MPC Overview

This document serves as a summary to extract the relevant parts of the [HoneyBadgerMPC] paper.
We will leave out the implementation of AsynchroMix as this part is not relevant for us.

[HoneyBadgerMPC]: https://eprint.iacr.org/2019/883

### Arithmetic in FFT-Friendly Finite Fields
HoneyBadgerMPC operates on the finite field behind the BL12-381 curve. This means that field
elements are a multiple of some $2^k$-root of unity $\omega$. We should check how these
operations are supported in the ark-ff crate.

### Robust Interpolation

Honey Badger MPC is based on a robust form of Shamir Secret Sharing. We assume that $t < n/3$ 
parties can act maliciously. **RobustInterpolate** tries to interpolate the polynomial with 
the first $t + 1$ shares received. After that a party waits until $2t + 1$ shares are received.
If all shares mtach with each other we can directly interpolate the polynomial. If not we
have to use a form of [online error correction].

[online error correction]: https://eprint.iacr.org/2012/517

### RSDecode

Used to correct errors if RobustInterpolate receives a set of shares, which do not agree. This
can either use ordinary matrix multiplication or an [FFT-based algorithm]. It seems that the
FFT-based approach is faster for Vandermonde matrices, which have more than 10k columns.

[FFT-based algorithm]: https://sci-hub.mksa.top/10.1109/ccnc.2010.5421749



### Batch Reconstruction

This is the [algorithm] used to batch-reconstruct shares, which makes use of RobustInterpolate twice.

[algorithm]: https://www.iacr.org/archive/crypto2007/46220565/46220565.pdf

### TODO:

- Preprocessing for Beaver triples with RanDouSha
