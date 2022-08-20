# stack-based-lang

This is my first attempt at creating an actual programming language, so I'd thought I'd start simple with a stack based language like forth. I'm writing a slow interpreter for it temporarily, but eventually I think it will transpile to C.

# Features needed to be added

- Multibyte character support (need to figure out how to do this efficiently lol so for now I'm doing byte indexing for everything)
- String interning for symbols instead of using smol_str
- Arg parsing
- Let bindings?
- Better number parsing (only works with positive numbers right now lol)
