// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing IDs of Synthetic tests and a timeframe
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsFetchUptimesPayload {
    /// Timestamp in seconds (Unix epoch) for the start of uptime.
    #[serde(rename = "from_ts")]
    pub from_ts: i64,
    /// An array of Synthetic test IDs you want to delete.
    #[serde(rename = "public_ids")]
    pub public_ids: Vec<String>,
    /// Timestamp in seconds (Unix epoch) for the end of uptime.
    #[serde(rename = "to_ts")]
    pub to_ts: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsFetchUptimesPayload {
    pub fn new(from_ts: i64, public_ids: Vec<String>, to_ts: i64) -> SyntheticsFetchUptimesPayload {
        SyntheticsFetchUptimesPayload {
            from_ts,
            public_ids,
            to_ts,
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

impl<'de> Deserialize<'de> for SyntheticsFetchUptimesPayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsFetchUptimesPayloadVisitor;
        impl<'a> Visitor<'a> for SyntheticsFetchUptimesPayloadVisitor {
            type Value = SyntheticsFetchUptimesPayload;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut from_ts: Option<i64> = None;
                let mut public_ids: Option<Vec<String>> = None;
                let mut to_ts: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "from_ts" => {
                            from_ts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_ids" => {
                            public_ids = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to_ts" => {
                            to_ts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let from_ts = from_ts.ok_or_else(|| M::Error::missing_field("from_ts"))?;
                let public_ids = public_ids.ok_or_else(|| M::Error::missing_field("public_ids"))?;
                let to_ts = to_ts.ok_or_else(|| M::Error::missing_field("to_ts"))?;

                let content = SyntheticsFetchUptimesPayload {
                    from_ts,
                    public_ids,
                    to_ts,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsFetchUptimesPayloadVisitor)
    }
}
