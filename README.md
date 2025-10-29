

# About

This is a demo repo. The idea is experiment with Claude Code to
implement using custom commands a Formal Specification assistant.


## Tools

1. Rust
2. Verus 
3. Claude Code


## Claude Code Commands

We are using Claude Code and we have added few custom commands:

/comment FILE-REF FUNCTION-NAME

This evoke a prompt that ask Claude to add NL comments as docstring in
the named function in the particular file.

/versus FILE-REF FUNCTION-NAME

This evoke a prompt that ask Claude to add Versus annotations to a
function, possible using the NL comments produced by `/comment`.


