use starlark::environment::Globals;
use starlark::environment::Module;
use starlark::eval::Evaluator;
use starlark::syntax::AstModule;
use starlark::syntax::Dialect;
use starlark::values::Value;

pub mod bindings {
    use crate::StarlarkExecute;
    wit_bindgen::generate!({ generate_all });
    export!(StarlarkExecute);
}

use crate::bindings::exports::thomas9911::expression::expression::Guest;

pub struct StarlarkExecute;

impl Guest for StarlarkExecute {
    fn run(script: String) -> Result<String, String> {
        let ast: AstModule =
            AstModule::parse("hello_world.star", script.to_owned(), &Dialect::Standard).map_err(|e| e.to_string())?;

        let globals: Globals = Globals::standard();

        let module: Module = Module::new();

        let mut eval: Evaluator = Evaluator::new(&module);

        let res: Value = eval.eval_module(ast, &globals).map_err(|e| e.to_string())?;
        Ok(res.to_json().map_err(|e| e.to_string())?)
    }
}
