use super::ConfigGenerator;
use serde_json::{json, Value};
use crate::services::generators::css_parser;

pub struct WalkerGenerator;

unsafe impl Send for WalkerGenerator {}
unsafe impl Sync for WalkerGenerator {}

impl ConfigGenerator for WalkerGenerator {
    fn get_app_name(&self) -> &'static str {
        "walker"
    }

    fn get_file_name(&self) -> &'static str {
        "walker.css"
    }

    fn generate_config(&self, theme_data: &Value) -> Result<String, String> {
        let empty_obj = json!({});
        let walker = theme_data.get("walker").unwrap_or(&empty_obj);

        // Extract color values with defaults from template
        let colors = walker.get("colors").unwrap_or(&empty_obj);

        let selected_text = colors
            .get("selected_text")
            .and_then(|st| st.as_str())
            .unwrap_or("#B91C1C");
        let text = colors
            .get("text")
            .and_then(|t| t.as_str())
            .unwrap_or("#EAEAEA");
        let base = colors
            .get("base")
            .and_then(|b| b.as_str())
            .unwrap_or("#121212");
        let border = colors
            .get("border")
            .and_then(|br| br.as_str())
            .unwrap_or("#EAEAEA88");
        let foreground = colors
            .get("foreground")
            .and_then(|fg| fg.as_str())
            .unwrap_or("#EAEAEA");
        let background = colors
            .get("background")
            .and_then(|bg| bg.as_str())
            .unwrap_or("#121212");

        Ok(format!(
            r#"/* ────────────────────────────────────────────────────────────
 * Omarchy Custom Theme for Walker
 * Generated with Omarchist
 * ────────────────────────────────────────────────────────────
 */

@define-color selected-text {selected_text};
@define-color text {text};
@define-color base {base};
@define-color border {border};
@define-color foreground {foreground};
@define-color background {background};
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
                    "selected_text": {
                        "type": "string",
                        "format": "color",
                        "default": "#B91C1C",
                        "title": "Selected Text Color",
                        "description": "Color of selected text"
                    },
                    "text": {
                        "type": "string",
                        "format": "color",
                        "default": "#EAEAEA",
                        "title": "Text Color",
                        "description": "Color of normal text"
                    },
                    "base": {
                        "type": "string",
                        "format": "color",
                        "default": "#121212",
                        "title": "Base Color",
                        "description": "Base background color"
                    },
                    "border": {
                        "type": "string",
                        "format": "color",
                        "output_format": "hex-alpha",
                        "default": "EAEAEA88",
                        "title": "Border Color",
                        "description": "Border color (can include alpha)"
                    },
                    "foreground": {
                        "type": "string",
                        "format": "color",
                        "default": "#EAEAEA",
                        "title": "Foreground Color",
                        "description": "Primary foreground color"
                    },
                    "background": {
                        "type": "string",
                        "format": "color",
                        "default": "#121212",
                        "title": "Background Color",
                        "description": "Primary background color"
                    }
                    }
                }
            }
        })
    }

    fn parse_existing_config(&self, content: &str) -> Result<Value, String> {
        let css_keys = [
            "selected-text",
            "text",
            "base",
            "border",
            "foreground",
            "background",
        ];

        let colors_obj = css_parser(content, &css_keys)?;

        let mut walker_obj = serde_json::Map::new();
        if !colors_obj.is_empty() {
            walker_obj.insert("colors".into(), Value::Object(colors_obj));
        }
        Ok(json!({ "walker": walker_obj }))
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    fn walker_json_round_trip(input_json: &Value) {
        let generator = WalkerGenerator;

        // Generate config from JSON
        let generated_conf = generator
            .generate_config(input_json)
            .expect("generate_config should succeed");

        // Parse back config -> JSON
        let parsed_json: Value = generator
            .parse_existing_config(&generated_conf)
            .expect("parse_existing_config should succeed");

        assert_eq!(
            parsed_json, *input_json,
            "Walker JSON before and after config generation/parsing should be identical"
        );
    }

    #[test]
    fn walker_json_round_trip_is_lossless() {
        let input_json = json!({
            "walker": {
                "colors": {
                    "selected_text": "#B91C1C",
                    "text": "#EAEAEA",
                    "base": "#121212",
                    "border": "#EAEAEA88",
                    "foreground": "#abc123",
                    "background": "#222222"
                }
            }
        });
        walker_json_round_trip(&input_json);
    }

    fn walker_round_trip(input_conf: &str) {
        let generator = WalkerGenerator;

        // Parse -> JSON
        let parsed1: Value = generator
            .parse_existing_config(input_conf)
            .expect("parse_existing_config should succeed");

        // Generate -> walker.css
        let regenerated_conf = generator
            .generate_config(&parsed1)
            .expect("generate_config should succeed");

        // Parse regenerated -> JSON
        let parsed2: Value = generator
            .parse_existing_config(&regenerated_conf)
            .expect("parse_existing_config on regenerated config should succeed");

        assert_eq!(
            parsed1, parsed2,
            "Walker parsed JSON before and after regeneration should be identical"
        );
    }

    #[test]
    fn walker_round_trip_is_lossless() {
        let input_conf = r#"
@define-color selected-text #B91C1C;
@define-color text #EAEAEA;
@define-color base #121212;
@define-color border #EAEAEA88;
@define-color foreground #abc123;
@define-color background #222222;
"#;
        walker_round_trip(input_conf);
    }

    #[test]
    fn walker_round_trip_accepts_extra_whitespace() {
        let input_conf = r#"
    @define-color   selected-text   #b91c1c  ;
@define-color    text    #eaeaea;
@define-color base   #121212     ;
@define-color border #eaeaea88;
      @define-color foreground #eaeaea;
   @define-color background #121212;
"#;
        walker_round_trip(input_conf);
    }

    #[test]
    fn walker_parses_partial_config() {
        let input_conf = r#"
@define-color text #dedede;
@define-color background #111111;
"#;
        let parsed = WalkerGenerator
            .parse_existing_config(input_conf)
            .unwrap();
        assert_eq!(
            parsed,
            json!({"walker": { "colors": { "text": "#dedede", "background": "#111111" }}})
        );
    }
}
