use crate::ActionContext;
use crate::window_actions::WindowActionHandler;
use winapi::shared::windef::HWND;

pub struct BottomLeftAction;

impl WindowActionHandler for BottomLeftAction {
    fn calculate_position(&self, context: &ActionContext, _hwnd: HWND) -> Result<(i32, i32, i32, i32), String> {
        let target_monitor_info = &context.monitors[context.target_monitor];
        let target_work_width = target_monitor_info.work_right - target_monitor_info.work_left;
        let target_work_height = target_monitor_info.work_bottom - target_monitor_info.work_top;

        // Snap to bottom-left quarter of the monitor
        let new_width = target_work_width / 2;
        let new_height = target_work_height / 2;
        let new_x = target_monitor_info.work_left;
        let new_y = target_monitor_info.work_top + target_work_height / 2;

        println!("Snapping to bottom-left quarter: {}x{} at ({},{})", new_width, new_height, new_x, new_y);
        Ok((new_width, new_height, new_x, new_y))
    }
}
