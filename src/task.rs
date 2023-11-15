#[macro_export]
macro_rules! task {
    ( c: $c:expr, d: $d:expr, t: $t:expr  ) => {
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

    pub fn p_util(&self) -> f64 {
        self.c as f64 / self.t as f64
    }
}

pub struct DeferredTask {
    pub c: u32,
    pub d: u32,
    pub t: u32,
    pub q: u32,
}

impl_abstract_task!(Task, DeferredTask);

pub trait AbstractTask {
    fn c(&self) -> u32;
    fn d(&self) -> u32;
    fn t(&self) -> u32;
    #[inline]
    fn p_util(&self) -> f64 {
        self.c() as f64 / self.t() as f64
    }
}

pub trait EDF: AbstractTask {}

impl EDF for Task {}

pub trait FPPS: AbstractTask {}

impl FPPS for Task {}
