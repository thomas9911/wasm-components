pub mod bindings {
    use crate::RunIt;
    wit_bindgen::generate!({ generate_all });
    export!(RunIt);
}
use crate::bindings::thomas9911::template::template::render;
use crate::bindings::thomas9911::expression::expression::run;
use crate::bindings::exports::thomas9911::run_template::out::Guest;

pub struct RunIt;

impl Guest for RunIt {
    fn run(template: String, json_variables: String) -> Result<String, String> {
        let template = render(&template, &json_variables)?;
        let out = run(&template)?;
        Ok(out)
    }
}
