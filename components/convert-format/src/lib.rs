pub mod bindings {
    use crate::Converter;
    wit_bindgen::generate!({ generate_all });
    export!(Converter);
}
use base64::prelude::*;
use std::collections::BTreeMap;

use crate::bindings::exports::thomastimmer::convert_format::convert_format::{
    Guest, InputFormat, OutputFormat,
};

pub struct Converter;

impl Guest for Converter {
    fn convert(
        input: String,
        input_format: InputFormat,
        output_format: OutputFormat,
    ) -> Result<String, String> {
        // This can probably be faster by use or serde_erased or value-bag
        // because currently we always go to serde_value::Value as an intermediate format
        // probably for the csv implementation we always need to use serde_value::Value

        let data = convert_input(&input, input_format)?;
        format_output(&data, output_format)
    }
}

fn convert_input(input: &str, input_format: InputFormat) -> Result<serde_value::Value, String> {
    match input_format {
        InputFormat::Json => {
            let deserialized: serde_value::Value = serde_json::from_str(input)
                .map_err(|e| format!("JSON deserialization error: {}", e))?;
            Ok(deserialized)
        }
        InputFormat::Yaml => {
            let deserialized: serde_value::Value = serde_yaml_ng::from_str(input)
                .map_err(|e| format!("Yaml deserialization error: {}", e))?;
            Ok(deserialized)
        }
        InputFormat::Base64Cbor => {
            let bytes = BASE64_STANDARD
                .decode(input.as_bytes())
                .map_err(|e| format!("CBOR deserialization error: {}", e))?;
            let value: serde_value::Value = minicbor_serde::from_slice(&bytes)
                .map_err(|e| format!("CBOR deserialization error: {}", e))?;
            Ok(value)
        }
        InputFormat::Base64Msgpack => {
            let bytes = BASE64_STANDARD
                .decode(input.as_bytes())
                .map_err(|e| format!("MessagePack deserialization error: {}", e))?;
            let value: serde_value::Value = rmp_serde::from_slice(&bytes)
                .map_err(|e| format!("MessagePack deserialization error: {}", e))?;
            Ok(value)
        }
        InputFormat::Toml => {
            let deserialized: serde_value::Value =
                toml::from_str(input).map_err(|e| format!("TOML deserialization error: {}", e))?;
            Ok(deserialized)
        }
        InputFormat::Json5 => {
            let deserialized: serde_value::Value = serde_json5::from_str(input)
                .map_err(|e| format!("JSON5 deserialization error: {}", e))?;
            Ok(deserialized)
        }
        InputFormat::Csv => {
            let mut rdr = csv::Reader::from_reader(input.as_bytes());
            let mut data = Vec::new();
            for result in rdr.deserialize() {
                let record: BTreeMap<String, serde_value::Value> =
                    result.map_err(|e| format!("CSV deserialization error: {}", e))?;
                data.push(serde_value::Value::Map(
                    record
                        .into_iter()
                        .map(|(k, v)| (serde_value::Value::String(k), v))
                        .collect(),
                ));
            }
            Ok(serde_value::Value::Seq(data))
        }
    }
}

