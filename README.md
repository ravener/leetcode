# Leetcode Rust

This repository holds my personal attempts to solve leetcode problems in Rust.

## Usage

The code is structured with a module per each leetcode problem, the main function does nothing but rather the modules describe test cases that needs to pass when calling `cargo test`, you can also run tests for a single module using `cargo test <module>` such as `cargo test two_sum`.

The `impl Solution` block is the part that can be copy pasted into leetcode (alongside some `use` statements probably).

Having the code inside a git repository and being able to use an editor and other tools such as clippy and rustfmt is helpful to have since the leetcode editor is pretty basic.