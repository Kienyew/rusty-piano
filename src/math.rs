use num_traits::Float;
use ordered_float::OrderedFloat;
use std::cmp::{max, min};

pub fn line_intersect_length<F: Float>(y1: F, height1: F, y2: F, height2: F) -> F {
    let lower_point = min(OrderedFloat(y1 + height1), OrderedFloat(y2 + height2)).into_inner();

    let upper_point = max(OrderedFloat(y1), OrderedFloat(y2)).into_inner();

    if lower_point < upper_point {
        F::zero()
    } else {
        lower_point - upper_point
    }
}
