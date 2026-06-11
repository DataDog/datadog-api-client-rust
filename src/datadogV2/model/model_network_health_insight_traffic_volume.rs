// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Network traffic volume metrics between the client and server services during the query window.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NetworkHealthInsightTrafficVolume {
    /// Total bytes read from the server to the client during the query window.
    #[serde(rename = "bytes_read")]
    pub bytes_read: Option<i64>,
    /// Total bytes written from the client to the server during the query window.
    #[serde(rename = "bytes_written")]
    pub bytes_written: Option<i64>,
    /// Sum of bytes written and bytes read across the query window.
    #[serde(rename = "total_traffic")]
    pub total_traffic: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NetworkHealthInsightTrafficVolume {
    pub fn new() -> NetworkHealthInsightTrafficVolume {
        NetworkHealthInsightTrafficVolume {
            bytes_read: None,
            bytes_written: None,
            total_traffic: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn bytes_read(mut self, value: i64) -> Self {
        self.bytes_read = Some(value);
        self
    }

    pub fn bytes_written(mut self, value: i64) -> Self {
        self.bytes_written = Some(value);
        self
    }

    pub fn total_traffic(mut self, value: i64) -> Self {
        self.total_traffic = Some(value);
        self
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl Default for NetworkHealthInsightTrafficVolume {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for NetworkHealthInsightTrafficVolume {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NetworkHealthInsightTrafficVolumeVisitor;
        impl<'a> Visitor<'a> for NetworkHealthInsightTrafficVolumeVisitor {
            type Value = NetworkHealthInsightTrafficVolume;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut bytes_read: Option<i64> = None;
                let mut bytes_written: Option<i64> = None;
                let mut total_traffic: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "bytes_read" => {
                            if v.is_null() {
                                continue;
                            }
                            bytes_read = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bytes_written" => {
                            if v.is_null() {
                                continue;
                            }
                            bytes_written =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total_traffic" => {
                            if v.is_null() {
                                continue;
                            }
                            total_traffic =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = NetworkHealthInsightTrafficVolume {
                    bytes_read,
                    bytes_written,
                    total_traffic,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NetworkHealthInsightTrafficVolumeVisitor)
    }
}
