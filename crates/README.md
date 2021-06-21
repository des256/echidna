# Crates

In an attempt to reduce compilation time and a long list of dependencies,
these are locally redeveloped or refactored versions of various essential
crates.

## math

Basic mathematics, vector and matrix calculus and SIMD data types.

## codec

Data serialization. This is essentially a simplified `serde` clone.

## async

- replaced by `tokio` (because of ReadHalf/WriteHalf).

## data

Data transport system over UDP. This is an extremely simplified DDS clone,
which can later be replaced by actual DDS, as soon as there is better native
support.

## sys-sys

Interface to underlying C or Objective-C APIs that are still required.
