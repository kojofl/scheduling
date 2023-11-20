use scheduling::prelude::*;

fn main() {
    let s = TaskSet::new(vec![
        task!(c: 1, t: 6, d: 4, o: 3, p: 3),
        task!(c: 3, t: 10, d: 10, o: 3, p: 2),
        task!(c: 6, t: 18, d: 12, o: 2, p: 1),
    ]);
    println!("{:?}", s.solve_fpts())
}
