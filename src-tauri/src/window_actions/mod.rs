use crate::{ActionContext, MonitorInfo};
use winapi::shared::windef::HWND;

pub trait WindowActionHandler {
    /// Determines which monitor the window should be moved to
    fn get_target_monitor(&self, current_monitor: usize, monitors: &[MonitorInfo]) -> usize {
        // Default: stay on current monitor
        current_monitor
    }

    /// Calculates the new position and size for the window
    fn calculate_position(
        &self,
        context: &ActionContext,
        hwnd: HWND,
    ) -> Result<(i32, i32, i32, i32), String>;
}

// Re-export all action implementations
mod bottom_half;
mod bottom_left;
mod bottom_right;
mod left_half;
mod maximize;
mod move_left;
mod move_right;
mod no_op;
mod right_half;
mod top_half;
mod top_left;
mod top_right;

pub use bottom_half::BottomHalfAction;
pub use bottom_left::BottomLeftAction;
pub use bottom_right::BottomRightAction;
pub use left_half::LeftHalfAction;
pub use maximize::MaximizeAction;
pub use move_left::MoveLeftAction;
pub use move_right::MoveRightAction;
pub use no_op::NoOpAction;
pub use right_half::RightHalfAction;
pub use top_half::TopHalfAction;
pub use top_left::TopLeftAction;
pub use top_right::TopRightAction;
