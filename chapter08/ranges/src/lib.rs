use std::ops::Range;

/// 2つのRangeに重なる部分があればtrueを返す。
/// ```
/// assert_eq!(ranges::overlap(0..7, 3..10), true);
/// assert_eq!(ranges::overlap(1..5, 101..105), false);
/// ```
///
/// どちらかの範囲が空であれば重なっていないことにする。
///
/// ```
/// assert_eq!(ranges::overlap(0..0, 0..10), false);
/// ```
///
pub fn overlap(r1: Range<usize>, r2: Range<usize>) -> bool {
    // r1の開始位置がr2の終了位置より前で、r1の終了位置がr2の開始位置より後であれば重なっている。
    // どちらかの範囲が空であれば重なっていない。
    r1.start < r1.end && r2.start < r2.end && r1.start < r2.end && r2.start < r1.end
}
