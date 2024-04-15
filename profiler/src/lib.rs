pub use profiler_macros::*;

#[cfg(debug_assertions)]
mod time_tracker;

#[cfg(debug_assertions)]
pub use time_tracker::*;
