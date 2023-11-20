# scheduling

Scheduling exposes schedubility tests for realtime systems.

Algorithms supported:

- [x] EDF
- [x] FPPS
- [x] FPNS
- [x] FPTS
- [ ] FPDS

For these different algorithms multiple task structs are available that can be
constructed using the task! macro.

The more information a task contains the more schedubility tests are available within 
the taskset i.e. a simple taskset constaining only tasks with c, t, d cannot support 
FPTS.

# Example

```rust
use scheduling::prelude::*;

fn main() {
    let s = TaskSet::new(vec![
        task!(c: 2, t: 5, d: 5, p: 3),
        task!(c: 2, t: 7, d: 7, p: 2),
        task!(c: 2, t: 7, d: 7, p: 1),
    ]);
    println!("{:?}", s.solve_edf())
    println!("{:?}", s.solve_fpps())
    println!("{:?}", s.solve_fpns(Numbers::Real))
}
```


