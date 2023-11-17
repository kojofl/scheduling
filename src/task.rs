#[macro_export]
macro_rules! task {
    ( c: $c:expr, d: $d:expr, t: $t:expr  ) => {
        Task {
            c: $c,
            d: $d,
            t: $t,
        }
    };
    ( c: $c:expr, t: $t:expr, d: $d:expr  ) => {
        Task {
            c: $c,
            d: $d,
            t: $t,
        }
    };
    ( c: $c:expr, d: $d:expr, t: $t:expr, q: $q:expr  ) => {
        DeferredTask {
            c: $c,
            d: $d,
            t: $t,
            q: $q,
        }
    };
    ( c: $c:expr, t: $t:expr, d: $d:expr, q: $q:expr  ) => {
        DeferredTask {
            c: $c,
            d: $d,
            t: $t,
            q: $q,
        }
    };
    ( c: $c:expr, d: $d:expr, t: $t:expr, o: $o:expr, p: $p:expr  ) => {
        ThreshTask {
            c: $c,
            d: $d,
            t: $t,
            o: $o,
            p: $p,
        }
    };
    ( c: $c:expr, t: $t:expr, d: $d:expr, o: $o:expr, p: $p:expr  ) => {
        ThreshTask {
            c: $c,
            d: $d,
            t: $t,
            o: $o,
            p: $p,
        }
    };
}

macro_rules! impl_abstract_task {
    ( $($t:ty),+ ) => {
        $(
            impl AbstractTask for $t {
                #[inline]
                fn c(&self) -> u32 {
                    self.c
                }
                #[inline]
                fn d(&self) -> u32 {
                    self.d
                }
                #[inline]
                fn t(&self) -> u32 {
                    self.t
                }
            }
        )+
    };
}

/// The Task represents a basic task that has
/// computation time c, deadline d and period t.
#[derive(Clone, Copy, Debug)]
pub struct Task {
    pub c: u32,
    pub d: u32,
    pub t: u32,
}

impl Task {
    pub fn new(c: u32, d: u32, t: u32) -> Self {
        Self { c, d, t }
    }
}

pub struct DeferredTask {
    pub c: u32,
    pub d: u32,
    pub t: u32,
    pub q: u32,
}

pub struct ThreshTask {
    pub c: u32,
    pub d: u32,
    pub t: u32,
    pub o: u32,
    pub p: u32,
}

impl_abstract_task!(Task, DeferredTask, ThreshTask);

/// The AbstractTask is the trait implemented by all Tasks.
/// All Tasks need to have a way to retrieve, c, d and t.
/// This Trait enables EDF and FPPS scheduling tests.
pub trait AbstractTask {
    /// computation time c.
    fn c(&self) -> u32;
    /// deadline c.
    fn d(&self) -> u32;
    /// period t.
    fn t(&self) -> u32;
    /// Calculates processor util of task.
    #[inline]
    fn p_util(&self) -> f64 {
        self.c() as f64 / self.t() as f64
    }
}
