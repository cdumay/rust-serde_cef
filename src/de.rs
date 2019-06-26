use std::collections::BTreeMap;

use serde::Deserialize;
use serde::export::fmt::Debug;
use serde_value::Value;

use crate::record::{CefRecord, CefSeverity, CefSignatureId};
use crate::result::CefResult;
use crate::utils::try_detect_type;

fn parse_extensions(extensions: &str) -> BTreeMap<Value, Value> {
    let mut result = BTreeMap::new();
    let mut start = 0;
    let mut sstart = 0;
    let mut pos = Some(0);
    let mut out: Vec<String> = vec![];

    while pos != None {
        pos = extensions[sstart..].find("=");
        match pos {
            None => out.push(extensions[start..].to_string()),
            Some(idx) => {
                match &extensions[(sstart + idx - 1)..(sstart + idx)] {
                    "\\" => sstart += idx + 1,
                    _ => {
                        let skey: Vec<&str> = extensions[start..(sstart + idx)].split(' ').collect();
                        let lenght = skey.len();
                        match lenght > 1 {
                            true => {
                                out.push(skey[0..(lenght - 1)].join(" "));
                                out.push(skey[lenght - 1].to_string());
                            }
                            false => out.push(extensions[start..(sstart + idx)].to_string())
                        };
                        sstart += idx + 1;
                        start = sstart;
                    }
                }
            }
        }
    }
    if (out.len() % 2) == 0 {
        for idx in (0..out.len()).step_by(2) {
            result.insert(
                Value::String(out[idx].clone()), try_detect_type(&out[idx + 1].clone()),
            );
        }
    }
    result
}

/// Deserialize an instance of type `CefRecord<T>` from a string of CEF text.
#[inline]
pub fn from_str<'a, T>(value: &'a str) -> CefResult<CefRecord<T>> where T: Deserialize<'a> + Debug {
    let re = regex::Regex::new(match value.to_string().starts_with("CEF") {
        true => r"^CEF:(?P<version>\d)\|(?P<device_vendor>[^\|]+)\|(?P<device_product>[^\|]+)\|(?P<device_version>[^\|]+)\|(?P<signature_id>[^\|]+)\|(?P<signature>[^\|]+)\|(?P<severity>[^\|]+)\|(?P<extensions>.*)",
        false => r"^(?P<headers>.*)CEF:(?P<version>\d)\|(?P<device_vendor>[^\|]+)\|(?P<device_product>[^\|]+)\|(?P<device_version>[^\|]+)\|(?P<signature_id>[^\|]+)\|(?P<signature>[^\|]+)\|(?P<severity>[^\|]+)\|(?P<extensions>.*)",
    })?;
    let captures = re.captures(value)?;

    Ok(CefRecord {
        headers: match captures.name("headers") {
            Some(data) => Some(data.as_str().to_string()),
            None => None
        },
        version: captures.name("version")?.as_str().parse::<u8>()?,
        device_vendor: captures.name("device_vendor")?.as_str().to_string(),
        device_product: captures.name("device_product")?.as_str().to_string(),
        device_version: captures.name("device_version")?.as_str().to_string(),
        signature_id: CefSignatureId::from(captures.name("signature_id")?.as_str()),
        signature: captures.name("signature")?.as_str().to_string(),
        severity: CefSeverity::from(captures.name("severity")?.as_str()),
        extensions: Value::Map(parse_extensions(
            captures.name("extensions")?.as_str()
        )).deserialize_into()?,
    })
}
