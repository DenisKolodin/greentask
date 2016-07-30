# greentask

Rust library to pass tasks into green coroutines and take the results.

Usage:
```rust
strcut In(&'static str);
struct Out(&'static str);

let mut resumer = spawn(|mut yielder, first_in| {
    let second_in = yielder.yield_with(Out("first out"));
    Out("second out")
});

resumer.resume_with(In("first in"));
resumer.resume_with(In("second in"));
```
