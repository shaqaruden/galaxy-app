#[cfg(test)]
mod tests {
    use super::super::key_mapping;

    #[test]
    fn test_symbol_key_translation() {
        assert_eq!(key_mapping::translate_key_name("BracketLeft"), "[");
        assert_eq!(key_mapping::translate_key_name("BracketRight"), "]");
        assert_eq!(key_mapping::translate_key_name("Semicolon"), ";");
        assert_eq!(key_mapping::translate_key_name("Quote"), "'");
        assert_eq!(key_mapping::translate_key_name("Comma"), ",");
        assert_eq!(key_mapping::translate_key_name("Period"), ".");
        assert_eq!(key_mapping::translate_key_name("Slash"), "/");
        assert_eq!(key_mapping::translate_key_name("Backquote"), "`");
        assert_eq!(key_mapping::translate_key_name("Minus"), "-");
        assert_eq!(key_mapping::translate_key_name("Equal"), "=");
    }

    #[test]
    fn test_shortcut_translation() {
        assert_eq!(key_mapping::translate_shortcut("Control+Alt+BracketLeft"), "control+alt+[");
        assert_eq!(key_mapping::translate_shortcut("Shift+Control+Semicolon"), "shift+control+;");
        assert_eq!(key_mapping::translate_shortcut("Alt+Quote"), "alt+'");
        assert_eq!(key_mapping::translate_shortcut("Control+Comma"), "control+,");
        assert_eq!(key_mapping::translate_shortcut("Shift+Period"), "shift+.");
    }

    #[test]
    fn test_number_key_translation() {
        assert_eq!(key_mapping::translate_key_name("Digit1"), "1");
        assert_eq!(key_mapping::translate_key_name("Digit9"), "9");
        assert_eq!(key_mapping::translate_key_name("Digit0"), "0");
    }

    #[test]
    fn test_function_key_translation() {
        assert_eq!(key_mapping::translate_key_name("F1"), "F1");
        assert_eq!(key_mapping::translate_key_name("F12"), "F12");
    }

    #[test]
    fn test_arrow_key_translation() {
        assert_eq!(key_mapping::translate_key_name("ArrowLeft"), "ArrowLeft");
        assert_eq!(key_mapping::translate_key_name("ArrowRight"), "ArrowRight");
    }

    #[test]
    fn test_unknown_key_passthrough() {
        assert_eq!(key_mapping::translate_key_name("UnknownKey"), "UnknownKey");
        assert_eq!(key_mapping::translate_shortcut("Control+UnknownKey"), "control+UnknownKey");
    }
}
