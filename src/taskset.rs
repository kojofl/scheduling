use std::{collections::HashSet, fmt::Debug};

use crate::task::{AbstractTask, Threshhold};

/// The epsilon used as the first iteration start time in FPNS
/// if there is no blocking time.
static EPSILON: f64 = 0.000001;

#[macro_export]
macro_rules! gcd {
    ($x:expr, $y:expr) => {
        scheduling::taskset::gcd($x, $y)
    };
    ($x:expr, $y:expr, $($r:expr),+) => {
        gcd!(scheduling::taskset::gcd($x, $y), $($r),+)
    };
}

#[derive(Clone, Debug)]
pub enum SchedulingResult {
    Schedulable(String),
    Unschedulable(String),
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeadlineStrategy {
    Constraint,
    Implicit,
    Arbitrary,
}

#[derive(Clone, Debug)]
pub struct TaskSet<T: AbstractTask + Ord> {
    pub set: Vec<T>,
    strategy: DeadlineStrategy,
}

impl<T: AbstractTask + Ord> TaskSet<T> {
    pub fn new(mut tasks: Vec<T>) -> Self {
        let mut strategy = DeadlineStrategy::Implicit;
        for t in tasks.iter() {
            if t.d() > t.t() {
                strategy = DeadlineStrategy::Arbitrary;
                break;
            }
            if t.d() < t.t() {
                strategy = DeadlineStrategy::Constraint;
            }
        }
        tasks.sort();
        Self {
            set: tasks,
            strategy,
        }
    }

    pub fn processor_util(&self) -> f64 {
        self.set.iter().fold(0.0, |acc, t| acc + t.p_util())
    }

    pub fn h(&self) -> u32 {
        let a = self.set[0].t();
        let b = self.set[1].t();
        let mut g = gcd(a, b);
        let mut h = a * b / g;
        for b in &self.set[2..] {
            let b = b.t();
            g = gcd(h, b);
            h = h * b / g;
        }
        h
    }
}

impl<T: Threshhold + Ord> TaskSet<T> {
    pub fn solve_fpts(&self) -> SchedulingResult {
        let u = self.processor_util();
        if u > 1.0 {
            return SchedulingResult::Unschedulable(format!("Processor demand above 1\nU: {u}"));
        }
        self.level_i_threshhold()
    }

    fn level_i_threshhold(&self) -> SchedulingResult {
        for (i, t) in self.set.iter().enumerate() {
            let b = self
                .set
                .iter()
                .filter(|t2| t2.p() < t.p() && t.p() <= t2.o())
                .fold(0, |max, t| max.max(t.c() - 1));
            println!("{b}");
            let mut level_i = b + self.set[..=i].iter().fold(0, |acc, t| acc + t.c());
            loop {
                let new_l = b + self.set[..=i].iter().fold(0, |acc, t| {
                    acc + (level_i as f64 / t.t() as f64).ceil() as u32 * t.c()
                });
                // We found fixpoint and thus the level i active period
                if new_l == level_i {
                    break;
                }
                level_i = new_l;
            }
            println!("task_{i} has level i active: {level_i}");
            let l = (level_i as f64 / t.t() as f64).ceil() as u32;
            let mut s = (b as f64).max(EPSILON);
            for k in 1..=l {
                let d_k = t.d() * k;
                loop {
                    let n_s_k = b
                        + (k - 1) * t.c()
                        + self.set[..i].iter().fold(0, |acc, t2| {
                            acc + ((s / t2.t() as f64).floor() as u32 + 1) * t2.c()
                        });
                    if n_s_k > d_k {
                        return SchedulingResult::Unschedulable(format!("Deadline miss in task: {i} number: {k} start_time: {n_s_k} deadline: {d_k}"));
                    }
                    if n_s_k == s as u32 {
                        println!("Task{i}_{k} start time at s: {s}");
                        break;
                    }
                    s = n_s_k as f64;
                }
                let mut f = s as u32 + t.c();
                loop {
                    if f > d_k {
                        return SchedulingResult::Unschedulable(format!(
                                "Deadline miss in task: {i} number: {k} finish_time: {f} deadline: {d_k}"
                                ));
                    }
                    let f_new = s as u32
                        + t.c()
                        + self.set[..i]
                            .iter()
                            .chain(&self.set[i + 1..])
                            .filter(|t2| t2.p() > t.o())
                            .fold(0, |acc, t| {
                                acc + ((f as f64 / t.t() as f64).ceil() as u32
                                    - ((s / t.t() as f64).floor() as u32 + 1))
                                    * t.c()
                            });
                    if f == f_new {
                        break;
                    }
                    f = f_new;
                }
            }
        }
        SchedulingResult::Schedulable("No deadline miss in tasks in level i active period".into())
    }
}

impl<T: AbstractTask + Ord> TaskSet<T> {
    pub fn solve_fpns(&self) -> SchedulingResult {
        let u = self.processor_util();
        if u > 1.0 {
            return SchedulingResult::Unschedulable(format!("Processor demand above 1\nU: {u}"));
        }
        self.level_i_blocking()
    }

