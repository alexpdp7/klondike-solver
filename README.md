# Klondike solver

I HAVE NOT VERIFIED THAT THE SOLUTIONS ARE CORRECT.

THE SOLVER IS OMNISCIENT.

The solver deals cards in threes, can cycle the draw pile an unlimited amount of times.

On my desktop computer, with a 1 minute timeout and 20 test runs:

* 55% solved games (~80% under three seconds, the rest between 12-25 seconds)
* 5% exhausts the search without finding a solution (in under a second)
* 40% times out

```
cargo run --release --bin auto
```

(Comparatively, I play a greedy strategy without considering the position of cards in the draw pile, solving about 14% of the games I play.)

Should pass:

```
cargo clippy -- -D clippy::pedantic -A clippy::single_match_else -A clippy::items-after-statements
```
