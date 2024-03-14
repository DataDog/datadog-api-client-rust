// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Represents the index exclusion filter object from configuration API.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsExclusion {
    /// Exclusion filter is defined by a query, a sampling rule, and a active/inactive toggle.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV1::model::LogsExclusionFilter>,
    /// Whether or not the exclusion filter is active.
    #[serde(rename = "is_enabled")]
    pub is_enabled: Option<bool>,
    /// Name of the index exclusion filter.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsExclusion {
    pub fn new(name: String) -> LogsExclusion {
        LogsExclusion {
            filter: None,
            is_enabled: None,
            name,
            _unparsed: false,
        }
    }

    pub fn filter(mut self, value: crate::datadogV1::model::LogsExclusionFilter) -> Self {
        self.filter = Some(value);
        self
    }

    pub fn is_enabled(mut self, value: bool) -> Self {
        self.is_enabled = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for LogsExclusion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsExclusionVisitor;
        impl<'a> Visitor<'a> for LogsExclusionVisitor {
            type Value = LogsExclusion;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filter: Option<crate::datadogV1::model::LogsExclusionFilter> = None;
                let mut is_enabled: Option<bool> = None;
                let mut name: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "filter" => {
                            if v.is_null() {
                                continue;
                            }
                            filter = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            is_enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = LogsExclusion {
                    filter,
                    is_enabled,
                    name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsExclusionVisitor)
    }
}
