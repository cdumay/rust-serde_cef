use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{Datelike, DateTime, FixedOffset, Local, NaiveDateTime, Timelike};
use serde_value::Value;

use crate::result::{CefError, CefResult};

pub fn try_detect_type(raw: &str) -> Value {
    if let Ok(data) = raw.parse::<bool>() {
        return Value::Bool(data);
    }
    if let Ok(data) = raw.parse::<u64>() {
        return Value::U64(data);
    }
    if let Ok(data) = raw.parse::<i64>() {
        return Value::I64(data);
    }
    if let Ok(data) = raw.parse::<f64>() {
        return Value::F64(data);
    }
    Value::String(raw.to_string())
}

fn rfc3339_to_unix(rfc3339: &str) -> CefResult<f64> {
    match DateTime::parse_from_rfc3339(rfc3339) {
        Ok(date) => Ok(PreciseTimestamp::from_datetime(date).as_f64()),
        Err(err) => Err(CefError::from(err))
    }
}

fn english_time_to_unix(et: &str) -> CefResult<f64> {
    match DateTime::parse_from_str(et, "%e/%b/%Y %H:%M:%S%.f %z") {
        Ok(date) => Ok(PreciseTimestamp::from_datetime(date).as_f64()),
        Err(err) => Err(CefError::from(err))
    }
}

fn unix_strtime_to_unix(et: &str) -> CefResult<f64> {
    match et.parse::<f64>() {
        Ok(ts) => Ok(ts),
        Err(err) => Err(CefError::from(err))
    }
}

fn native_to_unix(et: &str) -> CefResult<f64> {
    match NaiveDateTime::parse_from_str(&et, "%Y %b %d %H:%M:%S") {
        Ok(dt) => Ok(dt.timestamp() as f64),
        Err(err) => Err(CefError::from(err))
    }
}

/// Try to deserialize date from string.
///
/// Allowed formats are:
/// * Native `%b %d %H:%M:%S` (E.g: `Dec 19 01:07:56`)
/// * Timestamp (E.g: `1561554774.901402`)
/// * English format `%e/%b/%Y %H:%M:%S%.f %z` (E.g: `26/Jun/2019 15:21:55.152120022 +0200`)
pub fn parse_ts(line: &str) -> CefResult<f64> {
    unix_strtime_to_unix(line)
        .or_else(|_| native_to_unix(line))
        .or_else(|_| rfc3339_to_unix(line))
        .or_else(|_| english_time_to_unix(line))
}

pub struct PreciseTimestamp {
    ts: f64,
}

impl PreciseTimestamp {
    #[inline]
    pub fn now() -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        PreciseTimestamp {
            ts: now.as_secs() as f64 + f64::from(now.subsec_nanos()) / 1e9,
        }
    }

    #[inline]
    pub fn from_datetime(tsd: DateTime<FixedOffset>) -> Self {
        PreciseTimestamp {
            ts: tsd.timestamp() as f64 + f64::from(tsd.naive_utc().nanosecond()) / 1e9,
        }
    }

    #[inline]
    pub fn as_f64(&self) -> f64 {
        self.ts
    }
}

pub fn now() -> f64 {
    PreciseTimestamp::now().as_f64()
}

/// Extract timestamp from Syslog headers.
pub fn extract_ts_from_headers(headers: &str) -> CefResult<f64> {
    // Syslog doesn't provide year Oo, we append it....
    let data = format!("{} {}", Local::now().year(), &headers[0..15]);
    parse_ts(&data)
}

/// Extract hostname from Syslog headers.
pub fn extract_hostname_from_headers(headers: &str) -> CefResult<String> {
    let re = regex::Regex::new(r".*\s+(?P<host>.*)\sCEF:\d+").unwrap();
    Ok(re.captures(headers)?.name("host")?.as_str().to_string())
}
