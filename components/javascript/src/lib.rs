pub mod bindings {
    use crate::JavascriptExecute;
    wit_bindgen::generate!({ generate_all });
    export!(JavascriptExecute);
}

use rquickjs::{
    CatchResultExt, CaughtError, Context, Ctx, Runtime, Value,
};

use crate::bindings::exports::thomas9911::expression::expression::Guest;

pub struct JavascriptExecute;

fn into_json<'a>(value: Value<'a>, ctx: &Ctx<'a>) -> Result<String, String> {
    match ctx.json_stringify(value).map_err(|e| e.to_string())? {
        Some(x) => Ok(x.to_string().map_err(|e| e.to_string())?),
        None => Ok("null".to_string()),
    }
}

fn running(script: &str) -> Result<String, String> {
    let rt = Runtime::new().map_err(|e| e.to_string())?;
    let ctx = Context::full(&rt).map_err(|e| e.to_string())?;

    let out = ctx.with(|ctx| -> Result<_, String> {
        let val = ctx
            .eval::<Value, _>(script.as_bytes())
            .catch(&ctx)
            .map_err(|e| match e {
                CaughtError::Error(e) => e.to_string(),
                CaughtError::Exception(e) => e.to_string(),
                CaughtError::Value(e) => match into_json(e, &ctx) {
                    Ok(x) => x,
                    Err(e) => e,
                },
            })?;
        into_json(val, &ctx)
    })?;

    Ok(out)
}

impl Guest for JavascriptExecute {
    fn run(script: String) -> std::result::Result<String, String> {
        running(&script).map_err(|e| e.to_string())
    }
}

#[test]
fn runs_returns_int() {
    let out = running("1 + 3").unwrap();
    assert_eq!(out, "4".to_string())
}

#[test]
fn runs_returns_string() {
    let out = running("'hello' + ' ' + 'world'").unwrap();
    assert_eq!(out, r#""hello world""#.to_string())
}

#[test]
fn runs_returns_object() {
    let out = running(
        r#"function main() {
    return {test: 1}
}

main()"#,
    )
    .unwrap();
    assert_eq!(out, r#"{"test":1}"#.to_string())
}
