// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Filter definition for aggregate filtered queries.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorFormulaAndFunctionAggregateQueryFilter {
    /// Attribute from the base query to filter on.
    #[serde(rename = "base_attribute")]
    pub base_attribute: String,
    /// Whether to exclude matching records instead of including them.
    #[serde(rename = "exclude")]
    pub exclude: Option<bool>,
    /// Attribute from the filter query to match against.
    #[serde(rename = "filter_attribute")]
    pub filter_attribute: String,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorFormulaAndFunctionAggregateQueryFilter {
    pub fn new(
        base_attribute: String,
        filter_attribute: String,
    ) -> MonitorFormulaAndFunctionAggregateQueryFilter {
        MonitorFormulaAndFunctionAggregateQueryFilter {
            base_attribute,
            exclude: None,
            filter_attribute,
            _unparsed: false,
        }
    }

    pub fn exclude(mut self, value: bool) -> Self {
        self.exclude = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for MonitorFormulaAndFunctionAggregateQueryFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorFormulaAndFunctionAggregateQueryFilterVisitor;
        impl<'a> Visitor<'a> for MonitorFormulaAndFunctionAggregateQueryFilterVisitor {
            type Value = MonitorFormulaAndFunctionAggregateQueryFilter;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut base_attribute: Option<String> = None;
                let mut exclude: Option<bool> = None;
                let mut filter_attribute: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "base_attribute" => {
                            base_attribute =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exclude" => {
                            if v.is_null() {
                                continue;
                            }
                            exclude = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filter_attribute" => {
                            filter_attribute =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let base_attribute =
                    base_attribute.ok_or_else(|| M::Error::missing_field("base_attribute"))?;
                let filter_attribute =
                    filter_attribute.ok_or_else(|| M::Error::missing_field("filter_attribute"))?;

                let content = MonitorFormulaAndFunctionAggregateQueryFilter {
                    base_attribute,
                    exclude,
                    filter_attribute,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorFormulaAndFunctionAggregateQueryFilterVisitor)
    }
}
