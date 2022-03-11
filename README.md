# retrun

## About

This is a fast Wordle solver based on the algorithm 3blue1brown uses.

## Usage

```sh
retrun -c -- -----:0000,tares:21111,teach:21112,tweak:21111,might:13333
```

The syntax of `state` is based on the syntax [Wordle botfights](https://botfights.ai/game/wordle) uses. Run `retrun --help` for more usage info.

## Building

Note: This project uses a build script (`build.rs` in this project's root) to generate a static HashMap at compile time.
