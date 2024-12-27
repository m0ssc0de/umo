mod ranges;
#[cfg(feature = "until_2025")]
pub use ranges::get_open_ranges;

mod pick;
pub use pick::pick_open_range;
