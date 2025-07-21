use crate::ActionContext;
use crate::window_actions::WindowActionHandler;
use winapi::shared::windef::HWND;

pub struct NoOpAction;

impl WindowActionHandler for NoOpAction {
    fn calculate_position(&self, context: &ActionContext, _hwnd: HWND) -> Result<(i32, i32, i32, i32), String> {
        // Return current position and size (no change)
        Ok((
            context.window_info.current_width,
            context.window_info.current_height,
            context.window_info.center_x - context.window_info.current_width / 2,
            context.window_info.center_y - context.window_info.current_height / 2,
        ))
    }
}
