// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The counts of findings without a team tag by ownership confidence.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OwnershipUntaggedFindingsAttributes {
    /// The number of high confidence findings without a team tag.
    #[serde(rename = "high_confidence")]
    pub high_confidence: i64,
    /// The number of low confidence findings without a team tag.
    #[serde(rename = "low_confidence")]
    pub low_confidence: i64,
    /// The number of medium confidence findings without a team tag.
    #[serde(rename = "medium_confidence")]
    pub medium_confidence: i64,
    /// The total number of findings without a team tag.
    #[serde(rename = "total")]
    pub total: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OwnershipUntaggedFindingsAttributes {
    pub fn new(
        high_confidence: i64,
        low_confidence: i64,
        medium_confidence: i64,
        total: i64,
    ) -> OwnershipUntaggedFindingsAttributes {
        OwnershipUntaggedFindingsAttributes {
            high_confidence,
            low_confidence,
            medium_confidence,
            total,
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

impl<'de> Deserialize<'de> for OwnershipUntaggedFindingsAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OwnershipUntaggedFindingsAttributesVisitor;
        impl<'a> Visitor<'a> for OwnershipUntaggedFindingsAttributesVisitor {
            type Value = OwnershipUntaggedFindingsAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut high_confidence: Option<i64> = None;
                let mut low_confidence: Option<i64> = None;
                let mut medium_confidence: Option<i64> = None;
                let mut total: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "high_confidence" => {
                            high_confidence =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "low_confidence" => {
                            low_confidence =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "medium_confidence" => {
                            medium_confidence =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total" => {
                            total = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let high_confidence =
                    high_confidence.ok_or_else(|| M::Error::missing_field("high_confidence"))?;
                let low_confidence =
                    low_confidence.ok_or_else(|| M::Error::missing_field("low_confidence"))?;
                let medium_confidence = medium_confidence
                    .ok_or_else(|| M::Error::missing_field("medium_confidence"))?;
                let total = total.ok_or_else(|| M::Error::missing_field("total"))?;

                let content = OwnershipUntaggedFindingsAttributes {
                    high_confidence,
                    low_confidence,
                    medium_confidence,
                    total,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OwnershipUntaggedFindingsAttributesVisitor)
    }
}
