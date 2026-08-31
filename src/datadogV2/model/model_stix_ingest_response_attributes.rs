// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Counters describing the result of the STIX ingestion request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct STIXIngestResponseAttributes {
    /// The number of supported indicators accepted.
    #[serde(rename = "accepted")]
    pub accepted: i64,
    /// The number of indicators with patterns that could not be parsed.
    #[serde(rename = "invalid")]
    pub invalid: i64,
    /// The number of indicator objects with an unsupported STIX version or a pattern that produced no supported observable values.
    #[serde(rename = "unsupported")]
    pub unsupported: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl STIXIngestResponseAttributes {
    pub fn new(accepted: i64, invalid: i64, unsupported: i64) -> STIXIngestResponseAttributes {
        STIXIngestResponseAttributes {
            accepted,
            invalid,
            unsupported,
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

impl<'de> Deserialize<'de> for STIXIngestResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct STIXIngestResponseAttributesVisitor;
        impl<'a> Visitor<'a> for STIXIngestResponseAttributesVisitor {
            type Value = STIXIngestResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut accepted: Option<i64> = None;
                let mut invalid: Option<i64> = None;
                let mut unsupported: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "accepted" => {
                            accepted = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "invalid" => {
                            invalid = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "unsupported" => {
                            unsupported =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let accepted = accepted.ok_or_else(|| M::Error::missing_field("accepted"))?;
                let invalid = invalid.ok_or_else(|| M::Error::missing_field("invalid"))?;
                let unsupported =
                    unsupported.ok_or_else(|| M::Error::missing_field("unsupported"))?;

                let content = STIXIngestResponseAttributes {
                    accepted,
                    invalid,
                    unsupported,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(STIXIngestResponseAttributesVisitor)
    }
}
