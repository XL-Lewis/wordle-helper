# Installation
1. [Install rust](https://www.rust-lang.org/tools/install)
2. Clone repo and use `cargo run` to start

# Usage
1. First, input a word size (regular wordle is 5 letters)
2. Enter words as you guess them, and the helper will display:
  i)   A timer
  ii)  A set of the last letters removed
  iii) A set of any letters which have been used more than once in a single word
  iv)  A summary of unused letters, sorted alphabetically AND by letter frequency

## Commands / Args

`-e` Exit the solver (ctrl + c also works)

`-c` Clear the screen and reprint the current game state

`-r [word size]` Clear the solver's memory, and start a new game with the specified letter count

`-p [unknown letters] [known letters]` Display the potential placement of each unknown letter (on a new line) considering both used letters AND the specified known letters. In the following example, 'a' can be in either position 1 or 5, while b can be in 3 or 5 based on previous guesses:

```
>-p ab _C_E_

_C_E_
a___a
__b_b
```

`-s [unknown letters] [known letters]` Show all potential combinations of a particular set of known and unknown letters. Duplicate unknown letters should be specified twice. In the following example, there are three potential combinations involving the unkown letters A and B.
```
>-s ab _C_E_

_C_E_

ACBE_
AC_EB
_CBEA
```
