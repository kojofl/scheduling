use scheduling::prelude::*;

fn main() {
    let s = TaskSet::new(vec![
        task!(c: 2, t: 5, d: 5, p: 3),
        task!(c: 2, t: 7, d: 7, p: 2),
        task!(c: 2, t: 7, d: 7, p: 1),
    ]);
    println!("{:?}", s.solve_fpns(Numbers::Real))
}