    fn level_i_blocking(&self) -> SchedulingResult {
        for (i, t) in self.set.iter().enumerate() {
            let b: u32 = self.set[i + 1..]
                .iter()
                .fold(0, |max, t| max.max(t.c() - 1));
            println!("{b}");
            let mut level_i = b + self.set[..=i].iter().fold(0, |acc, t| acc + t.c());
            loop {
                let new_l = b + self.set[..=i].iter().fold(0, |acc, t| {
                    acc + (level_i as f64 / t.t() as f64).ceil() as u32 * t.c()
                });
                // We found fixpoint and thus the level i active period
                if new_l == level_i {
                    break;
                }
                level_i = new_l;
            }
            println!("task_{i} has level i active: {level_i}");
            let l = (level_i as f64 / t.t() as f64).ceil() as u32;
            let mut s = (b as f64).max(EPSILON);
            for k in 1..=l {
                let d_k = t.d() * k;
                loop {
                    let n_s_k = b
                        + (k - 1) * t.c()
                        + self.set[..i].iter().fold(0, |acc, t2| {
                            acc + ((s / t2.t() as f64).floor() as u32 + 1) * t2.c()
                        });
                    if n_s_k > d_k {
                        return SchedulingResult::Unschedulable(format!("Deadline miss in task: {i} number: {k} start_time: {n_s_k} deadline: {d_k}"));
                    }
                    if n_s_k == s as u32 {
                        println!("Task{i}_{k} start time at s: {s}");
                        break;
                    }
                    s = n_s_k as f64;
                }
                let r = s as u32 + t.c();
                if r > d_k {
                    return SchedulingResult::Unschedulable(format!(
                        "Deadline miss in task: {i} number: {k} response_time: {r} deadline: {d_k}"
                    ));
                }
            }
        }
        SchedulingResult::Schedulable("No deadline miss in tasks in level i active period".into())
    }
}

impl<T: AbstractTask + Ord> TaskSet<T> {
    /// Solves schedubility test for FPPS in increasing complexity
    /// It is important to sort the tasks according to their priority
    /// before calling this function.
    /// This priority depends on the tasks if for example theire are
    /// normal `Task` it will expect EDF if their are `PriorityTask`
    /// it is sorted according to the given priority.
    pub fn solve_fpps(&self) -> SchedulingResult {
        // Processor util test
        let u = self.processor_util();
        if u > 1.0 {
            return SchedulingResult::Unschedulable(format!("Processor demand above 1\nU: {u}"));
        }
        match self.strategy {
            // If implicit deadlines we can do U_lub and Hyperbolic bound befor resorting to rtime
            DeadlineStrategy::Implicit => {
                println!("U: {u}");
                // U_lub test
                let n = self.set.len() as f64;
                let u_lub = n * (2_f64.powf(1.0 / n) - 1.0);
                if u <= u_lub {
                    return SchedulingResult::Schedulable(format!(
                        "Processor util under lowest upper bound: {u_lub} U: {u}"
                    ));
                }
                let h_bound = self.set.iter().fold(1.0, |acc, t| acc * (t.p_util() + 1.0));
                if h_bound <= 2.0 {
                    return SchedulingResult::Schedulable(format!(
                        "Hyperbolic Bound test successfull {h_bound} <= 2.0"
                    ));
                }
                println!("h_bound: {h_bound}");
                self.rtime()
            }
            DeadlineStrategy::Constraint => self.rtime(),
            DeadlineStrategy::Arbitrary => self.level_i_non_blocking(),
        }
    }

    /// Response time analysis.
    fn rtime(&self) -> SchedulingResult {
        for (i, t) in self.set.iter().enumerate() {
            let mut point = self.set[..=i].iter().fold(0, |acc, t| acc + t.c());
            let mut j = 0;
            println!("R{i},{j}: {point}");
            loop {
                j += 1;
                let new_p = t.c()
                    + self.set[..i].iter().fold(0, |acc, t| {
                        acc + ((point as f64 / t.t() as f64).ceil() as u32) * t.c()
                    });
                if new_p > t.d() {
                    return SchedulingResult::Unschedulable(
                        format!("The response time analysis detected a deadline miss at {new_p} in T{i} c: {}, t: {}, d: {}", t.c(), t.t(), t.d()),
                        );
                }
                if point == new_p {
                    break;
                }
                point = new_p;
                println!("R{i},{j}: {point}");
            }
            println!("R{i}: {point}");
        }
        SchedulingResult::Schedulable(
            "No deadline miss in response time analysis => Schedulable using DM".into(),
        )
    }

