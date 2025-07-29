use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::sync::Mutex;
use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

// Helper function to translate symbol keys to their system names
fn translate_key_symbols(shortcut: &str) -> String {
    shortcut
        .replace("=", "Equal")
        .replace("-", "Minus")
        .replace("[", "BracketLeft")
        .replace("]", "BracketRight")
        .replace(";", "Semicolon")
        .replace("'", "Quote")
        .replace(",", "Comma")
        .replace(".", "Period")
        .replace("/", "Slash")
        .replace("\\", "Backslash")
        .replace("`", "Backquote")
}

// Helper function to normalize shortcut strings for comparison
fn normalize_shortcut(shortcut: &str) -> String {
    let canonical_order = ["shift", "control", "alt", "meta", "super", "cmd", "win"];
    let mut parts: Vec<&str> = shortcut.split('+').collect();
    if parts.len() <= 1 {
        return shortcut.to_lowercase();
    }
    let key = parts.pop().unwrap().to_lowercase();
    let mut key_norm = key.clone();
    // Normalize key names
    if let Some(stripped) = key.strip_prefix("key") {
        key_norm = stripped.to_lowercase();
    } else if let Some(stripped) = key.strip_prefix("arrow") {
        key_norm = format!("arrow{}", stripped);
    }
    let mut modifiers: Vec<String> = parts.iter().map(|s| s.trim().to_lowercase()).collect();
    let mut ordered_mods = Vec::new();
    for &canon in &canonical_order {
        if let Some(idx) = modifiers.iter().position(|m| m == canon) {
            ordered_mods.push(canon);
            modifiers.remove(idx);
        }
    }
    // Add any remaining modifiers (rare/unexpected)
    ordered_mods.extend(modifiers.iter().map(|s| s.as_str()));
    let mut normalized = ordered_mods.join("+");
    normalized.push('+');
    normalized.push_str(&key_norm);
    normalized
}

// Global state to store the current shortcuts
use std::sync::Arc;

pub struct ShortcutManager {
    pub shortcuts: Arc<Mutex<ShortcutsConfig>>,
}

impl ShortcutManager {
    pub fn new(shortcuts: Arc<Mutex<ShortcutsConfig>>) -> Self {
        Self {
            shortcuts,
        }
    }
}

