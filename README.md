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
- [ ] Builtins like `cd`, ... (We'll later integrate with Rust.)
- [ ] Pipes and redirection.
- [ ] Conditionals, loops...
### Builtin (Rust)
- [ ] fcat (cat on steroids)

TODO Think of more commands that should be builtin
