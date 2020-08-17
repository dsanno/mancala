Mancala
====

Mancala also known as [Kalah](https://en.wikipedia.org/wiki/Kalah) implementation with Rust.

# Requirements

- Rust language (https://www.rust-lang.org/)
- Python3 (https://www.python.org/) if you'd like to train computer player parameters.
  - torch 1.5.1 (https://pytorch.org/)

# How to build

```
$ cd mancala
$ cargo build --release
```

or `$ cargo build` if you want debug build binary.

# Usage

## Play with computer player

1. Download [a released binary](../../releases) and copy `eval.dat` and `position.dat` from it to `mancala` directory (which has `Cargo.toml`).
2. Run commands as follows
```
$ cd mancala
$ ./target/release/mancala play 18
```

Command line parameters:

```
$ mancala play <depth>
```

- `<depth>`: Depth of computer player thinking.

### Command during game

- `1-6`: Sow seeds in a pit (`1` for most left pit, `2` for next to it, etc)
- `undo` or `u`: Undo move.
- `reset` or `r`: Reset game.
- `quit` or `q`: Quit game.


## View evaluation

1. Download [a released binary](../../releases) and copy `eval.dat` and `position.dat` from it to `mancala` directory (which has `Cargo.toml`).
2. Run commands as follows
```
$ cd mancala
$ ./target/release/mancala eval 18
```

Command line parameters:

```
$ mancala play <depth>
```

- `<depth>`: Depth of evaluation.

### Command during evaluation

- `1-6`: Sow seeds in a pit (`1` for most left pit, `2` for next to it, etc)
- `undo` or `u`: Undo move.
- `reset` or `r`: Reset board.
- `quit` or `q`: Quit game.


## Train computer player parameters.

### 1. Run self playing

```
$ cd mancala
$ ./target/release/mancala self 12 30000
```

This takes several hours.

Command line parameters:

```
$ mancala self <depth> <self_play_num>
```

- `<depth>`: Depth of computer player thinking.
- `<self_play_num>`: Number of selp playing.

### 2. Optimize parameters

```
$ python script/train_evaluator.py mancala/position.dat mancala/eval.dat
```

This takes several hours.

### 3. Repeat 1 and 2


## Compare evaluator parameters

1. Make evaluator parameter files.
2. Run commands as follows (It may take several hours with deep thought.)
```
$ cd mancala
$ ./target/release/mancala comp 18 eval1.dat eval2.dat
```
3. You will get winning rate and average score of first player against second player.

Command line parameters:

```
$ mancala play <depth> <evaluation_file_1> <evaluation_file_2>
```

- `<depth>`: Depth of computer player thinking.
- `<evaluation_file_1>`: Evaluator parameter file for first player.
- `<evaluation_file_2>`: Evaluator parameter file for second player.
