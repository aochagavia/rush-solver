Rush Solver
===========

A solver for rush-hour-like puzzles. You can specify a starting point and
obtain the solution with the least amount of vehicle movement.

Try `cargo run --release`, to see it in action!

## Sample output

> Note that each number corresonds to a vehicle. For instance, `22` represents
> a horizontal car in the coordinates `(0, 0)` and `(1, 0)`. In the same vein,
> `777` represents a vertical truck in the coordinates `(5, 0)`, `(5, 1)` and
> `(5, 2)`.

```
Starting point:
|22   7|
|5  6 7|
|5116 7|
|5  6  |
|3   44|
|3 888 |

Solution found
Vehicle 2 Forward
Vehicle 4 Backward
Vehicle 4 Backward
Vehicle 4 Backward
Vehicle 5 Backward
Vehicle 3 Backward
Vehicle 6 Forward
Vehicle 7 Forward
Vehicle 7 Forward
Vehicle 7 Forward
Vehicle 8 Backward
Vehicle 8 Backward
Vehicle 6 Forward
Vehicle 1 Forward
Vehicle 1 Forward
Vehicle 1 Forward

Final stage:
|522   |
|5     |
|5   11|
|3  6 7|
|3446 7|
|8886 7|
```

## License

MIT
