use super::ConfigGenerator;
use serde_json::{json, Map, Value};
use toml;

pub struct AlacrittyGenerator;

unsafe impl Send for AlacrittyGenerator {}
unsafe impl Sync for AlacrittyGenerator {}

impl ConfigGenerator for AlacrittyGenerator {
    fn get_app_name(&self) -> &'static str {
        "alacritty"
    }

    fn get_file_name(&self) -> &'static str {
        "alacritty.toml"
    }

    fn generate_config(&self, theme_data: &Value) -> Result<String, String> {
        let empty_obj = json!({});
        let alacritty = theme_data.get("alacritty").unwrap_or(&empty_obj);

        // Primary colors
        let primary_bg = alacritty
            .get("colors")
            .and_then(|c| c.get("primary"))
            .and_then(|p| p.get("background"))
            .and_then(|b| b.as_str())
            .unwrap_or("#121212");
        let primary_fg = alacritty
            .get("colors")
            .and_then(|c| c.get("primary"))
            .and_then(|p| p.get("foreground"))
            .and_then(|f| f.as_str())
            .unwrap_or("#bebebe");
        let dim_fg = alacritty
            .get("colors")
            .and_then(|c| c.get("primary"))
            .and_then(|p| p.get("dim_foreground"))
            .and_then(|d| d.as_str())
            .unwrap_or("#8a8a8d");

        // Normal colors
        let empty_normal = json!({});
        let normal = alacritty
            .get("colors")
            .and_then(|c| c.get("normal"))
            .unwrap_or(&empty_normal);
        let normal_black = normal
            .get("black")
            .and_then(|b| b.as_str())
            .unwrap_or("#333333");
        let normal_red = normal
            .get("red")
            .and_then(|r| r.as_str())
            .unwrap_or("#D35F5F");
        let normal_green = normal
            .get("green")
            .and_then(|g| g.as_str())
            .unwrap_or("#FFC107");
        let normal_yellow = normal
            .get("yellow")
            .and_then(|y| y.as_str())
            .unwrap_or("#b91c1c");
        let normal_blue = normal
            .get("blue")
            .and_then(|b| b.as_str())
            .unwrap_or("#e68e0d");
        let normal_magenta = normal
            .get("magenta")
            .and_then(|m| m.as_str())
            .unwrap_or("#D35F5F");
        let normal_cyan = normal
            .get("cyan")
            .and_then(|c| c.as_str())
            .unwrap_or("#bebebe");
        let normal_white = normal
            .get("white")
            .and_then(|w| w.as_str())
            .unwrap_or("#bebebe");

        // Bright colors
        let empty_bright = json!({});
        let bright = alacritty
            .get("colors")
            .and_then(|c| c.get("bright"))
            .unwrap_or(&empty_bright);
        let bright_black = bright
            .get("black")
            .and_then(|b| b.as_str())
            .unwrap_or("#8a8a8d");
        let bright_red = bright
            .get("red")
            .and_then(|r| r.as_str())
            .unwrap_or("#B91C1C");
        let bright_green = bright
            .get("green")
            .and_then(|g| g.as_str())
            .unwrap_or("#FFC107");
        let bright_yellow = bright
            .get("yellow")
            .and_then(|y| y.as_str())
            .unwrap_or("#b90a0a");
        let bright_blue = bright
            .get("blue")
            .and_then(|b| b.as_str())
            .unwrap_or("#f59e0b");
        let bright_magenta = bright
            .get("magenta")
            .and_then(|m| m.as_str())
            .unwrap_or("#b91c1c");
        let bright_cyan = bright
            .get("cyan")
            .and_then(|c| c.as_str())
            .unwrap_or("#eaeaea");
        let bright_white = bright
            .get("white")
            .and_then(|w| w.as_str())
            .unwrap_or("#eaeaea");

        // Extract font settings
        // let font_size = alacritty
        //     .get("font")
        //     .and_then(|f| f.get("size"))
        //     .and_then(|s| s.as_f64())
        //     .unwrap_or(12.0);

        // Extract window settings
        // let empty_window = json!({});
        // let window = alacritty.get("window").unwrap_or(&empty_window);
        // let padding_x = window
        //     .get("padding")
        //     .and_then(|p| p.get("x"))
        //     .and_then(|x| x.as_i64())
        //     .unwrap_or(12);
        // let padding_y = window
        //     .get("padding")
        //     .and_then(|p| p.get("y"))
        //     .and_then(|y| y.as_i64())
        //     .unwrap_or(12);

        // Cursor colors
        let empty_cursor = json!({});
        let cursor_obj = alacritty
            .get("colors")
            .and_then(|c| c.get("cursor"))
            .unwrap_or(&empty_cursor);
        let cursor_text = cursor_obj
            .get("text")
            .and_then(|v| v.as_str())
            .unwrap_or("#121212");
        let cursor_cursor = cursor_obj
            .get("cursor")
            .and_then(|v| v.as_str())
            .unwrap_or("#eaeaea");

        // Vi mode cursor colors
        // let empty_vi_cursor = json!({});
        // let vi_cursor_obj = alacritty
        //     .get("colors")
        //     .and_then(|c| c.get("vi_mode_cursor"))
        //     .unwrap_or(&empty_vi_cursor);
        // let vi_cursor_text = vi_cursor_obj
        //     .get("text")
        //     .and_then(|v| v.as_str())
        //     .unwrap_or("#121212");
        // let vi_cursor_cursor = vi_cursor_obj
        //     .get("cursor")
        //     .and_then(|v| v.as_str())
        //     .unwrap_or("#eaeaea");

        // Selection
        let empty_selection = json!({});
        let selection_obj = alacritty
            .get("colors")
            .and_then(|c| c.get("selection"))
            .unwrap_or(&empty_selection);
        let selection_background = selection_obj
            .get("background")
            .and_then(|v| v.as_str())
            .unwrap_or("#333333");

        Ok(format!(
            r#"# ────────────────────────────────────────────────────────────
# Omarchy Custom Theme for Alacritty
# Generated with Omarchist
# ────────────────────────────────────────────────────────────

[colors]
[colors.primary]
background = "{primary_bg}"
foreground = "{primary_fg}"
dim_foreground = "{dim_fg}"

[colors.cursor]
text = "{cursor_text}"
cursor = "{cursor_cursor}"

[colors.selection]
text = "CellForeground"
background = "{selection_background}"

[colors.normal]
black = "{normal_black}"
red = "{normal_red}"
green = "{normal_green}"
yellow = "{normal_yellow}"
blue = "{normal_blue}"
magenta = "{normal_magenta}"
cyan = "{normal_cyan}"
white = "{normal_white}"

[colors.bright]
black = "{bright_black}"
red = "{bright_red}"
green = "{bright_green}"
yellow = "{bright_yellow}"
blue = "{bright_blue}"
magenta = "{bright_magenta}"
cyan = "{bright_cyan}"
white = "{bright_white}"
"#
        ))
    }

    fn get_config_schema(&self) -> Value {
        json!({
            "type": "object",
            // UI ordering for top-level properties
            "x-order": ["colors", "font", "window"],
            "properties": {
                "colors": {
                    "type": "object",
                    // Ensure colors sections render in intended order
                    "x-order": [
                        "primary",
                        "cursor",
                        "vi_mode_cursor",
                        "selection",
                        "normal",
                        "bright"
                    ],
                    "properties": {
                        "primary": {
                            "type": "object",
                            "x-order": ["background", "foreground", "dim_foreground"],
                            "properties": {
                                "background": {"type": "string", "format": "color", "title": "Background Color"},
                                "foreground": {"type": "string", "format": "color", "title": "Foreground Color"},
                                "dim_foreground": {"type": "string", "format": "color", "title": "Dim Foreground Color"}
                            }
                        },
                        "cursor": {
                            "type": "object",
                            "properties": {
                                "text": {"type": "string", "format": "color", "title": "Cursor Text", "default": "#121212"},
                                "cursor": {"type": "string", "format": "color", "title": "Cursor Color", "default": "#EAEAEA"}
                            }
                        },
                        "selection": {
                            "type": "object",
                            "properties": {
                                "background": {"type": "string", "format": "color", "title": "Selection Background", "default": "#333333"}
                            }
                        },
                        "normal": {
                            "type": "object",
                            "x-order": ["black", "red", "green", "yellow", "blue", "magenta", "cyan", "white"],
                            "properties": {
                                "black": {"type": "string", "format": "color", "title": "Black"},
                                "red": {"type": "string", "format": "color", "title": "Red"},
                                "green": {"type": "string", "format": "color", "title": "Green"},
                                "yellow": {"type": "string", "format": "color", "title": "Yellow"},
                                "blue": {"type": "string", "format": "color", "title": "Blue"},
                                "magenta": {"type": "string", "format": "color", "title": "Magenta"},
                                "cyan": {"type": "string", "format": "color", "title": "Cyan"},
                                "white": {"type": "string", "format": "color", "title": "White"}
                            }
                        },
                        "bright": {
                            "type": "object",
                            "x-order": ["black", "red", "green", "yellow", "blue", "magenta", "cyan", "white"],
                            "properties": {
                                "black": {"type": "string", "format": "color", "title": "Bright Black"},
                                "red": {"type": "string", "format": "color", "title": "Bright Red"},
                                "green": {"type": "string", "format": "color", "title": "Bright Green"},
                                "yellow": {"type": "string", "format": "color", "title": "Bright Yellow"},
                                "blue": {"type": "string", "format": "color", "title": "Bright Blue"},
                                "magenta": {"type": "string", "format": "color", "title": "Bright Magenta"},
                                "cyan": {"type": "string", "format": "color", "title": "Bright Cyan"},
                                "white": {"type": "string", "format": "color", "title": "Bright White"}
                            }
                        }
                    }
                }
            }
        })
    }

    fn parse_existing_config(&self, content: &str) -> Result<Value, String> {
        // Parse TOML into a toml::Value
        let parsed: toml::Value =
            toml::from_str(content).map_err(|e| format!("Failed to parse alacritty.toml: {e}"))?;

        let mut colors_json = Map::new();

        // [colors]
        if let Some(colors_tbl) = parsed.get("colors").and_then(|v| v.as_table()) {
            // [colors.primary]
            if let Some(primary_tbl) = colors_tbl.get("primary").and_then(|v| v.as_table()) {
                let mut primary_json = Map::new();
                transfer_kvs(primary_tbl, &mut primary_json, &["background", "foreground", "dim_foreground"]);
                if !primary_json.is_empty() {
                    colors_json.insert("primary".to_string(), Value::Object(primary_json));
                }
            }

            // [colors.cursor]
            if let Some(cursor_tbl) = colors_tbl.get("cursor").and_then(|v| v.as_table()) {
                let mut cursor_json = Map::new();
                transfer_kvs(cursor_tbl, &mut cursor_json, &["text", "cursor"]);
                if !cursor_json.is_empty() {
                    colors_json.insert("cursor".to_string(), Value::Object(cursor_json));
                }
            }

            // [colors.selection]
            if let Some(selection_tbl) = colors_tbl.get("selection").and_then(|v| v.as_table()) {
                let mut selection_json = Map::new();
                transfer_kvs(selection_tbl, &mut selection_json, &["background", "text"]);
                if !selection_json.is_empty() {
                    colors_json.insert("selection".to_string(), Value::Object(selection_json));
                }
            }

            let color_list = &["black", "red", "green", "yellow", "blue", "magenta", "cyan", "white"];

            // [colors.normal]
            if let Some(normal_tbl) = colors_tbl.get("normal").and_then(|v| v.as_table()) {
                let mut normal_json = Map::new();
                transfer_kvs( normal_tbl, &mut normal_json, color_list);
                if !normal_json.is_empty() {
                    colors_json.insert("normal".to_string(), Value::Object(normal_json));
                }
            }

            // [colors.bright]
            if let Some(bright_tbl) = colors_tbl.get("bright").and_then(|v| v.as_table()) {
                let mut bright_json = Map::new();
                transfer_kvs(bright_tbl, &mut bright_json, color_list);
                if !bright_json.is_empty() {
                    colors_json.insert("bright".to_string(), Value::Object(bright_json));
                }
            }
        }

        // Assemble final structure expected by generate_config
        let mut alacritty_obj = Map::new();
        if !colors_json.is_empty() {
            alacritty_obj.insert("colors".to_string(), Value::Object(colors_json));
        }

        let mut root = Map::new();
        root.insert("alacritty".to_string(), Value::Object(alacritty_obj));
        Ok(Value::Object(root))
    }
}

