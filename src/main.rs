use scheduling::prelude::*;

fn main() {
    let s = TaskSet::new(vec![
        task!(c: 2, t: 5, d: 5),
        task!(c: 2, t: 7, d: 7),
        task!(c: 2, t: 7, d: 7),
    ]);
    println!("{:?}", s.solve_fpps())
}
