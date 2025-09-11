use super::ConfigGenerator;
use serde_json::{json, Value};

pub struct MakoGenerator;

unsafe impl Send for MakoGenerator {}
unsafe impl Sync for MakoGenerator {}

impl ConfigGenerator for MakoGenerator {
    fn get_app_name(&self) -> &'static str {
        "mako"
    }

    fn get_file_name(&self) -> &'static str {
        "mako.ini"
    }

    fn generate_config(&self, theme_data: &Value) -> Result<String, String> {
        let empty_obj = json!({});
        let mako = theme_data.get("mako").unwrap_or(&empty_obj);

        // Extract color values with defaults from template
        let colors = mako.get("colors").unwrap_or(&empty_obj);
        let normal = colors.get("normal").unwrap_or(&empty_obj);
        let text_color = normal
            .get("text_color")
            .and_then(|t| t.as_str())
            .unwrap_or("#8A8A8D");
        let border_color = normal
            .get("border_color")
            .and_then(|b| b.as_str())
            .unwrap_or("#8A8A8D");
        let background_color = normal
            .get("background_color")
            .and_then(|bg| bg.as_str())
            .unwrap_or("#1E1E1E");

        Ok(format!(
            r#"# ────────────────────────────────────────────────────────────
# Omarchy Custom Theme for Mako
# Generated with Omarchist
# ────────────────────────────────────────────────────────────

text-color={text_color}
border-color={border_color}
background-color={background_color}
width=420
height=110
padding=10
border-size=2
font=Liberation Sans 11
anchor=top-right
outer-margin=20
default-timeout=5000
max-icon-size=32

[app-name=Spotify]
invisible=1

[mode=do-not-disturb]
invisible=true

[mode=do-not-disturb app-name=notify-send]
invisible=false
"#,
        ))
    }

    fn get_config_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "colors": {
                    "type": "object",
                    "properties": {
                        "normal": {
                            "type": "object",
                            "properties": {
                                "border_color": {
                                    "type": "string",
                                    "format": "color",
                                    "title": "Border Color",
                                    "description": "Color of notification border",
                                    "default": "#8A8A8D",
                                },
                                "background_color": {
                                    "type": "string",
                                    "format": "color",
                                    "title": "Background Color",
                                    "description": "Background color of notifications",
                                    "default": "#1E1E1E",
                                },
                                "text_color": {
                                    "type": "string",
                                    "format": "color",
                                    "title": "Text Color",
                                    "description": "Color of notification text",
                                    "default": "#8A8A8D",
                                },
                            }
                        },                      
                    }
                }
            }
        })
    }

    fn parse_existing_config(&self, content: &str) -> Result<serde_json::Value, String> {
        use serde_json::{json, Map, Value};
        use regex::Regex;

        let mut colors = Map::new();

        let mut extract_color = |ini_key: &str| {
            let regex = format!(r#"(?m)^\s*{}\s*=\s*([#a-zA-Z0-9]+)"#, ini_key);
            let re = Regex::new(&regex)
                .expect("Regex error building for color extraction. \
                All components are constants and can't be runtime error");
            if let Some(cap) = re.captures(content) {
                colors.insert(ini_key.replace("-", "_"), Value::String(cap[1].trim().to_string()));
            }
        };

        extract_color("background-color");
        extract_color("border-color");
        extract_color("text-color");

        if !colors.is_empty() {
            Ok(json!({
                "mako": {
                    "colors": {
                        "normal": Value::Object(colors)
                    }
                }
            }))
        } else {
            Ok(json!({ "mako": {} }))
        }

    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value as JsonValue;

    // Round-trip test: parse INI -> JSON -> generate INI -> parse INI -> JSON and compare
    #[test]
    fn ini_round_trip_is_lossless_for_top_level_colors() {
        let generator = MakoGenerator;

        let input_ini = r#"

text-color=#ABCDEF
border-color=#123456
background-color=#0F0F0F
width=420
height=110
padding=10
border-size=2
font=Liberation Sans 11
anchor=top-right
outer-margin=20
default-timeout=5000
max-icon-size=32

[app-name=Spotify]
invisible=1

[mode=do-not-disturb]
invisible=true

[mode=do-not-disturb app-name=notify-send]
invisible=false
"#;

        // Parse INI -> JSON
        let parsed1: JsonValue = generator
            .parse_existing_config(input_ini)
            .expect("parse_existing_config should succeed");

        // Generate INI from parsed JSON
        let regenerated_ini = generator
            .generate_config(&parsed1)
            .expect("generate_config should succeed");

        // Parse generated INI again -> JSON
        let parsed2: JsonValue = generator
            .parse_existing_config(&regenerated_ini)
            .expect("parse_existing_config on regenerated ini should succeed");

        // Assert round-trip stability for the parts we care about
        assert_eq!(
            parsed1, parsed2,
            "Parsed JSON before and after regeneration should be identical"
        );
    }

    #[test]
    fn json_round_trip_is_lossless_missing_fields() {
        let generator = MakoGenerator;

        // JSON input similar to what parse_existing_config produces
        let json_str = r##"
        {
            "mako": {
                "colors": {
                    "normal": {
                        "text_color": "#AABBCC",
                        "border_color": "#334455",
                        "background_color": "#112233"
                    }
                }
            }
        }
        "##;

        let parsed_json: JsonValue = serde_json::from_str(json_str).expect("Valid JSON");

        // Generate INI from JSON
        let ini = generator
            .generate_config(&parsed_json)
            .expect("generate_config should succeed");

        // Parse generated INI back to JSON
        let reparsed_json: JsonValue = generator
            .parse_existing_config(&ini)
            .expect("parse_existing_config should succeed");

        // For direct equality, since only top-level colors are covered,
        // we compare the relevant nested structure:
        assert_eq!(
            parsed_json, reparsed_json,
            "JSON config before and after INI round-trip should be identical for top-level colors"
        );
    }
}
