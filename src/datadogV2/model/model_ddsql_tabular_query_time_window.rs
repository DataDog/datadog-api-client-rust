// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Time window scoping the underlying data sources, expressed in Unix milliseconds
/// since the epoch. Inclusive on `from_timestamp`, exclusive on `to_timestamp`.
/// Results from static tables (for example, `dd.hosts`) are not affected by the
/// time window, but the field must still be provided.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DdsqlTabularQueryTimeWindow {
    /// Start of the query window (inclusive), in Unix milliseconds since the epoch.
    #[serde(rename = "from_timestamp")]
    pub from_timestamp: i64,
    /// End of the query window (exclusive), in Unix milliseconds since the epoch.
    #[serde(rename = "to_timestamp")]
    pub to_timestamp: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DdsqlTabularQueryTimeWindow {
    pub fn new(from_timestamp: i64, to_timestamp: i64) -> DdsqlTabularQueryTimeWindow {
        DdsqlTabularQueryTimeWindow {
            from_timestamp,
            to_timestamp,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for DdsqlTabularQueryTimeWindow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DdsqlTabularQueryTimeWindowVisitor;
        impl<'a> Visitor<'a> for DdsqlTabularQueryTimeWindowVisitor {
            type Value = DdsqlTabularQueryTimeWindow;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut from_timestamp: Option<i64> = None;
                let mut to_timestamp: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "from_timestamp" => {
                            from_timestamp =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to_timestamp" => {
                            to_timestamp =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let from_timestamp =
                    from_timestamp.ok_or_else(|| M::Error::missing_field("from_timestamp"))?;
                let to_timestamp =
                    to_timestamp.ok_or_else(|| M::Error::missing_field("to_timestamp"))?;

                let content = DdsqlTabularQueryTimeWindow {
                    from_timestamp,
                    to_timestamp,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DdsqlTabularQueryTimeWindowVisitor)
    }
}
