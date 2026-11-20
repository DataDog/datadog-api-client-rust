// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FixedSpan2 {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "from")]
    pub from: f64,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "to")]
    pub to: f64,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl FixedSpan2 {
    pub fn new(from: f64, to: f64) -> FixedSpan2 {
        FixedSpan2 {
            from,
            to,
            _unparsed: false,
        }
    }
    
}



impl<'de> Deserialize<'de> for FixedSpan2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FixedSpan2Visitor;
        impl<'a> Visitor<'a> for FixedSpan2Visitor {
            type Value = FixedSpan2;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut from: Option<f64> = None;
                let mut to: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "from" => {
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "to" => {
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let from = from.ok_or_else(|| M::Error::missing_field("from"))?;
                let to = to.ok_or_else(|| M::Error::missing_field("to"))?;

                let content = FixedSpan2 {
                    from,
                    to,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FixedSpan2Visitor)
    }
}