#[tauri::command]
pub async fn update_shortcut(
    app_handle: AppHandle,
    shortcut_id: String,
    new_shortcut: String,
    state: tauri::State<'_, ShortcutManager>,
) -> Result<(), String> {
    println!("Updating shortcut {} to {}", shortcut_id, new_shortcut);

    // Get the current shortcuts (shared Arc)
    let mut shortcuts = state.shortcuts.lock().map_err(|e| e.to_string())?;

    // Find the shortcut to update
    if let Some(shortcut_cfg) = shortcuts.shortcuts.get_mut(&shortcut_id) {
        let old_shortcut_str = shortcut_cfg.default_shortcut.clone();

        // Normalize both shortcuts for comparison
        let normalized_old = normalize_shortcut(&translate_key_symbols(&old_shortcut_str));
        let normalized_new = normalize_shortcut(&translate_key_symbols(&new_shortcut));

        println!("Old shortcut: {}", normalized_old);
        println!("New shortcut: {}", normalized_new);

        // Only proceed if the shortcut is actually changing
        if normalized_old == normalized_new {
            println!(
                "Shortcut {} is already set to {}",
                shortcut_id, new_shortcut
            );
            return Ok(());
        }

        // Update the shortcut in our config first
        shortcut_cfg.default_shortcut = new_shortcut.clone();

        // Parse the new shortcut first to validate it
        let translated_new_shortcut = translate_key_symbols(&new_shortcut);
        let new_shortcut_parsed = match Shortcut::from_str(&translated_new_shortcut) {
            Ok(s) => s,
            Err(e) => {
                let error = format!("Invalid shortcut format: {} - {}", new_shortcut, e);
                println!("{}", error);
                return Err(error);
            }
        };

        // Try to register the new shortcut first
        println!(
            "Registering new global shortcut: {} for {}",
            new_shortcut, shortcut_id
        );
        if let Err(e) = app_handle.global_shortcut().register(new_shortcut_parsed) {
            let error = format!("Failed to register new shortcut: {}", e);
            println!("{}", error);
            return Err(error);
        }

        println!("Successfully registered new shortcut: {}", new_shortcut);

        // Try to unregister all possible variations of the old shortcut
        let old_variants = [&old_shortcut_str, &normalize_shortcut(&old_shortcut_str)];
        for old_variant in old_variants.iter() {
            match Shortcut::from_str(old_variant) {
                Ok(old_shortcut) => {
                    if let Err(e) = app_handle.global_shortcut().unregister(old_shortcut) {
                        // Don't fail if unregister fails, just log it
                        println!(
                            "Note: Could not unregister old shortcut ({}): {}",
                            old_shortcut_str, e
                        );
                    } else {
                        println!("Successfully unregistered old shortcut: {}", old_variant);
                        break;
                    }
                }
                Err(e) => {
                    println!(
                        "Note: Could not parse old shortcut ({}): {}",
                        old_variant, e
                    );
                }
            }
        }

        println!(
            "Successfully updated shortcut {} from {} to {}",
            shortcut_id, old_shortcut_str, new_shortcut
        );

        // Save updated shortcuts config to shortcuts.json
        // Use the same path logic as ShortcutsConfig::load()
        let frontend_dir = std::env::current_dir().unwrap().parent().unwrap().join("src");
        let config_path = frontend_dir.join("shortcuts.json");
        let json = serde_json::to_string_pretty(&*shortcuts)
            .map_err(|e| format!("Failed to serialize shortcuts: {}", e))?;
        std::fs::write(&config_path, json)
            .map_err(|e| format!("Failed to write to {:?}: {}", config_path, e))?;
        println!("Shortcuts saved to {:?}", config_path);

        Ok(())
    } else {
        Err(format!("Shortcut {} not found", shortcut_id))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShortcutConfig {
    pub name: String,
    #[serde(rename = "defaultShortcut")]
    pub default_shortcut: String,
}

impl Default for ShortcutConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            default_shortcut: String::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortcutsConfig {
    pub shortcuts: HashMap<String, ShortcutConfig>,
}

impl Default for ShortcutsConfig {
    fn default() -> Self {
        let mut shortcuts = HashMap::new();

        // Add default shortcuts
        shortcuts.insert(
            "moveMonitorLeft".to_string(),
            ShortcutConfig {
                name: "Move to Left Monitor".to_string(),
                default_shortcut: "Shift+Control+Alt+ArrowLeft".to_string(),
            },
        );

        shortcuts.insert(
            "moveMonitorRight".to_string(),
            ShortcutConfig {
                name: "Move to Right Monitor".to_string(),
                default_shortcut: "Shift+Control+Alt+ArrowRight".to_string(),
            },
        );

        shortcuts.insert(
            "maximizeWindow".to_string(),
            ShortcutConfig {
                name: "Maximize Window".to_string(),
                default_shortcut: "Control+Alt+Enter".to_string(),
            },
        );

        shortcuts.insert(
            "almostMaximizeWindow".to_string(),
            ShortcutConfig {
                name: "Almost Maximize Window".to_string(),
                default_shortcut: "Shift+Control+Alt+Enter".to_string(),
            },
        );

        Self { shortcuts }
    }
}

impl ShortcutsConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        println!("Loading shortcuts from bundled resource...");
        
        // Try to load from bundled resource first (production)
        match Self::load_from_resource() {
            Ok(config) => {
                println!("Successfully loaded shortcuts from bundled resource: {} shortcuts", config.shortcuts.len());
                return Ok(config);
            }
            Err(e) => {
                println!("Failed to load from bundled resource: {}", e);
                println!("Falling back to file system (development mode)...");
            }
        }
        
        // Fallback to file system (development)
        Self::load_from_file()
    }
    
    fn load_from_resource() -> Result<Self, Box<dyn std::error::Error>> {
        // In production, the file is bundled as a resource
        let config_content = include_str!("../../src/shortcuts.json");
        
        let parsed: ShortcutsConfig = serde_json::from_str(config_content)
            .map_err(|e| format!("Failed to parse bundled shortcuts.json: {}", e))?;
            
        Ok(parsed)
    }
    
    fn load_from_file() -> Result<Self, Box<dyn std::error::Error>> {
        // Get the path to the shortcuts.json file in the frontend directory
        let frontend_dir = std::env::current_dir()?.parent().unwrap().join("src");
        let config_path = frontend_dir.join("shortcuts.json");

        println!("Loading shortcuts from: {:?}", config_path);

        let config_content = fs::read_to_string(&config_path).map_err(|e| {
            format!(
                "Failed to read shortcuts.json: {}\nPath: {:?}",
                e, config_path
            )
        })?;

        println!("Raw JSON content: {}", config_content);

        let parsed: ShortcutsConfig = serde_json::from_str(&config_content)
            .map_err(|e| format!("Failed to parse shortcuts.json: {}", e))?;

        println!("Successfully parsed {} shortcuts", parsed.shortcuts.len());

        Ok(parsed)
    }


    pub fn get_all_shortcuts(&self) -> &HashMap<String, ShortcutConfig> {
        &self.shortcuts
    }
}