fn format_output(data: &serde_value::Value, output_format: OutputFormat) -> Result<String, String> {
    match output_format {
        OutputFormat::Json => {
            let serialized = serde_json::to_string(data)
                .map_err(|e| format!("JSON serialization error: {}", e))?;
            Ok(serialized)
        }
        OutputFormat::Yaml => {
            let serialized = serde_yaml_ng::to_string(data)
                .map_err(|e| format!("Yaml serialization error: {}", e))?;
            Ok(serialized)
        }
        OutputFormat::Base64Cbor => {
            let cbor = minicbor_serde::to_vec(data)
                .map_err(|e| format!("CBOR serialization error: {}", e))?;
            Ok(BASE64_STANDARD.encode(&cbor))
        }
        OutputFormat::Base64Msgpack => {
            let msgpack = rmp_serde::to_vec(data)
                .map_err(|e| format!("MessagePack serialization error: {}", e))?;
            Ok(BASE64_STANDARD.encode(&msgpack))
        }
        OutputFormat::Toml => {
            let serialized =
                toml::to_string(data).map_err(|e| format!("TOML serialization error: {}", e))?;
            Ok(serialized)
        }
        OutputFormat::Json5 => {
            let serialized = serde_json5::to_string(data)
                .map_err(|e| format!("JSON5 serialization error: {}", e))?;
            Ok(serialized)
        }
        OutputFormat::Csv => {
            let mut wtr = csv::Writer::from_writer(vec![]);
            let mut headers: Option<Vec<serde_value::Value>> = None;
            if let serde_value::Value::Seq(records) = data {
                for record in records {
                    match record {
                        serde_value::Value::Map(map) => {
                            if headers.is_none() {
                                let new_headers = make_csv_headers(map)?;
                                wtr.write_record(&new_headers)
                                    .map_err(|e| format!("CSV serialization error: {}", e))?;
                                headers = Some(
                                    new_headers
                                        .into_iter()
                                        .map(serde_value::Value::String)
                                        .collect(),
                                );
                            }
                            let header_keys = headers.as_ref().expect("headers just set");
                            let row: Vec<_> = header_keys
                                .iter()
                                .map(|h| map.get(&h))
                                .collect::<Option<Vec<_>>>()
                                .ok_or_else(|| {
                                    "CSV serialization error: Missing expected column".to_string()
                                })?;
                            wtr.serialize(row)
                                .map_err(|e| format!("CSV serialization error: {}", e))?;
                        }
                        _ => {
                            return Err("CSV serialization error: Expected map records".to_string());
                        }
                    }
                }
            } else {
                return Err("Expected a sequence of records for CSV output".to_string());
            }
            let bytes = wtr
                .into_inner()
                .map_err(|e| format!("CSV serialization error: {}", e))?;

            Ok(String::from_utf8(bytes).map_err(|e| format!("CSV serialization error: {}", e))?)
        }
    }
}

fn make_csv_headers(
    record: &BTreeMap<serde_value::Value, serde_value::Value>,
) -> Result<Vec<String>, String> {
    record
        .keys()
        .map(|k| match k {
            serde_value::Value::String(s) => Ok(s.clone()),
            _ => Err(format!(
                "CSV serialization error: Expected string keys for CSV headers"
            )),
        })
        .collect()
}

#[test]
fn csv_to_json_test() {
    use serde_value::Value;

    let csv_data = "name,age\nAlice,30\nBob,25\n";
    let out = convert_input(csv_data, InputFormat::Csv).unwrap();

    assert_eq!(
        out,
        Value::Seq(vec![
            Value::Map(
                vec![
                    (
                        Value::String("name".to_string()),
                        Value::String("Alice".to_string())
                    ),
                    (Value::String("age".to_string()), Value::U64(30))
                ]
                .into_iter()
                .collect()
            ),
            Value::Map(
                vec![
                    (
                        Value::String("name".to_string()),
                        Value::String("Bob".to_string())
                    ),
                    (Value::String("age".to_string()), Value::U64(25))
                ]
                .into_iter()
                .collect()
            )
        ])
    );

    assert_eq!(
        format_output(&out, OutputFormat::Json).unwrap(),
        r#"[{"age":30,"name":"Alice"},{"age":25,"name":"Bob"}]"#
    );
}

#[test]
fn json_to_csv_test() {
    use serde_value::Value;

    let json_data = r#"[{"name":"Alice","age":30},{"name":"Bob","age":25}]"#;
    let out = convert_input(json_data, InputFormat::Json).unwrap();

    assert_eq!(
        out,
        Value::Seq(vec![
            Value::Map(
                vec![
                    (
                        Value::String("name".to_string()),
                        Value::String("Alice".to_string())
                    ),
                    (Value::String("age".to_string()), Value::U64(30))
                ]
                .into_iter()
                .collect()
            ),
            Value::Map(
                vec![
                    (
                        Value::String("name".to_string()),
                        Value::String("Bob".to_string())
                    ),
                    (Value::String("age".to_string()), Value::U64(25))
                ]
                .into_iter()
                .collect()
            )
        ])
    );

    assert_eq!(
        format_output(&out, OutputFormat::Csv).unwrap(),
        "age,name\n30,Alice\n25,Bob\n"
    );
}

#[test]
fn json_to_yaml_list_test() {
    let json_data = r#"[{"name":"Alice","age":30},{"name":"Bob","age":25}]"#;
    let out = convert_input(json_data, InputFormat::Json).unwrap();

    assert_eq!(
        format_output(&out, OutputFormat::Yaml).unwrap(),
        "- age: 30\n  name: Alice\n- age: 25\n  name: Bob\n"
    );
}

#[test]
fn json_to_yaml_test() {
    let json_data = r#"{"name":"Alice","age":30}"#;
    let out = convert_input(json_data, InputFormat::Json).unwrap();

    assert_eq!(
        format_output(&out, OutputFormat::Yaml).unwrap(),
        "age: 30\nname: Alice\n"
    );
}

