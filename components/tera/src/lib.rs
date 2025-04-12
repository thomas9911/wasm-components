pub mod bindings {
    use crate::TeraRenderer;
    // wit_bindgen::generate!({with: { "thomas9911:template/template@0.1.0": generate, }});
    wit_bindgen::generate!({ generate_all });
    export!(TeraRenderer);
}
use crate::bindings::exports::thomas9911::template::template::Guest;

use tera::{Tera, Context};

pub struct TeraRenderer;

impl Guest for TeraRenderer {
    fn render(template: String, json_variables: String) -> Result<String, String> {
        let vars: serde_json::Value = serde_json::from_str(&json_variables).map_err(|e| e.to_string())?;
        let context = Context::from_serialize(&vars).map_err(|e| e.to_string())?;
        let result = Tera::one_off(&template, &context, true).map_err(|e| e.to_string())?;
        Ok(result)
    }
}
