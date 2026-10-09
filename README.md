# Klondike solver

I HAVE ONLY VERIFIED THAT TWO SOLUTIONS ARE CORRECT.

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

I now recommend implementing a Klondike solver for fun.
I implemented the first version of the solver that solves 55% of games in three sessions across three days.
With the right approach (not the most obvious one, but not superobscure if you have solved similar problems), not too many difficult tricks are required.
Mainly, implementing the draw pile logic and the moves is quite tedious:

* 170 lines of code to implement basic deck functionality
* 800 lines of code to implement game logic
* 100 lines of code to implement the solver
* 20 lines of code to implement an interactive harness to develop the game logic
* 20 lines of code to implement the solver interface
