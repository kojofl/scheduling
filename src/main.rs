use scheduling::prelude::*;

fn main() {
    let s = TaskSet::new(vec![
        task!(c: 2, t: 5, d: 5, p: 3),
        task!(c: 2, t: 7, d: 7, p: 2),
        task!(c: 2, t: 7, d: 7, p: 1),
    ]);
    println!("{:?}", s.solve_fpns(Numbers::Real));
    println!("{:?}", s.solve_fpps());
    let s = TaskSet::new(vec![
        task!(c:  6, t: 24, d: 16, o: 3, p: 3),
        task!(c:  4, t: 35, d: 20, o: 3, p: 2),
        task!(c: 13, t: 40, d: 52, o: 2, p: 1),
    ]);
    println!("{:?}", s.solve_fpts(Numbers::Real));
    let s = TaskSet::new(vec![
        task!(c: 11, t: 32, d: 28, o: 2, p: 3),
        task!(c: 10, t: 49, d: 48, o: 3, p: 2),
        task!(c: 12, t: 83, d: 76, o: 3, p: 1),
    ]);
    println!("{:?}", s.solve_fpts(Numbers::Natural));
}
