use log::warn;
use regex::Regex;
use super::ConfigGenerator;
use serde_json::{json, Value};

pub struct HyprlockGenerator;

unsafe impl Send for HyprlockGenerator {}
unsafe impl Sync for HyprlockGenerator {}

impl ConfigGenerator for HyprlockGenerator {
    fn get_app_name(&self) -> &'static str {
        "hyprlock"
    }

    fn get_file_name(&self) -> &'static str {
        "hyprlock.conf"
    }

    fn generate_config(&self, theme_data: &Value) -> Result<String, String> {
        let empty_obj = json!({});
        let hyprlock = theme_data.get("hyprlock").unwrap_or(&empty_obj);

        // Extract color values with defaults from template
        let colors = hyprlock.get("colors").unwrap_or(&empty_obj);
        let color = colors
            .get("color")
            .and_then(|c| c.as_str())
            .unwrap_or("12,12,12,1.0");
        let inner_color = colors
            .get("inner_color")
            .and_then(|i| i.as_str())
            .unwrap_or("138,138,141,0.3");
        let outer_color = colors
            .get("outer_color")
            .and_then(|o| o.as_str())
            .unwrap_or("234,234,234,0.5");
        let font_color = colors
            .get("font_color")
            .and_then(|f| f.as_str())
            .unwrap_or("234,234,234,1.0");
        let check_color = colors
            .get("check_color")
            .and_then(|c| c.as_str())
            .unwrap_or("245,158,11,1.0");

        Ok(format!(
            r#"# ────────────────────────────────────────────────────────────
# Omarchy Custom Theme for Hyprlock
# Generated with Omarchist
# ────────────────────────────────────────────────────────────

$color = rgba({color})
$inner_color = rgba({inner_color})
$outer_color = rgba({outer_color})
$font_color = rgba({font_color})
$check_color = rgba({check_color})
"#
        ))
    }

    fn get_config_schema(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "colors": {
                    "type": "object",
                    "properties": {
                        "color": {
                            "type": "string",
                            "output_format": "rgba-comma",
                            "default": "12,12,12,1.0",
                            "title": "Background Color",
                            "description": "Main background color (RGBA format)"
                        },
                        "inner_color": {
                            "type": "string",
                            "output_format": "rgba-comma",
                            "default": "138,138,141,0.3",
                            "title": "Inner Color",
                            "description": "Inner element color (RGBA format)"
                        },
                        "outer_color": {
                            "type": "string",
                            "output_format": "rgba-comma",
                            "default": "234,234,234,0.5",
                            "title": "Outer Color",
                            "description": "Outer element color (RGBA format)"
                        },
                        "font_color": {
                            "type": "string",
                            "output_format": "rgba-comma",
                            "default": "234,234,234,1.0",
                            "title": "Font Color",
                            "description": "Text color (RGBA format)"
                        },
                        "check_color": {
                            "type": "string",
                            "output_format": "rgba-comma",
                            "default": "245,158,11,1.0",
                            "title": "Check Color",
                            "description": "Check/accent color (RGBA format)"
                        }
                    }
                }
            }
        })
    }

    fn parse_existing_config(&self, content: &str) -> Result<Value, String> {
        let mut colors_obj = serde_json::Map::new();

        let valid_keys = &["color", "inner_color", "outer_color", "font_color", "check_color"];

        // Capture variable assignments like: $color = rgba(12,34,56,0.7)
        let re = Regex::new(
            r#"(?mi)^\s*\$(?P<key>[A-Za-z0-9_]+)\s*=\s*rgba\(\s*(?P<value>[^)]+?)\s*\)\s*$"#)
            .unwrap();

        re.captures_iter(content).for_each(|cap| {
            if !valid_keys.contains(&cap["key"].to_string().as_str()) {
                warn!("Hyprlock config contains invalid key: {}", cap["key"].to_string());
            } else {
                colors_obj.insert(cap["key"].to_string(), json!(cap["value"]));
            }

        });

        let mut hyprlock_obj = serde_json::Map::new();
        if !colors_obj.is_empty() {
            hyprlock_obj.insert("colors".into(), Value::Object(colors_obj));
        }

        Ok(json!({ "hyprlock": hyprlock_obj }))
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value as JsonValue;

    #[test]
    fn hyprlock_round_trip_from_json_schema() {
        let generator = HyprlockGenerator;

        let config_json = json!({
            "hyprlock": {
                "colors": {
                    "color": "12,12,12,1.0",
                    "inner_color": "138,138,141,0.3",
                    "outer_color": "234,234,234,0.5",
                    "font_color": "234,234,234,1.0",
                    "check_color": "245,158,11,1.0",
                }
            }
        });


        // Generate config from this JSON
        let config = generator.generate_config(&config_json)
            .expect("Should generate hyprlock config from schema-based JSON");

        // Parse back to JSON
        let reparsed_json = generator.parse_existing_config(&config)
            .expect("Parsing generated config should succeed");

        // It should round-trip to the same structure
        assert_eq!(
            config_json, reparsed_json,
            "JSON config round-trips through generate_config and parse_existing_config"
        );
    }

    fn hyprlock_round_trip(input_conf: &str) {
        let generator = HyprlockGenerator;

        let parsed1: JsonValue = generator
            .parse_existing_config(input_conf)
            .expect("parse_existing_config should succeed");

        let regenerated_conf = generator
            .generate_config(&parsed1)
            .expect("generate_config should succeed");

        let parsed2: JsonValue = generator
            .parse_existing_config(&regenerated_conf)
            .expect("parse_existing_config on regenerated config should succeed");

        assert_eq!(
            parsed1, parsed2,
            "Hyprlock parsed JSON before and after regeneration should be identical"
        );
    }

    #[test]
    fn hyprlock_round_trip_is_lossless() {
        let input_conf = r#"
# Example Hyprlock config
$color = rgba(12,12,12,1.0)
$inner_color = rgba(138,138,141,0.3)
$outer_color = rgba(234,234,234,0.5)
$font_color = rgba(234,234,234,1.0)
$check_color = rgba(245,158,11,1.0)
"#;
        hyprlock_round_trip(input_conf);
    }

    #[test]
    fn hyprlock_round_trip_is_lossless_with_whitespace() {
        let input_conf = r#"
# Mixed whitespace and formatting
   $color     =   rgba( 12 ,  12,12 , 1.0  )
$inner_color=rgba(138,138,141,0.3)
   $outer_color = rgba(234, 234, 234, 0.5)
$font_color= rgba(234,234,234,1.0)
$check_color = rgba(245,158,11,1.0)
"#;
        hyprlock_round_trip(input_conf);
    }
}
