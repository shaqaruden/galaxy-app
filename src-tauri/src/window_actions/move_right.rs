use crate::{ActionContext, MonitorInfo};
use crate::window_actions::WindowActionHandler;
use winapi::shared::windef::HWND;

pub struct MoveRightAction;

impl WindowActionHandler for MoveRightAction {
    fn get_target_monitor(&self, current_monitor: usize, monitors: &[MonitorInfo]) -> usize {
        let current_monitor_info = &monitors[current_monitor];
        
        // Find the leftmost monitor that is to the right of current monitor
        monitors
            .iter()
            .enumerate()
            .filter(|(_, m)| m.left >= current_monitor_info.right)
            .min_by_key(|(_, m)| m.left)
            .map(|(idx, _)| idx)
            .unwrap_or_else(|| {
                // If no monitor to the right, wrap to leftmost monitor
                monitors
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, m)| m.left)
                    .map(|(idx, _)| idx)
                    .unwrap_or(current_monitor)
            })
    }

    fn calculate_position(&self, context: &ActionContext, _hwnd: HWND) -> Result<(i32, i32, i32, i32), String> {
        let target_monitor_info = &context.monitors[context.target_monitor];
        let current_monitor_info = &context.monitors[context.current_monitor];
        
        // Calculate work area dimensions
        let current_work_width = current_monitor_info.work_right - current_monitor_info.work_left;
        let current_work_height = current_monitor_info.work_bottom - current_monitor_info.work_top;
        let target_work_width = target_monitor_info.work_right - target_monitor_info.work_left;
        let target_work_height = target_monitor_info.work_bottom - target_monitor_info.work_top;

        // Calculate relative position within current monitor work area
        let relative_x = (context.window_info.center_x - current_monitor_info.work_left) as f32 / current_work_width as f32;
        let relative_y = (context.window_info.center_y - current_monitor_info.work_top) as f32 / current_work_height as f32;

        // Maintain aspect ratio
        let width_percentage = context.window_info.current_width as f32 / current_work_width as f32;
        let height_percentage = context.window_info.current_height as f32 / current_work_height as f32;

        let new_width = (target_work_width as f32 * width_percentage).min(target_work_width as f32) as i32;
        let new_height = (target_work_height as f32 * height_percentage).min(target_work_height as f32) as i32;

        // Calculate new center position
        let new_center_x = target_monitor_info.work_left + (target_work_width as f32 * relative_x) as i32;
        let new_center_y = target_monitor_info.work_top + (target_work_height as f32 * relative_y) as i32;

        // Calculate final window position ensuring it stays within work area bounds
        let new_x = (new_center_x - new_width / 2)
            .max(target_monitor_info.work_left)
            .min(target_monitor_info.work_right - new_width);
        let new_y = (new_center_y - new_height / 2)
            .max(target_monitor_info.work_top)
            .min(target_monitor_info.work_bottom - new_height);

        println!("Moving RIGHT to monitor {}: {}x{} at ({},{})", context.target_monitor, new_width, new_height, new_x, new_y);
        Ok((new_width, new_height, new_x, new_y))
    }
}
