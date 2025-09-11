use super::ConfigGenerator;
use serde_json::{json, Value};
use crate::services::generators::css_parser;

pub struct SwayosdGenerator;

unsafe impl Send for SwayosdGenerator {}
unsafe impl Sync for SwayosdGenerator {}

impl ConfigGenerator for SwayosdGenerator {
    fn get_app_name(&self) -> &'static str {
        "swayosd"
    }

    fn get_file_name(&self) -> &'static str {
        "swayosd.css"
    }

    fn generate_config(&self, theme_data: &Value) -> Result<String, String> {
        let empty_obj = json!({});
        let swayosd = theme_data.get("swayosd").unwrap_or(&empty_obj);

        // Extract color values with defaults from template
        let colors = swayosd.get("colors").unwrap_or(&empty_obj);
        let background_color = colors
            .get("background_color")
            .and_then(|bg| bg.as_str())
            .unwrap_or("#121212");
        let border_color = colors
            .get("border_color")
            .and_then(|bc| bc.as_str())
            .unwrap_or("#8A8A8D");
        let label = colors
            .get("label")
            .and_then(|l| l.as_str())
            .unwrap_or("#8A8A8D");
        let image = colors
            .get("image")
            .and_then(|i| i.as_str())
            .unwrap_or("#8A8A8D");
        let progress = colors
            .get("progress")
            .and_then(|p| p.as_str())
            .unwrap_or("#8A8A8D");

        Ok(format!(
            r#"/* ────────────────────────────────────────────────────────────
 * Omarchy Custom Theme for SwayOSD
 * Generated with Omarchist
 * ────────────────────────────────────────────────────────────
 */

@define-color background-color {background_color};
@define-color border-color {border_color};
@define-color label {label};
@define-color image {image};
@define-color progress {progress};
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
                            "background_color": {
                                "type": "string",
                                "format": "color",
                                "default": "#121212",
                                "title": "Background Color",
                                "description": "Background color of OSD"
                            },
                            "border_color": {
                                "type": "string",
                                "format": "color",
                                "default": "#8A8A8D",
                                "title": "Border Color",
                                "description": "Border color of OSD"
                            },
                            "label": {
                                "type": "string",
                                "format": "color",
                                "default": "#8A8A8D",
                                "title": "Label Color",
                                "description": "Color of text labels"
                            },
                            "image": {
                                "type": "string",
                                "format": "color",
                                "default": "#8A8A8D",
                                "title": "Image Color",
                                "description": "Color of icons/images"
                            },
                            "progress": {
                                "type": "string",
                                "format": "color",
                                "default": "#8A8A8D",
                                "title": "Progress Color",
                                "description": "Color of progress bars"
                            }
                     }
                }
            }
        })
    }

    fn parse_existing_config(&self, content: &str) -> Result<Value, String> {
        // Map CSS keys to JSON keys
        let css_keys = [
            "background-color",
            "border-color",
            "label",
            "image",
            "progress",
        ];
        
        let colors_obj = css_parser(content, &css_keys)?;

        let mut swayosd_obj = serde_json::Map::new();
        if !colors_obj.is_empty() {
            swayosd_obj.insert("colors".into(), Value::Object(colors_obj));
        }

        Ok(json!({ "swayosd": swayosd_obj }))
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value as JsonValue;

    #[test]
    fn swayosd_json_initiated_round_trip() {
        let generator = SwayosdGenerator;
        let json_input = json!({
            "swayosd": {
                "colors": {
                    "background_color": "#102030",
                    "border_color": "#234567",
                    "label": "#345678",
                    "image": "#456789",
                    "progress": "#56789A"
                }
            }
        });

        // JSON → CSS
        let css = generator
            .generate_config(&json_input)
            .expect("generate_config from JSON");

        // CSS → JSON
        let parsed_json = generator
            .parse_existing_config(&css)
            .expect("parse_existing_config on generated CSS");

        // Only compare the swayosd.colors object, which is what round-trips.
        let orig_colors = &json_input["swayosd"]["colors"];
        let parsed_colors = &parsed_json["swayosd"]["colors"];
        assert_eq!(
            orig_colors, parsed_colors,
            "Parsed JSON should match original JSON for color fields"
        );
    }


    fn swayosd_round_trip(input_css: &str) {
        let generator = SwayosdGenerator;

        // Parse -> JSON
        let parsed1: JsonValue = generator
            .parse_existing_config(input_css)
            .expect("parse_existing_config should succeed");

        // Generate -> css
        let regenerated_css = generator
            .generate_config(&parsed1)
            .expect("generate_config should succeed");

        // Parse regenerated -> JSON
        let parsed2: JsonValue = generator
            .parse_existing_config(&regenerated_css)
            .expect("parse_existing_config on regenerated config should succeed");

        assert_eq!(
            parsed1, parsed2,
            "SwayOSD parsed JSON before and after regeneration should be identical"
        );
    }

    #[test]
    fn swayosd_round_trip_is_lossless() {
        let input_css = r#"
/* Example SwayOSD CSS */
@define-color background-color #121212;
@define-color border-color #8A8A8D;
@define-color label #8A8A8D;
@define-color image #8A8A8D;
@define-color progress #8A8A8D;
"#;
        swayosd_round_trip(input_css);
    }

    #[test]
    fn swayosd_round_trip_handles_whitespace() {
        let input_css = r#"
/* Mixed spacing */
   @define-color   background-color    #121212  ;
@define-color border-color    #8A8A8D;
   @define-color label #8A8A8D ;
@define-color    image     #8A8A8D;
@define-color progress      #8A8A8D    ;
"#;
        swayosd_round_trip(input_css);
    }
}
