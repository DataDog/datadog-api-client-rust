// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object describing the logs filter.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogsCategoryProcessorCategory {
    /// Filter for logs.
    #[serde(rename = "filter")]
    pub filter: Option<crate::datadogV1::model::LogsFilter>,
    /// Value to assign to the target attribute.
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogsCategoryProcessorCategory {
    pub fn new() -> LogsCategoryProcessorCategory {
        LogsCategoryProcessorCategory {
            filter: None,
            name: None,
            _unparsed: false,
        }
    }

    pub fn filter(&mut self, value: crate::datadogV1::model::LogsFilter) -> &mut Self {
        self.filter = Some(value);
        self
    }

    pub fn name(&mut self, value: String) -> &mut Self {
        self.name = Some(value);
        self
    }
}

impl Default for LogsCategoryProcessorCategory {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for LogsCategoryProcessorCategory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogsCategoryProcessorCategoryVisitor;
        impl<'a> Visitor<'a> for LogsCategoryProcessorCategoryVisitor {
            type Value = LogsCategoryProcessorCategory;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut filter: Option<crate::datadogV1::model::LogsFilter> = None;
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
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = LogsCategoryProcessorCategory {
                    filter,
                    name,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogsCategoryProcessorCategoryVisitor)
    }
}
