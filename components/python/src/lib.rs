pub mod bindings {
    use crate::PythonExecute;
    wit_bindgen::generate!({ generate_all });
    export!(PythonExecute);
}

use crate::bindings::exports::thomastimmer::expression::expression::Guest;
use crate::bindings::exports::thomastimmer::python::extra;

use rustpython::vm::{
    self,
    builtins::{PyDict, PyStr},
    scope::Scope,
    PyRef,
};
use rustpython::vm::{PyResult, VirtualMachine};

pub struct PythonExecute;

fn load_context(vm: &VirtualMachine, scope: Scope, json_data: &str) -> PyResult<Scope> {
    if let Ok(extra_data) = dump_json(vm, json_data) {
        for (key, value) in extra_data.into_iter() {
            let k = key.try_downcast::<PyStr>(vm)?;
            scope.locals.set_item(k.as_str(), value, vm)?;
        }

        return Ok(scope);
    }

    Ok(scope)
}

fn dump_json(vm: &VirtualMachine, json_data: &str) -> PyResult<PyRef<PyDict>> {
    let json = vm.import("json", 0)?;
    let json_loads = json.get_attr("loads", vm)?;
    let dict = json_loads.call((vm.ctx.new_str(json_data),), vm)?;
    dict.try_downcast(vm)
}

fn running(vm: &VirtualMachine, script: &str, json_data: &str) -> PyResult<String> {
    let scope = vm.new_scope_with_builtins();
    let scope = load_context(vm, scope, json_data)?;

    let source = script;
    let code_obj = vm
        .compile(
            source,
            vm::compiler::Mode::BlockExpr,
            "<embedded>".to_owned(),
        )
        .map_err(|err| vm.new_syntax_error(&err, Some(source)))?;

    let out = vm.run_code_obj(code_obj, scope)?;

    Ok(out.str(vm)?.to_string())
}

impl Guest for PythonExecute {
    fn run(script: String) -> Result<String, String> {
        use extra::Guest;

        Self::run_with_context(script, String::new())
    }
}

impl extra::Guest for PythonExecute {
    fn run_with_context(script: String, json_data: String) -> Result<String, String> {
        vm::Interpreter::with_init(Default::default(), |vm| {
            // put this line to add stdlib to the vm
            vm.add_native_modules(rustpython_stdlib::get_module_inits());
            vm.add_frozen(rustpython_pylib::FROZEN_STDLIB);
        })
        .enter(|vm| {
            running(vm, &script, &json_data).map_err(|e| {
                let mut error = String::new();
                match vm.write_exception(&mut error, &e) {
                    Ok(_) => error,
                    Err(write_error) => write_error.to_string(),
                }
            })
        })
    }
}
