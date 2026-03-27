// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Pagination metadata for a version history response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SyntheticsTestVersionHistoryMeta {
    /// The version number to use as the `last_version_number` query parameter
    /// to fetch the next page. `null` indicates there are no more pages.
    #[serde(
        rename = "next_last_version_number",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub next_last_version_number: Option<Option<i64>>,
    /// The number of days that version history is retained.
    #[serde(rename = "retention_period_in_days")]
    pub retention_period_in_days: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SyntheticsTestVersionHistoryMeta {
    pub fn new() -> SyntheticsTestVersionHistoryMeta {
        SyntheticsTestVersionHistoryMeta {
            next_last_version_number: None,
            retention_period_in_days: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn next_last_version_number(mut self, value: Option<i64>) -> Self {
        self.next_last_version_number = Some(value);
        self
    }

    pub fn retention_period_in_days(mut self, value: i64) -> Self {
        self.retention_period_in_days = Some(value);
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

impl Default for SyntheticsTestVersionHistoryMeta {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SyntheticsTestVersionHistoryMeta {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SyntheticsTestVersionHistoryMetaVisitor;
        impl<'a> Visitor<'a> for SyntheticsTestVersionHistoryMetaVisitor {
            type Value = SyntheticsTestVersionHistoryMeta;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut next_last_version_number: Option<Option<i64>> = None;
                let mut retention_period_in_days: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "next_last_version_number" => {
                            next_last_version_number =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "retention_period_in_days" => {
                            if v.is_null() {
                                continue;
                            }
                            retention_period_in_days =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SyntheticsTestVersionHistoryMeta {
                    next_last_version_number,
                    retention_period_in_days,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SyntheticsTestVersionHistoryMetaVisitor)
    }
}
