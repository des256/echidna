# Crates

In an attempt to reduce compilation time and a long list of dependencies,
these are locally redeveloped or refactored versions of various essential
crates.

## math

Basic mathematics, vector and matrix calculus and SIMD data types.

## codec

Data serialization. This is essentially a simplified `serde` clone.

## async

Futures, a reactor and various asynchronous versions of APIs. This is
essentially a simplified `tokio` clone.

Currently repackages `smol`, which does a very good job at this.

## data

Data transport system over UDP. This is an extremely simplified DDS clone,
which can later be replaced by actual DDS, as soon as there is better native
support.

## sys-sys

Interface to underlying C or Objective-C APIs that are still required.
