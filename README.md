# Klondike solver

I HAVE NOT VERIFIED THAT THE SOLUTIONS ARE CORRECT.

THE SOLVER IS OMNISCIENT.

The solver deals cards in threes, can cycle the draw pile an unlimited amount of times.

On my desktop computer, with a 1 minute timeout and 20 test runs:

* 55% solved games
* 10% exhausts the search without finding a solution
* 35% times out

Games can take up to around 30 seconds to solve.

```
cargo run --release --bin auto
```

(Comparatively, I play a greedy strategy without considering the position of cards in the draw pile, solving about 14% of the games I play.)
