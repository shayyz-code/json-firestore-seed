// Copyright (c) 2025 RustRaccoon Software Company Ltd.

use chrono::{DateTime, Utc};
use firestore::FirestoreTimestamp;
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

// An internal representation that can serialize into JSON-like structure
// but also contains real FirestoreTimestamp variants.
#[derive(Debug, Clone, Deserialize)]
pub enum FirestoreValue {
    Null,
    Bool(bool),
    Number(serde_json::Number),
    String(String),
    Timestamp(FirestoreTimestamp),
    Array(Vec<FirestoreValue>),
    Object(BTreeMap<String, FirestoreValue>),
}

impl Serialize for FirestoreValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use FirestoreValue::*;
        match self {
            Null => serializer.serialize_none(),
            Bool(b) => serializer.serialize_bool(*b),
            Number(n) => {
                // serde_json::Number -> serialize as i64/ u64/ f64 accordingly
                if let Some(i) = n.as_i64() {
                    serializer.serialize_i64(i)
                } else if let Some(u) = n.as_u64() {
                    serializer.serialize_u64(u)
                } else if let Some(f) = n.as_f64() {
                    serializer.serialize_f64(f)
                } else {
                    // fallback to string
                    serializer.serialize_str(&n.to_string())
                }
            }
            String(s) => serializer.serialize_str(s),
            Timestamp(ts) => {
                // FirestoreTimestamp already implements Serialize — delegate to it.
                ts.serialize(serializer)
            }
            Array(vec) => {
                let mut seq = serializer.serialize_seq(Some(vec.len()))?;
                for v in vec {
                    seq.serialize_element(v)?;
                }
                seq.end()
            }
            Object(map) => {
                let mut m = serializer.serialize_map(Some(map.len()))?;
                for (k, v) in map {
                    m.serialize_entry(k, v)?;
                }
                m.end()
            }
        }
    }
}

// Convert serde_json::Value -> FirestoreValue, handling timestamp markers
pub fn json_to_firestore_value(v: &Value) -> Result<FirestoreValue, anyhow::Error> {
    use FirestoreValue::*;
    match v {
        Value::Null => Ok(Null),
        Value::Bool(b) => Ok(Bool(*b)),
        Value::Number(n) => Ok(Number(n.clone())),
        Value::String(s) => {
            // allow a special marker string for "now"
            if s == "__fire_ts_now__" {
                Ok(Timestamp(FirestoreTimestamp(Utc::now())))
            } else {
                Ok(String(s.clone()))
            }
        }
        Value::Array(arr) => {
            let mut out = Vec::with_capacity(arr.len());
            for el in arr {
                out.push(json_to_firestore_value(el)?);
            }
            Ok(Array(out))
        }
        Value::Object(map) => {
            // Check if this object is a timestamp marker like:
            // { "__fire_ts_from_date__": "2023-..." }
            if map.len() == 1
                && let Some(Value::String(s)) = map.get("__fire_ts_from_date__")
            {
                // parse RFC3339 (and common variants)
                // try chrono parse
                let dt = DateTime::parse_from_rfc3339(s)
                    .map(|dt| dt.with_timezone(&Utc))
                    .or_else(|_| {
                        // try parse with chrono naive fallback (YYYY-mm-dd HH:MM:SS)
                        chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
                            .map(|ndt| DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc))
                    })
                    .map_err(|e| {
                        anyhow::anyhow!("failed to parse timestamp string `{}`: {}", s, e)
                    })?;
                return Ok(Timestamp(FirestoreTimestamp(dt)));
            }

            // otherwise normal object -> recursively convert
            let mut out = BTreeMap::new();
            for (k, v2) in map {
                out.insert(k.clone(), json_to_firestore_value(v2)?);
            }
            Ok(Object(out))
        }
    }
}
