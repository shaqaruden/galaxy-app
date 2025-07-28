use crate::ActionContext;
use crate::window_actions::WindowActionHandler;
use winapi::shared::windef::HWND;

pub struct MaximizeHeightAction;

impl WindowActionHandler for MaximizeHeightAction {
    fn calculate_position(&self, context: &ActionContext, _hwnd: HWND) -> Result<(i32, i32, i32, i32), String> {
        let target_monitor_info = &context.monitors[context.target_monitor];
        let target_work_height = target_monitor_info.work_bottom - target_monitor_info.work_top;

        // Keep current width and horizontal position, maximize height only
        let new_width = context.window_info.current_width;
        let new_height = target_work_height;
        let new_x = context.window_info.center_x - (new_width / 2);
        let new_y = target_monitor_info.work_top;

        println!("Maximizing height: {}x{} at ({},{})", new_width, new_height, new_x, new_y);
        Ok((new_width, new_height, new_x, new_y))
    }
}