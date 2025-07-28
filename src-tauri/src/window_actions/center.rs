use crate::ActionContext;
use crate::window_actions::WindowActionHandler;
use winapi::shared::windef::HWND;

pub struct CenterAction;

impl WindowActionHandler for CenterAction {
    fn calculate_position(&self, context: &ActionContext, _hwnd: HWND) -> Result<(i32, i32, i32, i32), String> {
        let target_monitor_info = &context.monitors[context.target_monitor];
        let target_work_width = target_monitor_info.work_right - target_monitor_info.work_left;
        let target_work_height = target_monitor_info.work_bottom - target_monitor_info.work_top;

        // Keep current size, center the window
        let new_width = context.window_info.current_width;
        let new_height = context.window_info.current_height;
        let new_x = target_monitor_info.work_left + (target_work_width - new_width) / 2;
        let new_y = target_monitor_info.work_top + (target_work_height - new_height) / 2;

        println!("Centering window: {}x{} at ({},{})", new_width, new_height, new_x, new_y);
        Ok((new_width, new_height, new_x, new_y))
    }
}