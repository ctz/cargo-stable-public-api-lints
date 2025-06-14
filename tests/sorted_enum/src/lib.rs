pub enum NotSorted {
    C,
    B,
    A,
}

pub enum Sorted {
    A,
    B,
    C,
}

pub enum WithExplicitOrdinals {
    A = 1,
    B = 2,
    C = 3,
}

pub enum SortedWithNonExhaustive {
    A,
    B,
    #[non_exhaustive]
    C,
}
