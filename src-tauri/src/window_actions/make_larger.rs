use crate::ActionContext;
use crate::window_actions::WindowActionHandler;
use winapi::shared::windef::HWND;

pub struct MakeLargerAction;

impl WindowActionHandler for MakeLargerAction {
    fn calculate_position(&self, context: &ActionContext, _hwnd: HWND) -> Result<(i32, i32, i32, i32), String> {
        let target_monitor_info = &context.monitors[context.target_monitor];
        let target_work_width = target_monitor_info.work_right - target_monitor_info.work_left;
        let target_work_height = target_monitor_info.work_bottom - target_monitor_info.work_top;

        // Increase size by 10% of current dimensions, maintaining center position
        let size_increase_factor = 1.1;
        let new_width = ((context.window_info.current_width as f32) * size_increase_factor) as i32;
        let new_height = ((context.window_info.current_height as f32) * size_increase_factor) as i32;

        // Ensure new size doesn't exceed work area
        let final_width = new_width.min(target_work_width);
        let final_height = new_height.min(target_work_height);

        // Center the window with new size
        let new_x = context.window_info.center_x - (final_width / 2);
        let new_y = context.window_info.center_y - (final_height / 2);

        // Ensure window stays within work area bounds
        let final_x = new_x.max(target_monitor_info.work_left)
            .min(target_monitor_info.work_right - final_width);
        let final_y = new_y.max(target_monitor_info.work_top)
            .min(target_monitor_info.work_bottom - final_height);

        println!("Making window larger: {}x{} at ({},{})", final_width, final_height, final_x, final_y);
        Ok((final_width, final_height, final_x, final_y))
    }
}