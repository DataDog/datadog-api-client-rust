// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The response containing attributes for custom reports.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageCustomReportsAttributes {
    /// The date the specified custom report was computed.
    #[serde(rename = "computed_on")]
    pub computed_on: Option<String>,
    /// The ending date of custom report.
    #[serde(rename = "end_date")]
    pub end_date: Option<String>,
    /// size
    #[serde(rename = "size")]
    pub size: Option<i64>,
    /// The starting date of custom report.
    #[serde(rename = "start_date")]
    pub start_date: Option<String>,
    /// A list of tags to apply to custom reports.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageCustomReportsAttributes {
    pub fn new() -> UsageCustomReportsAttributes {
        UsageCustomReportsAttributes {
            computed_on: None,
            end_date: None,
            size: None,
            start_date: None,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn computed_on(mut self, value: String) -> Self {
        self.computed_on = Some(value);
        self
    }

    pub fn end_date(mut self, value: String) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn size(mut self, value: i64) -> Self {
        self.size = Some(value);
        self
    }

    pub fn start_date(mut self, value: String) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
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

impl Default for UsageCustomReportsAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageCustomReportsAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageCustomReportsAttributesVisitor;
        impl<'a> Visitor<'a> for UsageCustomReportsAttributesVisitor {
            type Value = UsageCustomReportsAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut computed_on: Option<String> = None;
                let mut end_date: Option<String> = None;
                let mut size: Option<i64> = None;
                let mut start_date: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "computed_on" => {
                            if v.is_null() {
                                continue;
                            }
                            computed_on =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "end_date" => {
                            if v.is_null() {
                                continue;
                            }
                            end_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "size" => {
                            if v.is_null() {
                                continue;
                            }
                            size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "start_date" => {
                            if v.is_null() {
                                continue;
                            }
                            start_date = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = UsageCustomReportsAttributes {
                    computed_on,
                    end_date,
                    size,
                    start_date,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageCustomReportsAttributesVisitor)
    }
}
