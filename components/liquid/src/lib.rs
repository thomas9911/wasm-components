pub mod bindings {
    use crate::LiquidRenderer;
    // wit_bindgen::generate!({with: { "thomas9911:template/template@0.1.0": generate, }});
    wit_bindgen::generate!({ generate_all });
    export!(LiquidRenderer);
}
use crate::bindings::exports::thomas9911::template::template::Guest;

pub struct LiquidRenderer;

impl Guest for LiquidRenderer {
    fn render(template: String, json_variables: String) -> Result<String, String> {
        let template = liquid::ParserBuilder::with_stdlib()
            .build()
            .map_err(|e| e.to_string())?
            .parse(&template)
            .map_err(|e| e.to_string())?;

        let vars: liquid::Object =
            serde_json::from_str(&json_variables).map_err(|e| e.to_string())?;

        Ok(template.render(&vars).map_err(|e| e.to_string())?)
    }
}
