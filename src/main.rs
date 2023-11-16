use scheduling::prelude::*;

fn main() {
    let mut s = TaskSet::new(vec![
        task!(c: 1, d: 4, t: 6, o: 3, p: 3),
        task!(c: 3, d: 10, t: 10, o: 3, p: 2),
        task!(c: 6, d: 12, t: 18, o: 2, p: 1),
    ]);
    println!("{:?}", s.solve_fpps())
}
