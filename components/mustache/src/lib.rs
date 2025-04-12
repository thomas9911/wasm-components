pub mod bindings {
    use crate::MustacheRenderer;
    // wit_bindgen::generate!({with: { "thomas9911:template/template@0.1.0": generate, }});
    wit_bindgen::generate!({ generate_all });
    export!(MustacheRenderer);
}
use crate::bindings::exports::thomas9911::template::template::Guest;

pub struct MustacheRenderer;

impl Guest for MustacheRenderer {
    fn render(template: String, json_variables: String) -> Result<String, String> {
        let template = mustache::compile_str(&template).map_err(|e| e.to_string())?;
        let data: serde_json::Value = serde_json::from_str(&json_variables).map_err(|e| e.to_string())?;
        Ok(template.render_to_string(&data).map_err(|e| e.to_string())?)
    }
}
