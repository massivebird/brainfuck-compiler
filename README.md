# Brainfuck compiler

Compiles Brainfuck source code into x86_64 Intel-flavored assembly.

Written in Rust 🦀 and inspired by [pretzelhammer's Too Many Brainfuck Compilers](https://github.com/pretzelhammer/rust-blog/blob/master/posts/too-many-brainfuck-compilers.md#what-is-brainfuck)!

Not sure if it's working? Try [this awesome browser interpreter](https://minond.xyz/brainfuck/).

## Usage

This is how I'm using this program:

```bash
$ brainfuck-compiler code.bf # produces a file called "out"
$ as out -o assembled # assembles assembly
$ ld assembled -o run # links the assembled assembly
$ ./run
```

> I'm new to compilers. Am I supposed to include a "run" option that automates the `as` and `ld` commands??
