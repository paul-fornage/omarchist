use super::ConfigGenerator;
use serde_json::{json, Map, Value};
use crate::services::generators::css_parser;

pub struct WaybarGenerator;

unsafe impl Send for WaybarGenerator {}
unsafe impl Sync for WaybarGenerator {}

impl ConfigGenerator for WaybarGenerator {
    fn get_app_name(&self) -> &'static str {
        "waybar"
    }

    fn get_file_name(&self) -> &'static str {
        "waybar.css"
    }

    fn generate_config(&self, theme_data: &Value) -> Result<String, String> {
        let empty_obj = json!({});
        let waybar = theme_data.get("waybar").unwrap_or(&empty_obj);

        // Extract color variables with defaults from template
        let empty_colors = json!({});
        let colors = waybar.get("colors").unwrap_or(&empty_colors);
        let main = colors.get("main").unwrap_or(&empty_colors);
        let bg = main
            .get("background")
            .and_then(|b| b.as_str())
            .unwrap_or("#1e1e1e");
        let fg = main
            .get("foreground")
            .and_then(|f| f.as_str())
            .unwrap_or("#8a8a8d");
        Ok(format!(
            r#"/* ────────────────────────────────────────────────────────────
 * Omarchy Custom Theme for Waybar
 * Generated with Omarchist
 * ────────────────────────────────────────────────────────────
 */

@define-color background {bg};
@define-color foreground {fg};
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
                        "main": {
                            "type": "object",
                            "properties": {
                                "background": {"type": "string", "format": "color", "title": "Background", "default": "#1e1e1e"},
                                "foreground": {"type": "string", "format": "color", "title": "Foreground", "default": "#8a8a8d"}
                            }
                        },
                    }
                }
            }
        })
    }

    fn parse_existing_config(&self, content: &str) -> Result<Value, String> {

        let css_keys = ["background", "foreground"];

        let main_colors = css_parser(content, &css_keys)?;
        
        let mut colors_obj = Map::new();
        if !main_colors.is_empty() {
            colors_obj.insert("main".to_string(), Value::Object(main_colors));
        }

        let mut waybar_obj = Map::new();
        if !colors_obj.is_empty() {
            waybar_obj.insert("colors".to_string(), Value::Object(colors_obj));
        }

        Ok(Value::Object({
            let mut m = Map::new();
            m.insert("waybar".to_string(), Value::Object(waybar_obj));
            m
        }))
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    // Helper for round-trip test: parse -> generate -> parse
    fn waybar_round_trip(input_css: &str) {
        let generator = WaybarGenerator;

        let parsed1 = generator
            .parse_existing_config(input_css)
            .expect("parse_existing_config should succeed");

        let regenerated_css = generator
            .generate_config(&parsed1)
            .expect("generate_config should succeed");

        let parsed2 = generator
            .parse_existing_config(&regenerated_css)
            .expect("parse_existing_config on regenerated config should succeed");

        assert_eq!(
            parsed1, parsed2,
            "Waybar parsed JSON before and after regeneration should be identical"
        );
    }

    #[test]
    fn parse_extracts_colors() {
        let css = r#"
        @define-color background #121212;
        @define-color foreground #ffffff;
        /* unrelated style */
        * { color: @foreground; }
        "#;
        let gen = WaybarGenerator;
        let parsed = gen.parse_existing_config(css).unwrap();
        assert_eq!(
            parsed,
            json!({
                "waybar": {
                    "colors": {
                        "main": {
                            "background": "#121212",
                            "foreground": "#ffffff"
                        }
                    }
                }
            })
        );
    }

    #[test]
    fn waybar_round_trip_is_lossless() {
        let input_css = r#"
        /* Omarchist theme */
        @define-color background #8a8a8d;
        @define-color foreground #121212;
        "#;
        waybar_round_trip(input_css);
    }

    #[test]
    fn waybar_json_round_trip_is_lossless() {
        let gen = WaybarGenerator;
        // The JSON value to use as the "source of truth"—can add more fields later if supported
        let original = json!({
            "waybar": {
                "colors": {
                    "main": {
                        "background": "#123456",
                        "foreground": "#abcdef"
                    }
                }
            }
        });

        let generated_css = gen
            .generate_config(&original)
            .expect("generate_config should succeed");

        let reparsed = gen
            .parse_existing_config(&generated_css)
            .expect("parse_existing_config should succeed");

        assert_eq!(
            original, reparsed,
            "Waybar JSON -> CSS -> JSON should round-trip losslessly"
        );
    }

}

