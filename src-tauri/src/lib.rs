#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::Mutex;

mod shortcuts;
mod window_actions;

use shortcuts::{update_shortcut, ShortcutManager};
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager, AppHandle,
};
use winapi::um::winuser::{EnumDisplayMonitors, GetMonitorInfoW, MONITORINFO, MONITORINFOEXW};
use window_actions::*;

// Global storage for monitor information
static MONITORS: Lazy<Mutex<Vec<MonitorInfo>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[derive(Clone, Debug)]
pub struct MonitorInfo {
    // Full monitor dimensions
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    // Work area (excludes taskbar and other reserved areas)
    pub work_left: i32,
    pub work_top: i32,
    pub work_right: i32,
    pub work_bottom: i32,
}

#[derive(Clone, Debug)]
pub struct WindowInfo {
    pub current_width: i32,
    pub current_height: i32,
    pub center_x: i32,
    pub center_y: i32,
}

#[derive(Clone, Debug)]
pub struct ActionContext {
    pub current_monitor: usize,
    pub target_monitor: usize,
    pub monitors: Vec<MonitorInfo>,
    pub window_info: WindowInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Action {
    MoveLeft,
    MoveRight,
    Maximize { gutter: i32 },
    LeftHalf,
    RightHalf,
    TopHalf,
    BottomHalf,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    FirstThird,
    CenterThird,
    LastThird,
    FirstTwoThirds,
    LastTwoThirds,
    Center,
    MakeLarger,
    MakeSmaller,
    MaximizeHeight,
    None,
}

impl Action {
    fn get_handler(&self) -> Box<dyn WindowActionHandler> {
        match self {
            Action::MoveLeft => Box::new(MoveLeftAction),
            Action::MoveRight => Box::new(MoveRightAction),
            Action::Maximize { gutter } => Box::new(MaximizeAction { gutter: *gutter }),
            Action::LeftHalf => Box::new(LeftHalfAction),
            Action::RightHalf => Box::new(RightHalfAction),
            Action::TopHalf => Box::new(TopHalfAction),
            Action::BottomHalf => Box::new(BottomHalfAction),
            Action::TopLeft => Box::new(TopLeftAction),
            Action::TopRight => Box::new(TopRightAction),
            Action::BottomLeft => Box::new(BottomLeftAction),
            Action::BottomRight => Box::new(BottomRightAction),
            Action::FirstThird => Box::new(FirstThirdAction),
            Action::CenterThird => Box::new(CenterThirdAction),
            Action::LastThird => Box::new(LastThirdAction),
            Action::FirstTwoThirds => Box::new(FirstTwoThirdsAction),
            Action::LastTwoThirds => Box::new(LastTwoThirdsAction),
            Action::Center => Box::new(CenterAction),
            Action::MakeLarger => Box::new(MakeLargerAction),
            Action::MakeSmaller => Box::new(MakeSmallerAction),
            Action::MaximizeHeight => Box::new(MaximizeHeightAction),
            _ => Box::new(NoOpAction),
        }
    }
}

/// Moves the currently focused window to a new position and/or size.
#[tauri::command]
fn move_window(action: Option<Action>) -> Result<(), String> {
    println!("move_window called with action: {:?}", action);

    #[cfg(target_os = "windows")]
    {
        use std::ptr;
        use winapi::shared::windef::HWND;
        use winapi::um::winuser::SWP_FRAMECHANGED;
        use winapi::um::winuser::{GetForegroundWindow, GetWindowRect, SetWindowPos};

        unsafe {
            let hwnd: HWND = GetForegroundWindow();
            if hwnd.is_null() {
                return Err("No focused window found".to_string());
            }

            // Get current window position and size
            let mut rect = std::mem::zeroed();
            if GetWindowRect(hwnd, &mut rect) == 0 {
                return Err("Failed to get window position".to_string());
            }

            // Update monitor information
            get_monitor_info();
            let monitors = MONITORS.lock().unwrap().clone();

            let window_center_x = (rect.left + rect.right) / 2;
            let window_center_y = (rect.top + rect.bottom) / 2;

            // Find current monitor
            let current_monitor = monitors
                .iter()
                .position(|m| {
                    window_center_x >= m.left
                        && window_center_x < m.right
                        && window_center_y >= m.top
                        && window_center_y < m.bottom
                })
                .unwrap_or(0);

            let window_info = WindowInfo {
                current_width: rect.right - rect.left,
                current_height: rect.bottom - rect.top,
                center_x: window_center_x,
                center_y: window_center_y,
            };

            // Get the action handler
            let action_ref = action.as_ref().unwrap_or(&Action::None);
            let handler = action_ref.get_handler();

            // Determine target monitor
            let target_monitor = handler.get_target_monitor(current_monitor, &monitors);

            // Create action context
            let context = ActionContext {
                current_monitor,
                target_monitor,
                monitors,
                window_info,
            };

            // Calculate new position and size
            let (new_width, new_height, new_x, new_y) =
                handler.calculate_position(&context, hwnd)?;

            // Apply the changes
            let result = SetWindowPos(
                hwnd,
                ptr::null_mut(),
                new_x,
                new_y,
                new_width,
                new_height,
                SWP_FRAMECHANGED,
            );

            if result != 0 {
                println!(
                    "Window moved successfully to {}x{} at ({},{})",
                    new_width, new_height, new_x, new_y
                );
                Ok(())
            } else {
                Err("Failed to move/resize window".to_string())
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

#[tauri::command]
fn toggle_window(app: AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        match window.is_visible() {
            Ok(true) => {
                let _ = window.hide();
            },
            Ok(false) => {
                let _ = window.show();
                let _ = window.set_focus();
            },
            Err(_) => {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
    }
}

pub fn run() {
    // Create the shared shortcut config ONCE
    let shortcuts_config = Arc::new(Mutex::new(
        shortcuts::ShortcutsConfig::load().unwrap_or_default(),
    ));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(ShortcutManager::new(shortcuts_config.clone()))
        .invoke_handler(tauri::generate_handler![move_window, update_shortcut, toggle_window])
        .on_window_event(|window, event| {
            match event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    // Prevent close and hide window instead
                    api.prevent_close();
                    let _ = window.hide();
                }
                _ => {}
            }
        })
        .setup({
            let shortcuts_config = shortcuts_config.clone();
            move |app| {
                // Create tray menu
                let open_settings = MenuItem::with_id(app, "open_settings", "Open Settings", true, None::<&str>)?;
                let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
                let menu = Menu::with_items(app, &[&open_settings, &quit])?;

                // Create system tray
                let tray_icon = include_bytes!("../icons/32x32.png");
                let image = tauri::image::Image::from_bytes(tray_icon)?;
                let _tray = TrayIconBuilder::with_id("main_tray")
                    .icon(image)
                    .menu(&menu)
                    .show_menu_on_left_click(false)
                    .on_menu_event(move |app, event| match event.id.as_ref() {
                        "open_settings" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    })
                    .build(app)?;

                // Hide window on startup (start minimized to tray)
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }

                #[cfg(desktop)]
                {
                    // Register all shortcuts
                    if let Err(e) = shortcuts::register_shortcuts(app, shortcuts_config.clone()) {
                        eprintln!("Failed to register shortcuts: {}", e);
                        return Err(e.into());
                    }
                }
                Ok(())
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
