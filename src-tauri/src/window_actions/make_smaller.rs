use crate::ActionContext;
use crate::window_actions::WindowActionHandler;
use winapi::shared::windef::HWND;

pub struct MakeSmallerAction;

impl WindowActionHandler for MakeSmallerAction {
    fn calculate_position(&self, context: &ActionContext, _hwnd: HWND) -> Result<(i32, i32, i32, i32), String> {
        // Decrease size by 10% of current dimensions, maintaining center position
        let size_decrease_factor = 0.9;
        let new_width = ((context.window_info.current_width as f32) * size_decrease_factor) as i32;
        let new_height = ((context.window_info.current_height as f32) * size_decrease_factor) as i32;

        // Ensure minimum window size (at least 200x150)
        let final_width = new_width.max(200);
        let final_height = new_height.max(150);

        // Center the window with new size
        let new_x = context.window_info.center_x - (final_width / 2);
        let new_y = context.window_info.center_y - (final_height / 2);

        println!("Making window smaller: {}x{} at ({},{})", final_width, final_height, new_x, new_y);
        Ok((final_width, final_height, new_x, new_y))
    }
}