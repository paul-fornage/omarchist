use super::ConfigGenerator;
use serde_json::{json, Value};
use regex::Regex;

pub struct HyprlandGenerator;

unsafe impl Send for HyprlandGenerator {}
unsafe impl Sync for HyprlandGenerator {}

impl ConfigGenerator for HyprlandGenerator {
    fn get_app_name(&self) -> &'static str {
        "hyprland"
    }

    fn get_file_name(&self) -> &'static str {
        "hyprland.conf"
    }

    fn generate_config(&self, theme_data: &Value) -> Result<String, String> {
        let empty_obj = json!({});
        let hyprland = theme_data.get("hyprland").unwrap_or(&empty_obj);

        // Extract color values with defaults from template
        let general = hyprland.get("general").unwrap_or(&empty_obj);
        let mut active_border = general
            .get("active_border")
            .and_then(|a| a.as_str())
            .unwrap_or("8A8A8D")
            .to_string();
        // normalize to hex without leading '#'
        if active_border.starts_with('#') {
            active_border = active_border.trim_start_matches('#').to_string();
        }
        let mut inactive_border = general
            .get("inactive_border")
            .and_then(|a| a.as_str())
            .unwrap_or("5C5C5E")
            .to_string();
        // normalize to hex without leading '#'
        if inactive_border.starts_with('#') {
            inactive_border = inactive_border.trim_start_matches('#').to_string();
        }
        let border_size = general
            .get("border_size")
            .and_then(|a| a.as_u64())
            .unwrap_or(1)
            .to_string();
        let gaps_in = general
            .get("gaps_in")
            .and_then(|a| a.as_u64())
            .unwrap_or(5)
            .to_string();
        let gaps_out = general
            .get("gaps_out")
            .and_then(|a| a.as_u64())
            .unwrap_or(20)
            .to_string();

        let decoration = hyprland.get("decoration").unwrap_or(&empty_obj);
        let rounding = decoration
            .get("rounding")
            .and_then(|a| a.as_u64())
            .unwrap_or(0)
            .to_string();

        Ok(format!(
            r#"# ────────────────────────────────────────────────────────────
# Omarchy Custom Theme for Hyprland
# Generated with Omarchist
# ────────────────────────────────────────────────────────────

general {{
    col.active_border = rgb({active_border})
    col.inactive_border = rgb({inactive_border})
    border_size = {border_size}
    gaps_in = {gaps_in}
    gaps_out = {gaps_out}
}}

decoration {{
    rounding = {rounding}
}}
"#
        ))
    }

    fn get_config_schema(&self) -> Value {
        json!({
            "type": "object",
            "x-order": ["general", "decoration"],
            "properties": {
                "general": {
                    "type": "object",
                    "x-order": ["active_border", "inactive_border", "border_size", "gaps_in", "gaps_out"],
                    "properties": {
                        "active_border": {
                            "type": "string",
                            "title": "Active Border",
                            "format": "color",
                            "description": "border color for the active window",
                            "output_format": "hex-no-hash",
                            "default": "8A8A8D",
                        },
                        "inactive_border": {
                            "type": "string",
                            "title": "Inactive Border",
                            "format": "color",
                            "description": "border color for inactive windows",
                            "output_format": "hex-no-hash",
                            "default": "5C5C5E",
                        },
                        "border_size": {
                            "type": "number",
                            "title": "Border Size",
                            "description": "size of the border around windows",
                            "default": 1
                        },
                        "gaps_in": {
                            "type": "number",
                            "title": "Gaps In",
                            "description": "gaps between windows",
                            "default": 5
                        },
                        "gaps_out": {
                            "type": "number",
                            "title": "Gaps Out",
                            "description": "gaps between windows and monitor edges",
                            "default": 20
                        }
                    }
                },
                "decoration": {
                    "type": "object",
                    "properties": {
                        "rounding": {
                            "type": "number",
                            "title": "Rounding",
                            "description": "rounded corners' radius",
                            "default": 0
                        }
                    }
                }
            }
        })
    }

    fn parse_existing_config(&self, content: &str) -> Result<Value, String> {
        // Helper to capture a block's body: e.g., general { ... } or decoration { ... }
        let block_body = |name: &str| -> Option<String> {
            let re = Regex::new(&format!(r"(?s){}\s*\{{(.*?)\}}", regex::escape(name))).ok()?;
            re.captures(content).and_then(|c| c.get(1)).map(|m| m.as_str().to_string())
        };

        // Extract values from "general" block
        let mut general_obj = serde_json::Map::new();
        if let Some(body) = block_body("general") {
            // Colors in form rgb(HEX) or rgb(#HEX); capture 6 hex digits
            let re_active = Regex::new(r"col\.active_border\s*=\s*rgb\(\s*#?([0-9A-Fa-f]{6})\s*\)").unwrap();
            let re_inactive = Regex::new(r"col\.inactive_border\s*=\s*rgb\(\s*#?([0-9A-Fa-f]{6})\s*\)").unwrap();
            if let Some(cap) = re_active.captures(&body) {
                general_obj.insert("active_border".to_string(), json!(cap[1].to_uppercase()));
            }
            if let Some(cap) = re_inactive.captures(&body) {
                general_obj.insert("inactive_border".to_string(), json!(cap[1].to_uppercase()));
            }

            let grab_num = |key: &str| -> Option<u64> {
                let re = Regex::new(&format!(r"{}\s*=\s*([0-9]+)", regex::escape(key))).ok()?;
                re.captures(&body).and_then(|c| c.get(1)).and_then(|m| m.as_str().parse::<u64>().ok())
            };

            if let Some(v) = grab_num("border_size") {
                general_obj.insert("border_size".to_string(), json!(v));
            }
            if let Some(v) = grab_num("gaps_in") {
                general_obj.insert("gaps_in".to_string(), json!(v));
            }
            if let Some(v) = grab_num("gaps_out") {
                general_obj.insert("gaps_out".to_string(), json!(v));
            }
        }

        // Extract values from "decoration" block
        let mut decoration_obj = serde_json::Map::new();
        if let Some(body) = block_body("decoration") {
            let re_rounding = Regex::new(r"rounding\s*=\s*([0-9]+)").unwrap();
            if let Some(cap) = re_rounding.captures(&body) {
                if let Ok(v) = cap[1].parse::<u64>() {
                    decoration_obj.insert("rounding".to_string(), json!(v));
                }
            }
        }

        // Assemble final JSON
        let mut hyprland_obj = serde_json::Map::new();
        if !general_obj.is_empty() {
            hyprland_obj.insert("general".to_string(), Value::Object(general_obj));
        }
        if !decoration_obj.is_empty() {
            hyprland_obj.insert("decoration".to_string(), Value::Object(decoration_obj));
        }

        Ok(json!({ "hyprland": hyprland_obj }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value as JsonValue;

    /// Strictly compare only the essential lines from Hyprland config
    /// (lines that would actually get read by the parser)
    fn normalize_raw_hyprland(raw: &str) -> String {
        let mut out = String::new();
        for line in raw.lines() {
            let line = line.trim();
            if line.starts_with("col.active_border")
                || line.starts_with("col.inactive_border")
                || line.starts_with("border_size")
                || line.starts_with("gaps_in")
                || line.starts_with("gaps_out")
                || line.starts_with("rounding")
            {
                out.push_str(line);
            }
        }
        out
    }

    fn hyprland_round_trip(input_conf: &str) {
        let generator = HyprlandGenerator;

        // Parse -> JSON
        let parsed1: JsonValue = generator
            .parse_existing_config(input_conf)
            .expect("parse_existing_config should succeed");

        // Generate -> conf
        let regenerated_conf = generator
            .generate_config(&parsed1)
            .expect("generate_config should succeed");

        // Parse regenerated -> JSON
        let parsed2: JsonValue = generator
            .parse_existing_config(&regenerated_conf)
            .expect("parse_existing_config on regenerated config should succeed");

        assert_eq!(
            parsed1, parsed2,
            "Hyprland parsed JSON before and after regeneration should be identical"
        );

        // Strict field-level structural check: compare extracted lines
        assert_eq!(
            normalize_raw_hyprland(input_conf),
            normalize_raw_hyprland(&regenerated_conf),
            "Regenerated config should match the key-value structure of the input config"
        );
    }

    #[test]
    fn hyprland_round_trip_is_lossless_with_extra_fields() {

        let input_conf = r#"
# Example Hyprland config
general {
    col.active_border = rgb(8A8A8D)
    col.inactive_border = rgb(5C5C5E)
    border_size = 2
    gaps_in = 6
    gaps_out = 22
}

# Bad value
exec-once = uwsm app -- waybar

decoration {
    rounding = 4
}
"#;
        hyprland_round_trip(input_conf);
    }

    #[test]
    fn hyprland_round_trip_is_lossless() {

        let input_conf = r#"
# Example Hyprland config
general {
    col.active_border = rgb(8A8A8D)
    col.inactive_border = rgb(5C5C5E)
    border_size = 2
    gaps_in = 6
    gaps_out = 22
}

decoration {
    rounding = 4
}
"#;
        hyprland_round_trip(input_conf);
    }

    #[test]
    fn hyprland_parse_catches_missing_field_errors() {
        let input_conf_missing = r#"
general {
    col.active_border = rgb(8A8A8D)
    border_size = 2
    gaps_in = 6
    # gaps_out is missing
}
decoration {
    rounding = 0
}
"#;
        let generator = HyprlandGenerator;
        let parsed = generator.parse_existing_config(input_conf_missing).unwrap();

        // It should NOT contain "gaps_out"
        assert!(!parsed["hyprland"]["general"].as_object().unwrap().contains_key("gaps_out"), "Should not parse missing gaps_out field");
    }

    #[test]
    fn hyprland_parse_handles_weird_spacing_and_casing() {
        let input_conf_weird = r#"
general {
    col.active_border    =   rgb(8a8a8d   )
    col.inactive_border=rgb(#5c5c5e)
    border_size=1
    gaps_out=12
    gaps_in =8
}
decoration
{
    rounding    =    5
}
"#;
        let generator = HyprlandGenerator;
        let parsed = generator.parse_existing_config(input_conf_weird).unwrap();
        let general = &parsed["hyprland"]["general"];
        assert_eq!(general["active_border"], "8A8A8D");
        assert_eq!(general["inactive_border"], "5C5C5E");
        assert_eq!(general["border_size"], 1);
        assert_eq!(general["gaps_in"], 8);
        assert_eq!(general["gaps_out"], 12);
        let decoration = &parsed["hyprland"]["decoration"];
        assert_eq!(decoration["rounding"], 5);
    }
}