pub fn register_shortcuts(
    app: &mut tauri::App,
    shortcuts_state: Arc<Mutex<ShortcutsConfig>>,
) -> Result<(), String> {
    use tauri_plugin_global_shortcut::GlobalShortcutExt;

    // Debug: Print all loaded shortcuts
    {
        let config = shortcuts_state.lock().unwrap();
        println!("Loaded shortcuts from config:");
        for (id, shortcut_cfg) in config.get_all_shortcuts() {
            println!(
                "  {}: {} = {}",
                id, shortcut_cfg.name, shortcut_cfg.default_shortcut
            );
        }
    } // lock released here
    let app_handle = app.handle();

    // Register the plugin
    let handler_shortcuts_state = shortcuts_state.clone();
    let plugin = tauri_plugin_global_shortcut::Builder::new()
        .with_handler(move |_app, shortcut, event| {
            use super::Action;
            use tauri_plugin_global_shortcut::ShortcutState;

            let shortcut_str = shortcut.to_string();
            let normalized_shortcut = normalize_shortcut(&shortcut_str);
            
            println!(
                "Shortcut event: {} (normalized: {}) - {:?}",
                shortcut_str, normalized_shortcut, event.state()
            );

            if event.state() == ShortcutState::Released {
                // Lock and get the latest shortcuts mapping
                let config = handler_shortcuts_state.lock().unwrap();
                
                // Create a mapping of normalized shortcuts to actions
                let mut shortcut_to_action = std::collections::HashMap::new();
                
                for (id, shortcut_cfg) in config.get_all_shortcuts() {
                    let normalized = normalize_shortcut(&translate_key_symbols(&shortcut_cfg.default_shortcut));
                    let action = match id.as_str() {
                        "moveMonitorLeft" => Some(Action::MoveLeft),
                        "moveMonitorRight" => Some(Action::MoveRight),
                        "maximizeWindow" => Some(Action::Maximize { gutter: 0 }),
                        "almostMaximizeWindow" => Some(Action::Maximize { gutter: 32 }),
                        "leftHalf" => Some(Action::LeftHalf),
                        "rightHalf" => Some(Action::RightHalf),
                        "topHalf" => Some(Action::TopHalf),
                        "bottomHalf" => Some(Action::BottomHalf),
                        "topLeft" => Some(Action::TopLeft),
                        "topRight" => Some(Action::TopRight),
                        "bottomLeft" => Some(Action::BottomLeft),
                        "bottomRight" => Some(Action::BottomRight),
                        "firstThird" => Some(Action::FirstThird),
                        "centerThird" => Some(Action::CenterThird),
                        "lastThird" => Some(Action::LastThird),
                        "firstTwoThirds" => Some(Action::FirstTwoThirds),
                        "lastTwoThirds" => Some(Action::LastTwoThirds),
                        "center" => Some(Action::Center),
                        "makeLarger" => Some(Action::MakeLarger),
                        "makeSmaller" => Some(Action::MakeSmaller),
                        "maximizeHeight" => Some(Action::MaximizeHeight),
                        _ => None,
                    };
                    
                    if let Some(action) = action {
                        shortcut_to_action.insert(normalized, (id.clone(), action));
                    }
                }

                
                println!("Available shortcuts: {:?}", shortcut_to_action.keys().collect::<Vec<_>>());

                // Look up the action for this shortcut
                if let Some((shortcut_id, action)) = shortcut_to_action.get(&normalized_shortcut) {
                    println!("Triggering {} action for shortcut: {}", shortcut_id, shortcut_str);
                    let _ = super::move_window(Some(action.clone()));
                } else {
                    println!("No action found for shortcut: {} (normalized: {})", shortcut_str, normalized_shortcut);
                }
            }
        })
        .build();

    // Register the plugin and handle the result
    if let Err(e) = app_handle.plugin(plugin) {
        return Err(format!("Failed to register global shortcut plugin: {}", e));
    }

    // Register individual shortcuts
    {
        let config = shortcuts_state.lock().unwrap();
        let mut registered_shortcuts = std::collections::HashSet::new();
        let mut errors = Vec::new();
        
        for (id, shortcut_cfg) in config.get_all_shortcuts() {
            let translated_shortcut = translate_key_symbols(&shortcut_cfg.default_shortcut);
            match Shortcut::from_str(&translated_shortcut) {
                Ok(shortcut) => {
                    let shortcut_str = shortcut.to_string();
                    
                    // Check if this exact shortcut was already registered
                    if registered_shortcuts.contains(&shortcut_str) {
                        let warning = format!(
                            "Warning: Duplicate shortcut '{}' for '{}' - skipping",
                            shortcut_str, id
                        );
                        println!("{}", warning);
                        continue;
                    }
                    
                    println!(
                        "Registering global shortcut: {} for {}",
                        shortcut_cfg.default_shortcut, id
                    );
                    
                    // Use catch_unwind to handle any panics during registration
                    let registration_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        app_handle.global_shortcut().register(shortcut)
                    }));
                    
                    match registration_result {
                        Ok(Ok(_)) => {
                            registered_shortcuts.insert(shortcut_str);
                        }
                        Ok(Err(e)) => {
                            let error_msg = format!(
                                "Failed to register shortcut '{}' for '{}': {}",
                                shortcut_cfg.default_shortcut, id, e
                            );
                            eprintln!("{}", error_msg);
                            errors.push(error_msg);
                        }
                        Err(_) => {
                            let error_msg = format!(
                                "Panic while registering shortcut '{}' for '{}' (likely already registered by another application)",
                                shortcut_cfg.default_shortcut, id
                            );
                            eprintln!("{}", error_msg);
                            errors.push(error_msg);
                        }
                    }
                }
                Err(e) => {
                    let error_msg = format!(
                        "Invalid shortcut format for '{}' (value: '{}'): {}",
                        id, shortcut_cfg.default_shortcut, e
                    );
                    eprintln!("{}", error_msg);
                    errors.push(error_msg);
                }
            }
        }
        
        // Handle any registration errors
        if !errors.is_empty() {
            let error_message = format!(
                "{} shortcuts couldn't be registered and have been disabled.\n\n{}",
                errors.len(),
                errors.join("\n")
            );
            
            // Log the error
            eprintln!("Warning: {}", error_message);
        }
        
        // If we didn't register any shortcuts at all, that's an error
        if registered_shortcuts.is_empty() {
            let error_msg = "Failed to register any shortcuts. All shortcut registrations failed.";
            return Err(error_msg.to_string());
        }
    }

    Ok(())
}
