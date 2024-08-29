# dmsh
dmsh is a modern C and Rust based shell with the goal of learning and creating
a fast and reliable shell.

## Coding style
### C
We use .clang-format for our C code, following Mozilla style-guide
### Rust
See [Rust style guide](https://doc.rust-lang.org/nightly/style-guide/index.html)
to learn about how we write our Rust code.

## Roadmap
### Shell (C)
- [x] Simple REPL to execute arbitrary commands from $PATH.
- [x] Builtins like `cd`, ...
- [x] Integrate Rust builtins with the shell.
### Builtin (Rust)
- [x] qcat (cat on steroids)
- [x] lls  (pretty ls)
- [x] qfind (quick find)
- [ ] qgrep (quick grep)

## Warning
dmsh doesn't handle symbolic links. Beware of `cd` and `pwd` (`pwd` is *not* a
builtin in dmsh).

## TODO
- [ ] Try not to use string literals in C when `const`ness isn't guaranteed.
- [ ] Handle SIGINT *during* child program execution.
- [ ] Create `install` target in Makefile and update DMSH_{QCAT,LLS} to use
      nonlocal versions of the binaries.
- [ ] TODOs mentioned in code comments.

## Credits
- [This](https://brennan.io/2015/01/16/write-a-shell-in-c/) great tutorial.
