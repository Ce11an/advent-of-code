# Credit 
A massive amount of credit for this repo goes to [Christopher Biscardi](https://github.com/ChristopherBiscardi) and their amazing
[Adventure of Code 2023](https://github.com/ChristopherBiscardi/advent-of-code/tree/main/2023/rust) repo. I am using 
this repo as a way to learn Rust and Advent of Code. I am following Christopher's [YouTube series](https://www.youtube.com/watch?v=JOgQMjpGum0&t=1498s). It is super helpful and I highly recommend it.

# Advent of Code 2023

This year I've pre-set up a series of functionality for testing, benchmarking, and otherwise evaluating the performance of our Rust programs.

This includes the command `just work` which is passed a particular day and part and is the equivalent workflow of running all of these in a row and stopping if one fails, then re-starting the flow after changes.

```
cargo check
cargo nextest run
cargo clippy
```

## Prepare for a new day

```shell
just create <day>
```

## Just

Just is used to partially document all tasks, so you (the person reading this) can see what commands we were running and perhaps run them yourself on your own codebase.

I also thought it would be neat to maybe have scripts that run flamegraphs for all of the days and parts easily, so that they could be checked in and viewable on github and generally make it easier to run and document the running of various tools.

```shell
brew install just
```

## cargo-nextest

[cargo-nextest][cargo-nextest] is "a next-generation test runner for Rust projects". Basically that means it includes [an interesting execution model][cargo-nextest-execution-model] than can be great for projects with a _lot_ of tests.

As of this year's AoC, [cargo-nextest][cargo-nextest] doesn't run doctests yet, so while that won't be an issue for us it is something to be aware of if you're using nextest in a "real project". (Basically that means you also run `cargo test --doc`).

cargo-nextest has what I consider [a positive relationship with the regular `cargo test`](https://nexte.st/book/how-it-works.html#contributing-features-back-to-cargo) and is rightfully a nice place to be experimenting with new testing UX. `cargo test` works well and `cargo nextest` is a forward-looking place for experimentation.

```shell
cargo install cargo-nextest
```

[cargo-nextest]: https://nexte.st/
[cargo-nextest-execution-model]: https://nexte.st/book/how-it-works.html