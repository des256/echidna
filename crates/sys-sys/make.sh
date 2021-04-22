#!/bin/sh
# This generates the Rust code bindings to underlying C and Objective-C APIs.
bindgen --disable-nested-struct-naming --no-prepend-enum-name --no-layout-tests wrapper.h -o src/bindings.rs

# add -- -D_SYMBOL_ to do configuration between systems later