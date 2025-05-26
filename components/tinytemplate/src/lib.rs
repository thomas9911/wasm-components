pub mod bindings {
    use crate::TinyTemplateRenderer;
    // wit_bindgen::generate!({with: { "thomastimmer:template/template@0.1.0": generate, }});
    wit_bindgen::generate!({ generate_all });
    export!(TinyTemplateRenderer);
}
use crate::bindings::exports::thomastimmer::template::template::Guest;

use tinytemplate::TinyTemplate;

pub struct TinyTemplateRenderer;

impl Guest for TinyTemplateRenderer {
    fn render(template: String, json_variables: String) -> Result<String, String> {
        let mut tt = TinyTemplate::new();
        tt.add_template("main", &template).map_err(|e| e.to_string())?;
    
        let vars: serde_json::Value = serde_json::from_str(&json_variables).map_err(|e| e.to_string())?;
    
        Ok(tt.render("main", &vars).map_err(|e| e.to_string())?)
    }
}
