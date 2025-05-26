pub mod bindings {
    use crate::HandlebarsRenderer;
    // wit_bindgen::generate!({with: { "thomastimmer:template/template@0.1.0": generate, }});
    wit_bindgen::generate!({ generate_all });
    export!(HandlebarsRenderer);
}
use crate::bindings::exports::thomastimmer::template::template::Guest;
use handlebars::Handlebars;

pub struct HandlebarsRenderer;

impl Guest for HandlebarsRenderer {
    fn render(template: String, json_variables: String) -> Result<String, String> {
        let reg = Handlebars::new();

        let vars: serde_json::Value =
            serde_json::from_str(&json_variables).map_err(|e| e.to_string())?;

        Ok(reg.render_template(&template, &vars).map_err(|e| e.to_string())?)
    }
}
