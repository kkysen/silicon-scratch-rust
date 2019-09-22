use std::cmp::max;

// order important, lower can be cast to higher
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum ComputeKind {
    Computational,
    Graphical,
    Reactive,
}

pub trait Computable {
    fn get_compute_kind(&self) -> ComputeKind;
}

impl Computable for ComputeKind {
    fn get_compute_kind(&self) -> ComputeKind {
        *self
    }
}

impl<T: Computable, U: Computable> Computable for (&T, &U) {
    fn get_compute_kind(&self) -> ComputeKind {
        let (a, b) = self;
        max(a.get_compute_kind(), b.get_compute_kind())
    }
}

// can't impl on an Iterator b/c (_, _) is an Iterator and can't specialize Traits
impl<T> Computable for &[T] where T: Computable {
    fn get_compute_kind(&self) -> ComputeKind {
        self.iter()
            .map(Computable::get_compute_kind)
            .max()
            .unwrap_or(ComputeKind::Computational)
    }
}
