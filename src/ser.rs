use serde::Serialize;
use serde_value::Value;

use crate::record::CefRecord;
use crate::result::CefResult;

fn value2str(value: Value) -> String {
    match value {
        Value::String(data) => data,
        Value::Bool(data) => data.to_string(),
        Value::U8(data) => format!("{}", data),
        Value::U16(data) => format!("{}", data),
        Value::U32(data) => format!("{}", data),
        Value::U64(data) => format!("{}", data),
        Value::I8(data) => format!("{}", data),
        Value::I16(data) => format!("{}", data),
        Value::I32(data) => format!("{}", data),
        Value::I64(data) => format!("{}", data),
        Value::F32(data) => format!("{}", data),
        Value::F64(data) => format!("{}", data),
        Value::Char(data) => format!("{}", data),
        _ => panic!("Invalid data: {:?}", value)
    }
}

/// Serialize an instance of type `CefRecord<T>` into a string of CEF text.
#[inline]
pub fn to_string<T: ?Sized>(value: &CefRecord<T>) -> CefResult<String> where T: Serialize {
    let mut fields = String::new();
    if let Some(headers) = value.headers.clone() {
        fields += &headers;
    }
    fields += &format!(
        "CEF:{}|{}|{}|{}|{}|{}|{}|",
        value.version, value.device_vendor, value.device_product, value.device_version,
        value.signature_id, value.signature, value.severity
    );

    let mut extensions: Vec<String> = vec![];
    for (k, v) in serde_value_flatten::to_flatten_maptree("_", None, &value.extensions)? {
        extensions.push(format!("{}={}", value2str(k), value2str(v)));
    }
    fields += &extensions.join(" ");
    Ok(fields)
}