fn transfer_kv(table: &toml::Table, json: &mut Map<String, Value>, key: &str) {
    if let Some(v) = table.get(key).and_then(|x| x.as_str()) {
        json.insert(key.to_string(), Value::String(v.to_string()));
    }
}

fn transfer_kvs(table: &toml::Table, json: &mut Map<String, Value>, keys: &[&str]) {
    for key in keys {
        transfer_kv(table, json, key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toml_test_conversion_1() {
        let input_toml = r##"
[colors]
[colors.primary]
background = "#121212"
foreground = "#bebebe"
dim_foreground = "#8a8a8d"

[colors.cursor]
text = "#abcdef"
cursor = "#123456"

[colors.selection]
text = "CellForeground"
background = "#333333"

[colors.normal]
black = "#111111"
red = "#ff0055"
green = "#28fc3b"
yellow = "#c0c000"
blue = "#0055fc"
magenta = "#e24fff"
cyan = "#6beceb"
white = "#fafafa"

[colors.bright]
black = "#ababab"
red = "#ff0000"
green = "#00ff00"
yellow = "#ffff00"
blue = "#0000ff"
magenta = "#ff00ff"
cyan = "#00ffff"
white = "#ffffff"
"##;
        toml_round_trip(input_toml);
    }

    #[test]
    fn toml_test_conversion_2() {
        let input_toml = r##"
[colors]
[colors.primary]
background = "#222222"
foreground = "#cccccc"
dim_foreground = "#797979"

[colors.cursor]
text = "#333333"
cursor = "#dddddd"

[colors.selection]
text = "CellForeground"
background = "#444444"

[colors.normal]
black = "#111122"
red = "#ee3344"
green = "#22cc33"
yellow = "#ffd700"
blue = "#3355ee"
magenta = "#ee33ee"
cyan = "#33eeee"
white = "#eeeeee"

[colors.bright]
black = "#999999"
red = "#ff2222"
green = "#22ff22"
yellow = "#fffe22"
blue = "#2222ff"
magenta = "#ff22ff"
cyan = "#22ffff"
white = "#eeeeee"
"##;
        toml_round_trip(input_toml);
    }

    #[test]
    fn toml_test_conversion_3() {
        let input_toml = r##"
[colors]
[colors.primary]
background = "#232323"
foreground = "#dfdfdf"
dim_foreground = "#8f8e90"

[colors.cursor]
text = "#232323"
cursor = "#fefefe"

[colors.selection]
text = "CellForeground"
background = "#232323"

[colors.normal]
black = "#222222"
red = "#ff6565"
green = "#55fa65"
yellow = "#cabf30"
blue = "#5068df"
magenta = "#bb32ff"
cyan = "#27eae4"
white = "#c0c0c0"

[colors.bright]
black = "#555555"
red = "#fd4e4e"
green = "#91ff91"
yellow = "#ffee58"
blue = "#597cff"
magenta = "#f661ef"
cyan = "#56fff7"
white = "#ffffff"
"##;
        toml_round_trip(input_toml);
    }


    fn toml_round_trip(input_toml: &str) {
        let generator = AlacrittyGenerator;

        // Parse TOML -> JSON
        let parsed1: Value = generator
            .parse_existing_config(input_toml)
            .expect("parse_existing_config should succeed");

        // Generate TOML from parsed JSON
        let regenerated_toml = generator
            .generate_config(&parsed1)
            .expect("generate_config should succeed");

        // Parse generated TOML again -> JSON
        let parsed2: Value = generator
            .parse_existing_config(&regenerated_toml)
            .expect("parse_existing_config on regenerated toml should succeed");

        // Assert round-trip stability
        assert_eq!(
            parsed1, parsed2,
            "Parsed JSON before and after regeneration should be identical"
        );

        let sanitized_input: toml::Value = toml::from_str(input_toml).unwrap();
        let sanitized_regenerated: toml::Value = toml::from_str(&regenerated_toml).unwrap();

        assert_eq!(sanitized_input, sanitized_regenerated)
    }


    #[test]
    fn json_test_conversion_1() {
        let config_json = json!({
            "alacritty": {
                "colors": {
                    "primary": {
                        "background": "#121212",
                        "foreground": "#bebebe",
                        "dim_foreground": "#8a8a8d"
                    },
                    "cursor": {
                        "text": "#121212",
                        "cursor": "#eaeaea"
                    },
                    "selection": {
                        "text": "CellForeground",
                        "background": "#333333"
                    },
                    "normal": {
                        "black": "#333333",
                        "red": "#D35F5F",
                        "green": "#FFC107",
                        "yellow": "#b91c1c",
                        "blue": "#e68e0d",
                        "magenta": "#D35F5F",
                        "cyan": "#bebebe",
                        "white": "#bebebe"
                    },
                    "bright": {
                        "black": "#8a8a8d",
                        "red": "#B91C1C",
                        "green": "#FFC107",
                        "yellow": "#b90a0a",
                        "blue": "#f59e0b",
                        "magenta": "#B91C1C",
                        "cyan": "#eaeaea",
                        "white": "#ffffff"
                    }
                }
            }
        });

        alacritty_round_trip_from_json_schema(config_json);
    }


    fn alacritty_round_trip_from_json_schema(test_val: Value) {
        let generator = AlacrittyGenerator;

        // Example (minimal) schema-style JSON structure for Alacritty


        // Generate config string from JSON
        let generated_toml = generator
            .generate_config(&test_val)
            .expect("generate_config should succeed");

        // Parse the generated config string back into JSON
        let reparsed_json = generator
            .parse_existing_config(&generated_toml)
            .expect("parse_existing_config on generated TOML should succeed");

        // Should round-trip exactly (structure preserved)
        assert_eq!(
            test_val, reparsed_json,
            "Alacritty parsed JSON round-trips through generate_config and parse_existing_config"
        );
    }

}