#[test]
fn yaml_to_csv_test() {
    let yaml_data = "- age: 30\n  name: Alice\n- age: 25\n  name: Bob\n";
    let out = convert_input(yaml_data, InputFormat::Yaml).unwrap();
    assert_eq!(
        format_output(&out, OutputFormat::Csv).unwrap(),
        "age,name\n30,Alice\n25,Bob\n"
    );
}

#[test]
fn json_cbor_roundtrip_test() {
    let json_data = r#"{"name":"Alice","age":30}"#;
    let out = convert_input(json_data, InputFormat::Json).unwrap();
    let cbor_base64 = format_output(&out, OutputFormat::Base64Cbor).unwrap();
    assert_eq!("omNhZ2UYHmRuYW1lZUFsaWNl", cbor_base64);
    let back = convert_input(&cbor_base64, InputFormat::Base64Cbor).unwrap();
    let json_output = format_output(&back, OutputFormat::Json).unwrap();
    let last = convert_input(&json_output, InputFormat::Json).unwrap();
    assert_eq!(out, last);
}

#[test]
fn json_msgpack_roundtrip_test() {
    let json_data = r#"{"name":"Alice","age":30}"#;
    let out = convert_input(json_data, InputFormat::Json).unwrap();
    let msgpack_base64 = format_output(&out, OutputFormat::Base64Msgpack).unwrap();
    assert_eq!("gqNhZ2UepG5hbWWlQWxpY2U=", msgpack_base64);
    let back = convert_input(&msgpack_base64, InputFormat::Base64Msgpack).unwrap();
    let json_output = format_output(&back, OutputFormat::Json).unwrap();
    let last = convert_input(&json_output, InputFormat::Json).unwrap();
    assert_eq!(out, last);
}

#[test]
fn toml_to_json_test() {
    let toml_data = r#"
    [keys]
    github = 'xxxxxxxxxxxxxxxxx'
    travis = 'yyyyyyyyyyyyyyyyy'
    "#;

    let out = convert_input(toml_data, InputFormat::Toml).unwrap();
    assert_eq!(
        out,
        serde_value::Value::Map(
            vec![(
                serde_value::Value::String("keys".to_string()),
                serde_value::Value::Map(
                    vec![
                        (
                            serde_value::Value::String("github".to_string()),
                            serde_value::Value::String("xxxxxxxxxxxxxxxxx".to_string())
                        ),
                        (
                            serde_value::Value::String("travis".to_string()),
                            serde_value::Value::String("yyyyyyyyyyyyyyyyy".to_string())
                        )
                    ]
                    .into_iter()
                    .collect()
                )
            )]
            .into_iter()
            .collect()
        )
    );

    assert_eq!(
        format_output(&out, OutputFormat::Json).unwrap(),
        r#"{"keys":{"github":"xxxxxxxxxxxxxxxxx","travis":"yyyyyyyyyyyyyyyyy"}}"#
    );

    assert_eq!(
        format_output(&out, OutputFormat::Yaml).unwrap(),
        "keys:\n  github: xxxxxxxxxxxxxxxxx\n  travis: yyyyyyyyyyyyyyyyy\n"
    );

    assert_eq!(
        format_output(&out, OutputFormat::Toml).unwrap(),
        r#"[keys]
github = "xxxxxxxxxxxxxxxxx"
travis = "yyyyyyyyyyyyyyyyy"
"#
    );
}

#[test]
fn json5_to_json_test() {
    let json5_data = r#"
    {
        // This is a comment
        name: 'Alice',
        age: 30,
        hobbies: ['reading', 'gaming', 'hiking'],
    }
    "#;

    let out = convert_input(json5_data, InputFormat::Json5).unwrap();
    assert_eq!(
        out,
        serde_value::Value::Map(
            vec![
                (
                    serde_value::Value::String("name".to_string()),
                    serde_value::Value::String("Alice".to_string())
                ),
                (
                    serde_value::Value::String("age".to_string()),
                    serde_value::Value::I64(30)
                ),
                (
                    serde_value::Value::String("hobbies".to_string()),
                    serde_value::Value::Seq(vec![
                        serde_value::Value::String("reading".to_string()),
                        serde_value::Value::String("gaming".to_string()),
                        serde_value::Value::String("hiking".to_string()),
                    ])
                )
            ]
            .into_iter()
            .collect()
        )
    );

    assert_eq!(
        format_output(&out, OutputFormat::Json).unwrap(),
        r#"{"age":30,"hobbies":["reading","gaming","hiking"],"name":"Alice"}"#
    );
}
