# mlogp
very slightly higher level wrapper around mlog.
makes jump statements actually usable and will yell at you incomprehensibly if you make a mistake (partial syntax checking)

## how to use
download the code from the [latest release](https://github.com/notvelleda/mlogp/releases/latest) or compile it yourself (directions below), then run it in your terminal of choice (it's a command line program)

## compiling
- install cargo if you haven't already ([guide](https://doc.rust-lang.org/cargo/getting-started/installation.html))
- run `cargo build --release` in the source code directory
- the built executable will be located in `target/release/`, you can do whatever with it at this point

## what it changes
- as with some flavors of BASIC, you create labels with the syntax `labelName:` (where label name can be any label name)
- instead of taking addresses to jump to, the `jump` instruction now takes labels, and will jump to them accordingly
- the instruction `goto <labelName>` is an alias to `jump <labelName> always`, if you want to type slightly less
- basic subroutine support, uses variables to store the instruction pointer locations since we don't have a stack. downsides are recursion doesn't work. it's possible to create a stack, but it would be slow so i didn't
- you can create subroutines with `routine <routineName>:`, return from a subroutine with `return`, and end a subroutine with `endroutine`
- subroutines are called with `gosub <routineName>` or you can use `gosubc` for a conditional gosub, syntax is the same as `jump`
