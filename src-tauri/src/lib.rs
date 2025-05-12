#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
};
use winapi::um::winuser::{EnumDisplayMonitors, GetMonitorInfoW, MONITORINFO, MONITORINFOEXW};
use window_vibrancy::apply_mica;

// Global storage for monitor information
static MONITORS: Lazy<Mutex<Vec<MonitorInfo>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[derive(Clone, Debug)]
struct MonitorInfo {
    // Full monitor dimensions
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
    // Work area (excludes taskbar and other reserved areas)
    work_left: i32,
    work_top: i32,
    work_right: i32,
    work_bottom: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Action {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Maximize { gutter: i32 },
    None,
}

/// Moves the currently focused window to a new position and/or size.
///
/// # Arguments
///
/// * `action`: The action to perform on the window. If `None`, no action is taken.
/// * `width`: The new width of the window. If `None`, the current width is maintained.
/// * `height`: The new height of the window. If `None`, the current height is maintained.
/// * `gutter`: The gutter size to use when maximizing the window. If `None`, a default gutter size is used.
#[tauri::command]
fn move_window(
    action: Option<Action>
) -> Result<(), String> {
    println!("move_window called with action: {:?}", action);
    #[cfg(target_os = "windows")]
    {
        use std::ptr;

        use winapi::shared::windef::HWND;
        use winapi::um::winuser::{
            GetDpiForWindow, GetSystemMetricsForDpi, SM_CXSCREEN, SM_CYSCREEN,
        };
        use winapi::um::winuser::{GetForegroundWindow, GetWindowRect, SetWindowPos};

        unsafe {
            let hwnd: HWND = GetForegroundWindow();
            if !hwnd.is_null() {
                // Get current window position and size
                let mut rect = std::mem::zeroed();
                if GetWindowRect(hwnd, &mut rect) == 0 {
                    return Err("Failed to get window position".to_string());
                }

                // Update monitor information
                get_monitor_info();
                println!("Getting monitor info");

                // Find current monitor
                let monitors = MONITORS.lock().unwrap();
                let window_center_x = (rect.left + rect.right) / 2;
                let window_center_y = (rect.top + rect.bottom) / 2;

                println!(
                    "Window center at ({}, {})",
                    window_center_x, window_center_y
                );
                let current_monitor = monitors
                    .iter()
                    .position(|m| {
                        let is_in_monitor = window_center_x >= m.left
                            && window_center_x < m.right
                            && window_center_y >= m.top
                            && window_center_y < m.bottom;
                        println!(
                            "Checking monitor: left={}, right={}, top={}, bottom={}, matches={}",
                            m.left, m.right, m.top, m.bottom, is_in_monitor
                        );
                        is_in_monitor
                    })
                    .unwrap_or(0);
                println!("Current monitor index: {}", current_monitor);

                let current_monitor_info = &monitors[current_monitor];
                let target_monitor = match action.as_ref().unwrap_or(&Action::None) {
                    &Action::MoveLeft => {
                        // Find the rightmost monitor that is to the left of current monitor
                        let target = monitors
                            .iter()
                            .enumerate()
                            .filter(|(_, m)| m.right <= current_monitor_info.left) // Only consider monitors to the left
                            .max_by_key(|(_, m)| m.right) // Get the rightmost one
                            .map(|(idx, _)| idx)
                            .unwrap_or_else(|| {
                                // If no monitor to the left, wrap to rightmost monitor
                                monitors
                                    .iter()
                                    .enumerate()
                                    .max_by_key(|(_, m)| m.right)
                                    .map(|(idx, _)| idx)
                                    .unwrap_or(current_monitor)
                            });
                        println!("Moving LEFT to monitor {}", target);
                        target
                    }
                    &Action::MoveRight => {
                        // Find the leftmost monitor that is to the right of current monitor
                        let target = monitors
                            .iter()
                            .enumerate()
                            .filter(|(_, m)| m.left >= current_monitor_info.right) // Only consider monitors to the right
                            .min_by_key(|(_, m)| m.left) // Get the leftmost one
                            .map(|(idx, _)| idx)
                            .unwrap_or_else(|| {
                                // If no monitor to the right, wrap to leftmost monitor
                                monitors
                                    .iter()
                                    .enumerate()
                                    .min_by_key(|(_, m)| m.left)
                                    .map(|(idx, _)| idx)
                                    .unwrap_or(current_monitor)
                            });
                        println!("Moving RIGHT to monitor {}", target);
                        target
                    }
                    &Action::MoveUp | &Action::MoveDown | &Action::None => {
                        println!("No movement");
                        current_monitor
                    }
                    &Action::Maximize { gutter } => {
                        println!("Maximizing window on current monitor with gutter {}", gutter);
                        current_monitor
                    }
                };

                let target_monitor_info = &monitors[target_monitor];

                // Calculate current window dimensions
                let current_width = rect.right - rect.left;
                let current_height = rect.bottom - rect.top;

                // Calculate current monitor work area dimensions
                let current_work_width = current_monitor_info.work_right - current_monitor_info.work_left;
                let current_work_height = current_monitor_info.work_bottom - current_monitor_info.work_top;

                // Calculate target monitor work area dimensions
                let target_work_width = target_monitor_info.work_right - target_monitor_info.work_left;
                let target_work_height = target_monitor_info.work_bottom - target_monitor_info.work_top;

                // Calculate relative position within current monitor work area (as percentages)
                let relative_x = (window_center_x - current_monitor_info.work_left) as f32 / current_work_width as f32;
                let relative_y = (window_center_y - current_monitor_info.work_top) as f32 / current_work_height as f32;

                // Calculate new dimensions and position based on action
                let (new_width, new_height, new_x, new_y) = match action.as_ref().unwrap_or(&Action::None) {
                    &Action::Maximize { gutter } => {
                        // For maximize, use the monitor's work area dimensions minus gutter
                        let adjusted_width = target_work_width - (gutter * 2);
                        let adjusted_height = target_work_height - (gutter * 2);
                        (
                            adjusted_width,
                            adjusted_height,
                            target_monitor_info.work_left + gutter,
                            target_monitor_info.work_top + gutter
                        )
                    }
                    _ => {
                        // For other actions, maintain aspect ratio if possible
                        let width_percentage = current_width as f32 / current_work_width as f32;
                        let height_percentage = current_height as f32 / current_work_height as f32;

                        let new_width = (target_work_width as f32 * width_percentage).min(target_work_width as f32) as i32;
                        let new_height = (target_work_height as f32 * height_percentage).min(target_work_height as f32) as i32;

                        // Calculate new center position
                        let new_center_x = target_monitor_info.work_left + (target_work_width as f32 * relative_x) as i32;
                        let new_center_y = target_monitor_info.work_top + (target_work_height as f32 * relative_y) as i32;

                        // Calculate final window position ensuring it stays within work area bounds
                        let new_x = (new_center_x - new_width / 2).max(target_monitor_info.work_left)
                            .min(target_monitor_info.work_right - new_width);
                        let new_y = (new_center_y - new_height / 2).max(target_monitor_info.work_top)
                            .min(target_monitor_info.work_bottom - new_height);

                        (new_width, new_height, new_x, new_y)
                    }
                };

                let result = SetWindowPos(
                    hwnd,
                    ptr::null_mut(),
                    new_x,
                    new_y,
                    new_width,
                    new_height,
                    0,                    // Remove SWP_NOMOVE flag to allow movement
                );
                if result != 0 {
                    println!("Window moved successfully");
                    Ok(())
                } else {
                    let error = "Failed to move/resize window".to_string();
                    println!("Error: {}", error);
                    Err(error)
                }
            } else {
                Err("No focused window found".to_string())
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("Function not implemented for this OS".to_string())
    }
}

#[tauri::command]
/// Gets information about all connected monitors
///
/// # Safety
/// This function is unsafe because it uses Windows API calls.
/// The callback function is guaranteed to be valid for the duration of EnumDisplayMonitors.
fn get_monitor_info() {
    // Clear existing monitor information
    MONITORS.lock().unwrap().clear();

    unsafe {
        // Safe to pass null for HDC and RECT to enumerate all monitors
        let _ = EnumDisplayMonitors(
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            Some(enum_monitor_callback),
            0,
        );
    }
}

unsafe extern "system" fn enum_monitor_callback(
  monitor: winapi::shared::windef::HMONITOR,
  _: winapi::shared::windef::HDC,
  _: *mut winapi::shared::windef::RECT,
  _: winapi::shared::minwindef::LPARAM,
) -> i32 {
  let mut monitor_info: MONITORINFOEXW = unsafe { std::mem::zeroed() };
  monitor_info.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;

  if GetMonitorInfoW(
      monitor,
      &mut monitor_info as *mut MONITORINFOEXW as *mut MONITORINFO,
  ) != 0
  {
      let width = monitor_info.rcMonitor.right - monitor_info.rcMonitor.left;
      let height = monitor_info.rcMonitor.bottom - monitor_info.rcMonitor.top;
      println!(
          "Monitor at ({}, {}): {}x{}",
          monitor_info.rcMonitor.left, monitor_info.rcMonitor.top, width, height
      );

      // Store monitor information
      MONITORS.lock().unwrap().push(MonitorInfo {
          left: monitor_info.rcMonitor.left,
          top: monitor_info.rcMonitor.top,
          right: monitor_info.rcMonitor.right,
          bottom: monitor_info.rcMonitor.bottom,
          work_left: monitor_info.rcWork.left,
          work_top: monitor_info.rcWork.top,
          work_right: monitor_info.rcWork.right,
          work_bottom: monitor_info.rcWork.bottom,
      });
  }

  1 // Continue enumeration
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            #[cfg(desktop)]
            {
                use std::str::FromStr;
                use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

                // Define shortcuts using strings (format: "modifiers+key")
                let move_monitor_left = Shortcut::from_str("CommandOrControl+Alt+Shift+ArrowLeft").unwrap();
                let move_monitor_right = Shortcut::from_str("CommandOrControl+Alt+Shift+ArrowRight").unwrap();
                let maximize_window = Shortcut::from_str("CommandOrControl+Alt+Enter").unwrap();
                let almost_maximize_window = Shortcut::from_str("CommandOrControl+Alt+Shift+Enter").unwrap();

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new().with_handler(move |_app, shortcut, event| {
                        println!("{:?}", shortcut);
                        if shortcut == &move_monitor_left {
                            match event.state() {
                              ShortcutState::Pressed => {}
                              ShortcutState::Released => {
                                let _ = move_window(Some(Action::MoveLeft));
                              }
                            }
                        }

                        if shortcut == &move_monitor_right {
                            match event.state() {
                              ShortcutState::Pressed => {}
                              ShortcutState::Released => {
                                let _ = move_window(Some(Action::MoveRight));
                              }
                            }
                        }

                        if shortcut == &maximize_window {
                            match event.state() {
                              ShortcutState::Pressed => {}
                              ShortcutState::Released => {
                                let _ = move_window(Some(Action::Maximize { gutter: 0 }));
                                println!("Maximize window");
                              }
                            }
                        }

                        if shortcut == &almost_maximize_window {
                            match event.state() {
                              ShortcutState::Pressed => {}
                              ShortcutState::Released => {
                                let _ = move_window(Some(Action::Maximize { gutter: 32 }));
                                println!("Almost Maximize window");
                              }
                            }
                        }
                    })
                    .build(),
                )?;

                app.global_shortcut().register(move_monitor_left)?;
                app.global_shortcut().register(move_monitor_right)?;
                app.global_shortcut().register(maximize_window)?;
                app.global_shortcut().register(almost_maximize_window)?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
