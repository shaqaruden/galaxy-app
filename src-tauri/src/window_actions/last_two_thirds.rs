use crate::ActionContext;
use crate::window_actions::WindowActionHandler;
use winapi::shared::windef::HWND;

pub struct LastTwoThirdsAction;

impl WindowActionHandler for LastTwoThirdsAction {
    fn calculate_position(&self, context: &ActionContext, _hwnd: HWND) -> Result<(i32, i32, i32, i32), String> {
        let target_monitor_info = &context.monitors[context.target_monitor];
        let target_work_width = target_monitor_info.work_right - target_monitor_info.work_left;
        let target_work_height = target_monitor_info.work_bottom - target_monitor_info.work_top;

        // Snap to last two-thirds of the monitor (right two-thirds)
        let new_width = (2 * target_work_width) / 3;
        let new_height = target_work_height;
        let new_x = target_monitor_info.work_left + (target_work_width / 3);
        let new_y = target_monitor_info.work_top;

        println!("Snapping to last two-thirds: {}x{} at ({},{})", new_width, new_height, new_x, new_y);
        Ok((new_width, new_height, new_x, new_y))
    }
}
