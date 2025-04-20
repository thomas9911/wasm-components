pub mod bindings {
    use crate::PythonExecute;
    wit_bindgen::generate!({ generate_all });
    export!(PythonExecute);
}

use crate::bindings::exports::thomas9911::expression::expression::Guest;

use rustpython::vm;
use rustpython::vm::{PyResult, VirtualMachine};

pub struct PythonExecute;

fn running(vm: &VirtualMachine, script: &str) -> PyResult<String> {
    let scope = vm.new_scope_with_builtins();
    let source = script;
    let code_obj = vm
        .compile(source, vm::compiler::Mode::BlockExpr, "<embedded>".to_owned())
        .map_err(|err| vm.new_syntax_error(&err, Some(source)))?;

    let out = vm.run_code_obj(code_obj, scope)?;

    Ok(out.str(vm)?.to_string())
}

impl Guest for PythonExecute {
    fn run(script: String) -> Result<String, String> {
         vm::Interpreter::with_init(Default::default(), |vm| {
            // put this line to add stdlib to the vm
            // vm.add_native_modules(rustpython_stdlib::get_module_inits());
            vm.add_frozen(rustpython_pylib::FROZEN_STDLIB);
        })
        // rustpython::InterpreterConfig::new()
        // .init_stdlib()
        // .interpreter()
        .enter(|vm| {
            running(vm, &script).map_err(|e| {
                let mut error = String::new();
                match vm.write_exception(&mut error, &e) {
                    Ok(_) => error,
                    Err(write_error) => write_error.to_string(),
                }
            })
        })
    }
}
