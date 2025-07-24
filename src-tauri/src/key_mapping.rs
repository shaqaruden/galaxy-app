use std::collections::HashMap;

lazy_static::lazy_static! {
    pub static ref KEY_NAME_MAPPING: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        
        // Symbol keys
        map.insert("BracketLeft", "[");
        map.insert("BracketRight", "]");
        map.insert("Backslash", "\\");
        map.insert("Semicolon", ";");
        map.insert("Quote", "'");
        map.insert("Comma", ",");
        map.insert("Period", ".");
        map.insert("Slash", "/");
        map.insert("Backquote", "`");
        map.insert("Minus", "-");
        map.insert("Equal", "=");
        
        // Number keys
        map.insert("Digit0", "0");
        map.insert("Digit1", "1");
        map.insert("Digit2", "2");
        map.insert("Digit3", "3");
        map.insert("Digit4", "4");
        map.insert("Digit5", "5");
        map.insert("Digit6", "6");
        map.insert("Digit7", "7");
        map.insert("Digit8", "8");
        map.insert("Digit9", "9");
        
        // Function keys
        map.insert("F1", "F1");
        map.insert("F2", "F2");
        map.insert("F3", "F3");
        map.insert("F4", "F4");
        map.insert("F5", "F5");
        map.insert("F6", "F6");
        map.insert("F7", "F7");
        map.insert("F8", "F8");
        map.insert("F9", "F9");
        map.insert("F10", "F10");
        map.insert("F11", "F11");
        map.insert("F12", "F12");
        
        // Arrow keys
        map.insert("ArrowUp", "ArrowUp");
        map.insert("ArrowDown", "ArrowDown");
        map.insert("ArrowLeft", "ArrowLeft");
        map.insert("ArrowRight", "ArrowRight");
        
        // Navigation keys
        map.insert("Home", "Home");
        map.insert("End", "End");
        map.insert("PageUp", "PageUp");
        map.insert("PageDown", "PageDown");
        map.insert("Insert", "Insert");
        map.insert("Delete", "Delete");
        map.insert("Backspace", "Backspace");
        map.insert("Tab", "Tab");
        map.insert("Enter", "Enter");
        map.insert("Space", "Space");
        map.insert("Escape", "Escape");
        
        // Numpad keys
        map.insert("Numpad0", "Numpad0");
        map.insert("Numpad1", "Numpad1");
        map.insert("Numpad2", "Numpad2");
        map.insert("Numpad3", "Numpad3");
        map.insert("Numpad4", "Numpad4");
        map.insert("Numpad5", "Numpad5");
        map.insert("Numpad6", "Numpad6");
        map.insert("Numpad7", "Numpad7");
        map.insert("Numpad8", "Numpad8");
        map.insert("Numpad9", "Numpad9");
        map.insert("NumpadMultiply", "NumpadMultiply");
        map.insert("NumpadAdd", "NumpadAdd");
        map.insert("NumpadSubtract", "NumpadSubtract");
        map.insert("NumpadDecimal", "NumpadDecimal");
        map.insert("NumpadDivide", "NumpadDivide");
        map.insert("NumpadEnter", "NumpadEnter");
        
        // Special characters (shift+number combinations)
        map.insert("Exclamation", "!");
        map.insert("At", "@");
        map.insert("Hash", "#");
        map.insert("Dollar", "$");
        map.insert("Percent", "%");
        map.insert("Caret", "^");
        map.insert("Ampersand", "&");
        map.insert("Asterisk", "*");
        map.insert("LeftParen", "(");
        map.insert("RightParen", ")");
        map.insert("Underscore", "_");
        map.insert("Plus", "+");
        map.insert("LeftCurlyBracket", "{");
        map.insert("RightCurlyBracket", "}");
        map.insert("Pipe", "|");
        map.insert("Colon", ":");
        map.insert("DoubleQuote", "\"");
        map.insert("LeftAngleBracket", "<");
        map.insert("RightAngleBracket", ">");
        map.insert("Question", "?");
        map.insert("Tilde", "~");
        map.insert("Backtick", "`");
        
        map
    };
}

/// Translates frontend key names to registration key values
pub fn translate_key_name(key_name: &str) -> String {
    KEY_NAME_MAPPING
        .get(key_name)
        .map(|&s| s.to_string())
        .unwrap_or_else(|| key_name.to_string())
}

/// Translates a complete shortcut string from frontend format to registration format
pub fn translate_shortcut(shortcut: &str) -> String {
    let parts: Vec<&str> = shortcut.split('+').collect();
    if parts.len() <= 1 {
        return translate_key_name(shortcut);
    }
    
    let mut translated_parts = Vec::new();
    for part in parts.iter().take(parts.len() - 1) {
        translated_parts.push(part.to_lowercase());
    }
    
    let key = parts.last().unwrap();
    let translated_key = translate_key_name(key);
    
    translated_parts.push(translated_key);
    translated_parts.join("+")
}
