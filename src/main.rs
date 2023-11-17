use scheduling::prelude::*;

fn main() {
    let s = TaskSet::new(vec![
        task!( c: 13, t: 34, d: 19),
        task!( c: 11, t: 41, d: 29),
        task!( c: 10, t: 53, d: 43),
    ]);
    println!("{:?}", s.solve_edf())
}
