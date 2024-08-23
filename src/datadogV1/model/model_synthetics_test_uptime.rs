// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the uptime for a Synthetic test ID.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestUptime {
    /// Timestamp in seconds for the start of uptime.
    #[serde(rename = "from_ts")]
    pub from_ts: Option<i64>,
    /// Object containing the uptime information.
    #[serde(rename = "overall")]
    pub overall: Option<crate::datadogV1::model::SyntheticsUptime>,
    /// A Synthetic test ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    /// Timestamp in seconds for the end of uptime.
    #[serde(rename = "to_ts")]
    pub to_ts: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestUptime {
    pub fn new() -> SyntheticsTestUptime {
        SyntheticsTestUptime {
            from_ts: None,
            overall: None,
            public_id: None,
            to_ts: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn from_ts(mut self, value: i64) -> Self {
        self.from_ts = Some(value);
        self
    }

    pub fn overall(mut self, value: crate::datadogV1::model::SyntheticsUptime) -> Self {
        self.overall = Some(value);
        self
    }

    pub fn public_id(mut self, value: String) -> Self {
        self.public_id = Some(value);
        self
    }

    pub fn to_ts(mut self, value: i64) -> Self {
        self.to_ts = Some(value);
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

impl Default for SyntheticsTestUptime {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestUptime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestUptimeVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestUptimeVisitor {
            type Value = SyntheticsTestUptime;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut from_ts: Option<i64> = None;
                let mut overall: Option<crate::datadogV1::model::SyntheticsUptime> = None;
                let mut public_id: Option<String> = None;
                let mut to_ts: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "from_ts" => {
                            if v.is_null() {
                                continue;
                            }
                            from_ts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "overall" => {
                            if v.is_null() {
                                continue;
                            }
                            overall = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to_ts" => {
                            if v.is_null() {
                                continue;
                            }
                            to_ts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestUptime {
                    from_ts,
                    overall,
                    public_id,
                    to_ts,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestUptimeVisitor)
    }
}
