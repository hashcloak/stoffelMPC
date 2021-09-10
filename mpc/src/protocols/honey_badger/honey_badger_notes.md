# Honey Badger MPC Overview

This document serves as a summary to extract the relevant parts of the [HoneyBadgerMPC] paper.
We will leave out the implementation of AsynchroMix as this part is not relevant for us.

[HoneyBadgerMPC]: https://eprint.iacr.org/2019/883

### Robust Interpolation

Honey Badger MPC is based on a robust form of Shamir Secret Sharing. We assume that $t < n/3$ 
parties can act maliciously. **RobustInterpolate** tries to interpolate the polynomial with 
the first $t + 1$ shares received. After that a party waits until $2t + 1$ shares are received.
If all shares agree with each other we can directly interpolate the polynomial. If not we
have to use a form of [online error correction].

[online error correction]: https://eprint.iacr.org/2012/517

### Batch Reconstruction
TODO:

- BatchRecPub: c.f. https://www.iacr.org/archive/crypto2007/46220565/46220565.pdf
- Usage of FFT (Fast-Fourier Transform) to get an O(n log²n) overhead
   instead of O(n²) from Vandermonde for interpolation
- Preprocessing for Beaver triples with RanDouSha
