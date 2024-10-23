# Intermediate Representation Language

This Repo houses a WIP toy language that I hope to use to learn more
about compiler optimizations.

Currently it's a pure language that supports some basic features and has
a pretty simple optimizer that can get rid of dead code and do some
basic peephole optimizations.

There's also serialization and deserialization by way of bincode, to
allow the language to be saved as bytecode, and later re-run.

## Goals

- Scaffolding out a frontend (tokenizer/parser/repl)
- Setting up fuzz testing
- Adding a few more features in, like loops, conditional expressions.
- Figuring out how to transform the AST into SSA form and test out
  optimizations that can be done on those.
