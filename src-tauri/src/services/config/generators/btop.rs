use regex::Regex;
use super::ConfigGenerator;
use serde_json::{json, Map, Value};
// ... existing code ...
pub struct BtopGenerator;

unsafe impl Send for BtopGenerator {}
unsafe impl Sync for BtopGenerator {}

impl ConfigGenerator for BtopGenerator {
    fn get_app_name(&self) -> &'static str {
        "btop"
    }

    fn get_file_name(&self) -> &'static str {
        "btop.theme"
    }

    fn generate_config(&self, theme_data: &Value) -> Result<String, String> {
        let empty_obj = json!({});
        let btop = theme_data.get("btop").unwrap_or(&empty_obj);
        let colors = btop.get("colors").unwrap_or(&empty_obj);

        // gets nested color values from sections
        let get_color = |section: &str, field: &str, default: &str| -> String {
            colors.get(section)
                .and_then(|s| s.get(field))
                .and_then(|v| v.as_str())
                .unwrap_or(default)
                .to_string()
        };

        // Extract colors with defaults from template
        let main_bg = get_color("basic", "main_bg", "");
        let main_fg = get_color("basic", "main_fg", "#EAEAEA");
        let title = get_color("basic", "title", "#8a8a8d");
        let hi_fg = get_color("basic", "hi_fg", "#f59e0b");
        let selected_bg = get_color("basic", "selected_bg", "#f59e0b");
        let selected_fg = get_color("basic", "selected_fg", "#EAEAEA");
        let inactive_fg = get_color("basic", "inactive_fg", "#333333");
        let proc_misc = get_color("basic", "proc_misc", "#8a8a8d");
        let cpu_box = get_color("boxes", "cpu_box", "#8a8a8d");
        let mem_box = get_color("boxes", "mem_box", "#8a8a8d");
        let net_box = get_color("boxes", "net_box", "#8a8a8d");
        let proc_box = get_color("boxes", "proc_box", "#8a8a8d");
        let div_line = get_color("boxes", "div_line", "#8a8a8d");
        let temp_start = get_color("temperature", "temp_start", "#8a8a8d");
        let temp_mid = get_color("temperature", "temp_mid", "#f59e0b");
        let temp_end = get_color("temperature", "temp_end", "#b91c1c");
        let cpu_start = get_color("cpu", "cpu_start", "#8a8a8d");
        let cpu_mid = get_color("cpu", "cpu_mid", "#f59e0b");
        let cpu_end = get_color("cpu", "cpu_end", "#b91c1c");
        let free_start = get_color("memory", "free_start", "#8a8a8d");
        let free_mid = get_color("memory", "free_mid", "#f59e0b");
        let free_end = get_color("memory", "free_end", "#b91c1c");
        let cached_start = get_color("memory", "cached_start", "#8a8a8d");
        let cached_mid = get_color("memory", "cached_mid", "#f59e0b");
        let cached_end = get_color("memory", "cached_end", "#b91c1c");
        let available_start = get_color("memory", "available_start", "#8a8a8d");
        let available_mid = get_color("memory", "available_mid", "#f59e0b");
        let available_end = get_color("memory", "available_end", "#b91c1c");
        let used_start = get_color("memory", "used_start", "#8a8a8d");
        let used_mid = get_color("memory", "used_mid", "#f59e0b");
        let used_end = get_color("memory", "used_end", "#b91c1c");
        let download_start = get_color("network", "download_start", "#8a8a8d");
        let download_mid = get_color("network", "download_mid", "#f59e0b");
        let download_end = get_color("network", "download_end", "#b91c1c");
        let upload_start = get_color("network", "upload_start", "#8a8a8d");
        let upload_mid = get_color("network", "upload_mid", "#f59e0b");
        let upload_end = get_color("network", "upload_end", "#b91c1c");

        Ok(format!(
            r#"# ────────────────────────────────────────────────────────────
# Omarchy Custom Theme for btop
# Generated with Omarchist
# ────────────────────────────────────────────────────────────

# Main background, empty for terminal default, need to be empty if you want transparent background
theme[main_bg]="{main_bg}"

# Main text color
theme[main_fg]="{main_fg}"

# Title color for boxes
theme[title]="{title}"

# Highlight color for keyboard shortcuts
theme[hi_fg]="{hi_fg}"

# Background color of selected item in processes box
theme[selected_bg]="{selected_bg}"

# Foreground color of selected item in processes box
theme[selected_fg]="{selected_fg}"

# Color of inactive/disabled text
theme[inactive_fg]="{inactive_fg}"

# Misc colors for processes box including mini cpu graphs, details memory graph and details status text
theme[proc_misc]="{proc_misc}"

# Cpu box outline color
theme[cpu_box]="{cpu_box}"

# Memory/disks box outline color
theme[mem_box]="{mem_box}"

# Net up/down box outline color
theme[net_box]="{net_box}"

# Processes box outline color
theme[proc_box]="{proc_box}"

# Box divider line and small boxes line color
theme[div_line]="{div_line}"

# Temperature graph colors
theme[temp_start]="{temp_start}"
theme[temp_mid]="{temp_mid}"
theme[temp_end]="{temp_end}"

# CPU graph colors
theme[cpu_start]="{cpu_start}"
theme[cpu_mid]="{cpu_mid}"
theme[cpu_end]="{cpu_end}"

# Mem/Disk free meter
theme[free_start]="{free_start}"
theme[free_mid]="{free_mid}"
theme[free_end]="{free_end}"

# Mem/Disk cached meter
theme[cached_start]="{cached_start}"
theme[cached_mid]="{cached_mid}"
theme[cached_end]="{cached_end}"

# Mem/Disk available meter
theme[available_start]="{available_start}"
theme[available_mid]="{available_mid}"
theme[available_end]="{available_end}"

# Mem/Disk used meter
theme[used_start]="{used_start}"
theme[used_mid]="{used_mid}"
theme[used_end]="{used_end}"

# Download graph colors
theme[download_start]="{download_start}"
theme[download_mid]="{download_mid}"
theme[download_end]="{download_end}"

# Upload graph colors
theme[upload_start]="{upload_start}"
theme[upload_mid]="{upload_mid}"
theme[upload_end]="{upload_end}"
"#
        ))
    }

    fn get_config_schema(&self) -> Value {
        let mut properties = serde_json::Map::new();

        // Basic Colors Section
        let mut basic_properties = serde_json::Map::new();
        // Defaults mirror src-tauri/src/data/template/btop.theme
        basic_properties.insert(
            "main_bg".to_string(),
            json!({"type": "string", "format": "color", "title": "Main Background", "default": ""}),
        );
        basic_properties.insert("main_fg".to_string(), json!({"type": "string", "format": "color", "title": "Main Foreground", "default": "#EAEAEA"}));
        basic_properties.insert("title".to_string(), json!({"type": "string", "format": "color", "title": "Title Color", "default": "#8a8a8d"}));
        basic_properties.insert("hi_fg".to_string(), json!({"type": "string", "format": "color", "title": "Highlight Color", "default": "#f59e0b"}));
        basic_properties.insert("selected_bg".to_string(), json!({"type": "string", "format": "color", "title": "Selected Background", "default": "#f59e0b"}));
        basic_properties.insert("selected_fg".to_string(), json!({"type": "string", "format": "color", "title": "Selected Foreground", "default": "#EAEAEA"}));
        basic_properties.insert("inactive_fg".to_string(), json!({"type": "string", "format": "color", "title": "Inactive Text", "default": "#333333"}));
        basic_properties.insert("proc_misc".to_string(), json!({"type": "string", "format": "color", "title": "Process Misc", "default": "#8a8a8d"}));
        properties.insert(
            "basic".to_string(),
            json!({"type": "object", "title": "Basic Colors", "properties": basic_properties}),
        );

        // Box Outlines Section
        let mut box_properties = serde_json::Map::new();
        box_properties.insert("cpu_box".to_string(), json!({"type": "string", "format": "color", "title": "CPU Box Outline", "default": "#8a8a8d"}));
        box_properties.insert("mem_box".to_string(), json!({"type": "string", "format": "color", "title": "Memory Box Outline", "default": "#8a8a8d"}));
        box_properties.insert("net_box".to_string(), json!({"type": "string", "format": "color", "title": "Network Box Outline", "default": "#8a8a8d"}));
        box_properties.insert("proc_box".to_string(), json!({"type": "string", "format": "color", "title": "Process Box Outline", "default": "#8a8a8d"}));
        box_properties.insert("div_line".to_string(), json!({"type": "string", "format": "color", "title": "Divider Line", "default": "#8a8a8d"}));
        properties.insert(
            "boxes".to_string(),
            json!({"type": "object", "title": "Box Outlines", "properties": box_properties}),
        );

        // Temperature Graph Section
        let mut temp_properties = serde_json::Map::new();
        temp_properties.insert("temp_start".to_string(), json!({"type": "string", "format": "color", "title": "Start Color", "default": "#8a8a8d"}));
        temp_properties.insert("temp_mid".to_string(), json!({"type": "string", "format": "color", "title": "Mid Color", "default": "#f59e0b"}));
        temp_properties.insert("temp_end".to_string(), json!({"type": "string", "format": "color", "title": "End Color", "default": "#b91c1c"}));
        properties.insert(
            "temperature".to_string(),
            json!({"type": "object", "title": "Temperature Graph", "properties": temp_properties}),
        );

        // CPU Graph Section
        let mut cpu_properties = serde_json::Map::new();
        cpu_properties.insert("cpu_start".to_string(), json!({"type": "string", "format": "color", "title": "Start Color", "default": "#8a8a8d"}));
        cpu_properties.insert("cpu_mid".to_string(), json!({"type": "string", "format": "color", "title": "Mid Color", "default": "#f59e0b"}));
        cpu_properties.insert("cpu_end".to_string(), json!({"type": "string", "format": "color", "title": "End Color", "default": "#b91c1c"}));
        properties.insert(
            "cpu".to_string(),
            json!({"type": "object", "title": "CPU Graph", "properties": cpu_properties}),
        );

        // Memory Meters Section
        let mut memory_properties = serde_json::Map::new();
        memory_properties.insert("free_start".to_string(), json!({"type": "string", "format": "color", "title": "Free Start", "default": "#8a8a8d"}));
        memory_properties.insert(
            "free_mid".to_string(),
            json!({"type": "string", "format": "color", "title": "Free Mid", "default": "#f59e0b"}),
        );
        memory_properties.insert(
            "free_end".to_string(),
            json!({"type": "string", "format": "color", "title": "Free End", "default": "#b91c1c"}),
        );
        memory_properties.insert("cached_start".to_string(), json!({"type": "string", "format": "color", "title": "Cached Start", "default": "#8a8a8d"}));
        memory_properties.insert("cached_mid".to_string(), json!({"type": "string", "format": "color", "title": "Cached Mid", "default": "#f59e0b"}));
        memory_properties.insert("cached_end".to_string(), json!({"type": "string", "format": "color", "title": "Cached End", "default": "#b91c1c"}));
        memory_properties.insert("available_start".to_string(), json!({"type": "string", "format": "color", "title": "Available Start", "default": "#8a8a8d"}));
        memory_properties.insert("available_mid".to_string(), json!({"type": "string", "format": "color", "title": "Available Mid", "default": "#f59e0b"}));
        memory_properties.insert("available_end".to_string(), json!({"type": "string", "format": "color", "title": "Available End", "default": "#b91c1c"}));
        memory_properties.insert("used_start".to_string(), json!({"type": "string", "format": "color", "title": "Used Start", "default": "#8a8a8d"}));
        memory_properties.insert(
            "used_mid".to_string(),
            json!({"type": "string", "format": "color", "title": "Used Mid", "default": "#f59e0b"}),
        );
        memory_properties.insert(
            "used_end".to_string(),
            json!({"type": "string", "format": "color", "title": "Used End", "default": "#b91c1c"}),
        );
        properties.insert(
            "memory".to_string(),
            json!({"type": "object", "title": "Memory Meters", "properties": memory_properties}),
        );

        // Network Meters Section
        let mut network_properties = serde_json::Map::new();
        network_properties.insert("download_start".to_string(), json!({"type": "string", "format": "color", "title": "Download Start", "default": "#8a8a8d"}));
        network_properties.insert("download_mid".to_string(), json!({"type": "string", "format": "color", "title": "Download Mid", "default": "#f59e0b"}));
        network_properties.insert("download_end".to_string(), json!({"type": "string", "format": "color", "title": "Download End", "default": "#b91c1c"}));
        network_properties.insert("upload_start".to_string(), json!({"type": "string", "format": "color", "title": "Upload Start", "default": "#8a8a8d"}));
        network_properties.insert("upload_mid".to_string(), json!({"type": "string", "format": "color", "title": "Upload Mid", "default": "#f59e0b"}));
        network_properties.insert("upload_end".to_string(), json!({"type": "string", "format": "color", "title": "Upload End", "default": "#b91c1c"}));
        properties.insert(
            "network".to_string(),
            json!({"type": "object", "title": "Network Meters", "properties": network_properties}),
        );

        json!({
            "type": "object",
            "properties": {
                "colors": {
                    "type": "object",
                    "properties": properties
                }
            }
        })
    }

    fn parse_existing_config(&self, content: &str) -> Result<Value, String> {

        // Regex to capture lines like: theme[key]="value"
        let re = Regex::new(r#"^\s*theme\[(?P<key>[A-Za-z0-9_]+)]\s*=\s*"(?P<val>[^"]*)"\s*$"#)
            .map_err(|e| format!("Failed to compile regex: {e}"))?;

        // Helper to map a flat key to its section
        let section_for = |k: &str| -> Option<&'static str> {
            match k {
                // basic
                "main_bg" | "main_fg" | "title" | "hi_fg" | "selected_bg" | "selected_fg"
                | "inactive_fg" | "proc_misc" => Some("basic"),
                // boxes
                "cpu_box" | "mem_box" | "net_box" | "proc_box" | "div_line" => Some("boxes"),
                // temperature
                "temp_start" | "temp_mid" | "temp_end" => Some("temperature"),
                // cpu
                "cpu_start" | "cpu_mid" | "cpu_end" => Some("cpu"),
                // memory
                "free_start" | "free_mid" | "free_end" | "cached_start" | "cached_mid"
                | "cached_end" | "available_start" | "available_mid" | "available_end"
                | "used_start" | "used_mid" | "used_end" => Some("memory"),
                // network
                "download_start" | "download_mid" | "download_end" | "upload_start"
                | "upload_mid" | "upload_end" => Some("network"),
                _ => None,
            }
        };

        let mut colors_obj: Map<String, Value> = Map::new();

        for line in content.lines() {
            if let Some(caps) = re.captures(line) {
                let key = &caps["key"];
                let val = &caps["val"];
                if let Some(section) = section_for(key) {
                    let section_entry = colors_obj
                        .entry(section.to_string())
                        .or_insert_with(|| Value::Object(Map::new()));
                    if let Value::Object(ref mut sect_map) = section_entry {
                        sect_map.insert(key.to_string(), Value::String(val.to_string()));
                    }
                }
            }
        }

        let mut btop_obj = Map::new();
        if !colors_obj.is_empty() {
            btop_obj.insert("colors".to_string(), Value::Object(colors_obj));
        }

        let mut root = Map::new();
        root.insert("btop".to_string(), Value::Object(btop_obj));
        Ok(Value::Object(root))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value as JsonValue;

    // Round-trip test: parse btop theme -> JSON -> generate -> parse again -> JSON should match
    #[test]
    fn btop_theme_round_trip_is_lossless_for_all_keys() {
        let generator = BtopGenerator;

        // Representative complete btop theme content including all keys used by the generator
        let input_theme = r##"
# Comments and blank lines should be ignored
theme[main_bg]=""
theme[main_fg]="#EAEAEA"
theme[title]="#8a8a8d"
theme[hi_fg]="#f59e0b"
theme[selected_bg]="#f59e0b"
theme[selected_fg]="#EAEAEA"
theme[inactive_fg]="#333333"
theme[proc_misc]="#8a8a8d"

theme[cpu_box]="#8a8a8d"
theme[mem_box]="#8a8a8d"
theme[net_box]="#8a8a8d"
theme[proc_box]="#8a8a8d"
theme[div_line]="#8a8a8d"

theme[temp_start]="#8a8a8d"
theme[temp_mid]="#f59e0b"
theme[temp_end]="#b91c1c"

theme[cpu_start]="#8a8a8d"
theme[cpu_mid]="#f59e0b"
theme[cpu_end]="#b91c1c"

theme[free_start]="#8a8a8d"
theme[free_mid]="#f59e0b"
theme[free_end]="#b91c1c"

theme[cached_start]="#8a8a8d"
theme[cached_mid]="#f59e0b"
theme[cached_end]="#b91c1c"

theme[available_start]="#8a8a8d"
theme[available_mid]="#f59e0b"
theme[available_end]="#b91c1c"

theme[used_start]="#8a8a8d"
theme[used_mid]="#f59e0b"
theme[used_end]="#b91c1c"

theme[download_start]="#8a8a8d"
theme[download_mid]="#f59e0b"
theme[download_end]="#b91c1c"

theme[upload_start]="#8a8a8d"
theme[upload_mid]="#f59e0b"
theme[upload_end]="#b91c1c"
"##;

        // Parse theme -> JSON
        let parsed1: JsonValue = generator
            .parse_existing_config(input_theme)
            .expect("parse_existing_config should succeed");

        // Generate theme from parsed JSON
        let regenerated_theme = generator
            .generate_config(&parsed1)
            .expect("generate_config should succeed");

        // Parse generated theme again -> JSON
        let parsed2: JsonValue = generator
            .parse_existing_config(&regenerated_theme)
            .expect("parse_existing_config on regenerated theme should succeed");

        // Assert round-trip stability
        assert_eq!(
            parsed1, parsed2,
            "Parsed JSON before and after regeneration should be identical"
        );
    }

    #[test]
    fn btop_theme_json_schema_all_fields_round_trip() {
        use serde_json::json;
        use super::BtopGenerator;

        // Build a JSON object that includes ALL fields in the schema, fully filled
        let config_json = json!({
            "btop": {
                "colors": {
                    "basic": {
                        "main_bg": "#111111",
                        "main_fg": "#eeeeee",
                        "title": "#cccccc",
                        "hi_fg": "#ffaa00",
                        "selected_bg": "#292900",
                        "selected_fg": "#eeeeee",
                        "inactive_fg": "#444444",
                        "proc_misc": "#123456"
                    },
                    "boxes": {
                        "cpu_box": "#0101FF",
                        "mem_box": "#01FF01",
                        "net_box": "#FF0101",
                        "proc_box": "#CCCCCC",
                        "div_line": "#333333"
                    },
                    "temperature": {
                        "temp_start": "#222222",
                        "temp_mid": "#fcba04",
                        "temp_end": "#ba0404"
                    },
                    "cpu": {
                        "cpu_start": "#030303",
                        "cpu_mid": "#ffa500",
                        "cpu_end": "#ff0000"
                    },
                    "memory": {
                        "free_start": "#0e0e0e",
                        "free_mid": "#ffdb58",
                        "free_end": "#bada55",
                        "cached_start": "#99aabb",
                        "cached_mid": "#887766",
                        "cached_end": "#555555",
                        "available_start": "#102030",
                        "available_mid": "#304050",
                        "available_end": "#506070",
                        "used_start": "#706050",
                        "used_mid": "#a0b0c0",
                        "used_end": "#d0e0f0"
                    },
                    "network": {
                        "download_start": "#0f0f99",
                        "download_mid": "#777700",
                        "download_end": "#f97f00",
                        "upload_start": "#1a2b3c",
                        "upload_mid": "#4d5e6f",
                        "upload_end": "#7f8e9d"
                    }
                }
            }
        });

        let generator = BtopGenerator;

        // Generate config from JSON, then parse generated config, and compare JSON
        let generated = generator
            .generate_config(&config_json)
            .expect("generate_config should succeed");

        let reparsed = generator
            .parse_existing_config(&generated)
            .expect("parse_existing_config on generated config should succeed");

        assert_eq!(
            config_json, reparsed,
            "JSON before and after round-trip through btop format must match exactly"
        );
    }
}