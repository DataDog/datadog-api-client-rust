// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A response containing the number of analyzed logs for each hour for a given organization.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageAnalyzedLogsResponse {
    /// Get hourly usage for analyzed logs.
    #[serde(rename = "usage")]
    pub usage: Option<Vec<crate::datadogV1::model::UsageAnalyzedLogsHour>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageAnalyzedLogsResponse {
    pub fn new() -> UsageAnalyzedLogsResponse {
        UsageAnalyzedLogsResponse {
            usage: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn usage(mut self, value: Vec<crate::datadogV1::model::UsageAnalyzedLogsHour>) -> Self {
        self.usage = Some(value);
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

impl Default for UsageAnalyzedLogsResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageAnalyzedLogsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageAnalyzedLogsResponseVisitor;
        impl<'a> Visitor<'a> for UsageAnalyzedLogsResponseVisitor {
            type Value = UsageAnalyzedLogsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut usage: Option<Vec<crate::datadogV1::model::UsageAnalyzedLogsHour>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "usage" => {
                            if v.is_null() {
                                continue;
                            }
                            usage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = UsageAnalyzedLogsResponse {
                    usage,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageAnalyzedLogsResponseVisitor)
    }
}
