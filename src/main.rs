use scheduling::prelude::*;

fn main() {
    let mut s = TaskSet::new(vec![
        task!(c: 2, d: 5, t: 9),
        task!(c: 12, d: 33, t: 38),
        task!(c: 7, d: 16, t: 23),
    ]);
    println!("{:?}", s.solve_fpps())
}
