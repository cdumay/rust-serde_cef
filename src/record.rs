use std::fmt;

use chrono::{Datelike, Local, NaiveDateTime};

use crate::result::CefResult;

/// Severity is a string or integer and reflects the importance of the event.
///
/// The valid string values are:
/// * `Unknown`
/// * `Low`
/// * `Medium`
/// * `High`
/// * `Very-High`
///
/// The valid integer values are:
/// * `0-3`  = `Low`
/// * `4-6`  = `Medium`
/// * `7-8`  = `High`
/// * `9-10` = `Very-High`
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum CefSeverity {
    String(String),
    U8(u8),
}

impl From<&str> for CefSeverity {
    fn from(raw: &str) -> CefSeverity {
        match raw.parse::<u8>() {
            Ok(data) => {
                if data > 10 {
                    panic!("Invalid Cef Level, MUST be between 0 to 10")
                }
                CefSeverity::U8(data)
            }
            Err(_) => {
                match raw {
                    "Unknown" | "Low" | "Medium" | "High" | "Very-High" => CefSeverity::String(raw.into()),
                    _ => panic!("Invalid Cef Level, MUST be between Unknown, Low, Medium, High or Very-High")
                }
            }
        }
    }
}

impl From<CefSeverity> for u8 {
    fn from(severity: CefSeverity) -> u8 {
        match severity {
            CefSeverity::String(desc) => match desc.as_str() {
                "Unknown" => 0,
                "Low" => 1,
                "Medium" => 4,
                "High" => 7,
                "Very-High" => 9,
                _ => 0
            },
            CefSeverity::U8(level) => level
        }
    }
}

impl fmt::Display for CefSeverity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CefSeverity::String(data) => write!(f, "{}", data),
            CefSeverity::U8(data) => write!(f, "{}", data)
        }
    }
}

/// Device Event Class ID is a unique identifier per event-type.
///
/// This can be a string or an integer.
/// Device Event Class ID identifies the type of event reported. In the intrusion detection system (IDS)
/// world, each signature or rule that detects certain activity has a unique Device Event Class ID
/// assigned. This is a requirement for other types of devices as well, and helps correlation engines
/// process the events. Also known as Signature ID.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum CefSignatureId {
    String(String),
    U64(u64),
}

impl fmt::Display for CefSignatureId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CefSignatureId::String(data) => write!(f, "{}", data),
            CefSignatureId::U64(data) => write!(f, "{}", data)
        }
    }
}

impl From<&str> for CefSignatureId {
    fn from(raw: &str) -> CefSignatureId {
        match raw.parse::<u64>() {
            Ok(data) => CefSignatureId::U64(data),
            Err(_) => CefSignatureId::String(raw.to_string())
        }
    }
}

/// Struct which represent a CEF record according to the specification.
///
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CefRecord<T: ?Sized> {
    /// Optional Syslog headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<String>,
    /// An integer which identifies the version of the CEF format. The current CEF version is `0`.
    pub version: u8,
    /// Identify the device vendor.
    pub device_vendor: String,
    /// Identify the device name.
    pub device_product: String,
    /// Identify the device version.
    pub device_version: String,
    /// Signature ID also known as _Device Event Class ID_ identifies the type of event reported.
    pub signature_id: CefSignatureId,
    /// Representing a human-readable and understandable description of the event.
    pub signature: String,
    /// Reflects the importance of the event.
    ///
    /// The valid string values are `Unknown`, `Low`, `Medium`, `High`, and `Very-High`.
    /// The valid integer values are `0-3`=`Low`, `4-6`=`Medium`,`7-8`=`High`, and `9-10`=`Very-High`.
    pub severity: CefSeverity,
    /// Contains a collection of key-value pairs. The keys are part of a predefined set.
    pub extensions: T,
}

impl<T: ?Sized> CefRecord<T> {
    pub fn extract_ts_from_headers(headers: &str) -> CefResult<f64> {
        // Syslog doesn't provide year Oo, we append it....
        let data = format!("{} {}", Local::now().year(), &headers[0..15]);
        Ok(NaiveDateTime::parse_from_str(&data, "%Y %b %d %H:%M:%S")?.timestamp() as f64)
    }
    pub fn extract_hostname_from_headers(headers: &str) -> CefResult<String> {
        let re = regex::Regex::new(r".*\s+(?P<host>.*)\sCEF:\d+").unwrap();
        Ok(re.captures(headers)?.name("host")?.as_str().to_string())
    }
}