    fn level_i_non_blocking(&self) -> SchedulingResult {
        for (i, t) in self.set.iter().enumerate() {
            let mut level_i = self.set[..=i].iter().fold(0, |acc, t| acc + t.c()) as f64;
            loop {
                let new_l = self.set[..=i].iter().fold(0.0, |acc, t| {
                    acc + (level_i / t.t() as f64).ceil() * t.c() as f64
                });
                // We found fixpoint and thus the level i active period
                if new_l == level_i {
                    break;
                }
                level_i = new_l;
            }
            let l = (level_i / t.t() as f64).ceil() as u32;
            let mut f: Option<u32> = None;
            for k in 1..=l {
                let mut f_k = if k == 1 {
                    t.c() + self.set[..i].iter().fold(0, |acc, t| acc + t.c())
                } else {
                    f.unwrap() + t.c()
                };
                let d_k = t.d() * k;
                loop {
                    let n_f_k = k * t.c()
                        + self.set[..i].iter().fold(0, |acc, t2| {
                            acc + (f_k as f64 / t2.t() as f64).ceil() as u32 * t2.c()
                        });
                    if n_f_k > d_k {
                        return SchedulingResult::Unschedulable(format!("Deadline miss in task: {i} number: {k} finish_time: {n_f_k} deadline: {d_k}"));
                    }
                    if n_f_k == f_k {
                        println!("Task{i}_{k} scheduable with {f_k} and deadline {d_k}");
                        break;
                    }
                    f_k = n_f_k;
                }
                f = Some(f_k);
            }
        }
        SchedulingResult::Schedulable("No deadline miss in tasks in level i active period".into())
    }
}

// Seperate edf implementation
impl<T: AbstractTask + Ord> TaskSet<T> {
    pub fn solve_edf(&self) -> SchedulingResult {
        let u = self.processor_util();
        if u > 1.0 {
            return SchedulingResult::Unschedulable(format!("Processor demand above 1\nU: {u}"));
        }
        println!("U: {u}");
        match self.strategy {
            DeadlineStrategy::Implicit => SchedulingResult::Schedulable(format!(
                "Processor demand below 1 and implicit deadlines\nU: {u}"
            )),
            _ => self.pda(),
        }
    }

    fn pda(&self) -> SchedulingResult {
        // DS = { D_k | d_k < min(H, max(D_max, L*)) }
        let bound = (self.h() as f64)
            .min((self.set.iter().map(AbstractTask::d).max().unwrap() as f64).max(self.l_star()));
        let mut ls: Vec<u32> = self
            .set
            .iter()
            .map(|t| (t.d(), t.t()))
            .flat_map(|(d, t)| {
                let mut l_v = vec![d];
                let mut start = t;
                while start + d < bound as u32 {
                    l_v.push(start + d);
                    start += t;
                }
                l_v.into_iter()
            })
            .collect::<HashSet<u32>>()
            .into_iter()
            .collect();
        ls.sort();
        for l in ls {
            let g = self.g(l as f64);
            if l < g as u32 {
                return SchedulingResult::Unschedulable(format!(
                    "Processor demand failed L: {}, g(0,L): {}",
                    l, g
                ));
            }
        }
        SchedulingResult::Schedulable(format!(
            "Processor demand success all d < {bound} could be respected",
        ))
    }

    /// L* that is used to determine the max number in stepfunction
    /// of the processor demand to check.
    fn l_star(&self) -> f64 {
        let (u, s) = self.set.iter().fold((0.0, 0.0), |mut acc, t| {
            let u = t.p_util();
            let d = t.d() as f64;
            let t = t.t() as f64;
            acc.0 += u;
            acc.1 += (t - d) * u;
            acc
        });
        s / (1.0 - u)
    }

    /// calculates processor demand until l
    fn g(&self, l: f64) -> f64 {
        self.set.iter().fold(0.0, |acc, t| {
            acc + ((l + t.t() as f64 - t.d() as f64) / t.t() as f64).floor() * t.c() as f64
        })
    }
}

/// Recursively calculates the greates common divisor.
pub fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
