# dmsh
dmsh is a modern C and Rust based shell with the goal of learning and creating
a fast and reliable shell.

## Coding style
### C
See [suckless](https://suckless.org/coding_style/) to learn about how we write
our C code.
### Rust
See [Rust style guide](https://doc.rust-lang.org/nightly/style-guide/index.html)
to learn about how we write our Rust code.

## Roadmap
### Shell (C)
- [x] Simple REPL to execute arbitrary commands from $PATH.
- [x] Builtins like `cd`, ...
- [x] Integrate Rust builtins with the shell.
### Builtin (Rust)
- [x] fcat (cat on steroids)
- [ ] lls  (pretty ls)

## Warning
dmsh doesn't handle symbolic links. Beware of `cd` and `pwd` (`pwd` is *not* a
builtin in dmsh).

## TODO
- [ ] Try not to use string literals in C when `const`ness isn't guaranteed.
- [ ] Handle SIGINT *during* child program execution.
- [ ] TODOs mentioned in code comments.
