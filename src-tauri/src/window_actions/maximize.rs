use crate::ActionContext;
use crate::window_actions::WindowActionHandler;
use winapi::shared::windef::HWND;

pub struct MaximizeAction {
    pub gutter: i32,
}

impl WindowActionHandler for MaximizeAction {
    fn calculate_position(&self, _context: &ActionContext, hwnd: HWND) -> Result<(i32, i32, i32, i32), String> {
        use winapi::shared::windef::RECT;
        use winapi::um::dwmapi::{DwmGetWindowAttribute, DWMWA_EXTENDED_FRAME_BOUNDS};
        use winapi::um::winuser::{GetDpiForWindow, GetMonitorInfoW, GetWindowRect, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST};

        unsafe {
            // Get the monitor info for proper work area
            let monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
            let mut monitor_info: MONITORINFO = std::mem::zeroed();
            monitor_info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;

            if GetMonitorInfoW(monitor, &mut monitor_info) == 0 {
                return Err("Failed to get monitor info".to_string());
            }

            // Use the actual work area from monitor info
            let work_area = monitor_info.rcWork;
            let dpi = GetDpiForWindow(hwnd);
            let dpi_scale = dpi as f32 / 96.0; // 96 is the default DPI

            // Get the extended frame bounds first to understand current shadow size
            let mut window_rect = std::mem::zeroed();
            let mut extended_frame: RECT = std::mem::zeroed();
            if GetWindowRect(hwnd, &mut window_rect) == 0
                || DwmGetWindowAttribute(
                    hwnd,
                    DWMWA_EXTENDED_FRAME_BOUNDS,
                    &mut extended_frame as *mut RECT as *mut _,
                    std::mem::size_of::<RECT>() as u32,
                ) != 0
            {
                return Err("Failed to get window metrics".to_string());
            }

            // Calculate shadow offsets
            let shadow_left = extended_frame.left - window_rect.left;
            let shadow_top = extended_frame.top - window_rect.top;
            let shadow_right = window_rect.right - extended_frame.right;
            let shadow_bottom = window_rect.bottom - extended_frame.bottom;

            // Calculate dimensions accounting for DPI and gutter
            let scaled_gutter = (self.gutter as f32 * dpi_scale) as i32;
            let work_width = work_area.right - work_area.left;
            let work_height = work_area.bottom - work_area.top;

            // Adjust for shadow offsets to ensure the visible window fills the space
            let adjusted_width = work_width - (scaled_gutter * 2) + shadow_left + shadow_right;
            let adjusted_height = work_height - (scaled_gutter * 2) + shadow_top + shadow_bottom;

            let new_x = work_area.left + scaled_gutter - shadow_left;
            let new_y = work_area.top + scaled_gutter - shadow_top;

            println!("Maximizing with gutter {}: {}x{} at ({},{})", self.gutter, adjusted_width, adjusted_height, new_x, new_y);
            Ok((adjusted_width, adjusted_height, new_x, new_y))
        }
    }
}
