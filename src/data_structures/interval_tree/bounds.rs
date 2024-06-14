use crate::utils::Interval;

pub trait Bounds {
    fn traverse_left_candidate<N: Ord + Clone>(interval_start: &N, candidate_max: &N) -> bool;
    fn traverse_right_candidate<N: Ord + Clone>(interval_end: &N, candidate_start: &N) -> bool;
    fn intersects<N: Ord + Clone>(base: &Interval<N>, other: &Interval<N>) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Inclusive;

impl Bounds for Inclusive {
    fn intersects<N: Ord + Clone>(range_1: &Interval<N>, range_2: &Interval<N>) -> bool {
        range_1.start <= range_1.end
            && range_2.start <= range_2.end
            && range_1.end >= range_2.start
            && range_1.start <= range_2.end
    }

    fn traverse_left_candidate<N: Ord + Clone>(interval_start: &N, candidate_max: &N) -> bool {
        interval_start <= candidate_max
    }

    fn traverse_right_candidate<N: Ord + Clone>(interval_end: &N, candidate_start: &N) -> bool {
        interval_end >= candidate_start
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Exclusive;

impl Bounds for Exclusive {
    fn intersects<N: Ord + Clone>(range_1: &Interval<N>, range_2: &Interval<N>) -> bool {
        range_1.start < range_1.end
            && range_2.start < range_2.end
            && range_1.end > range_2.start
            && range_1.start < range_2.end
    }

    fn traverse_left_candidate<N: Ord + Clone>(interval_start: &N, candidate_max: &N) -> bool {
        interval_start < candidate_max
    }

    fn traverse_right_candidate<N: Ord + Clone>(interval_end: &N, candidate_start: &N) -> bool {
        interval_end > candidate_start
    }
}
