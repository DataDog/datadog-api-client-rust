// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A filter applied to a step, or a range of steps, of the journey graph.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsJourneySearchGraphFilter {
    /// The journey-level metric the graph filter applies to.
    #[serde(rename = "name")]
    pub name: crate::datadogV2::model::ProductAnalyticsJourneySearchGraphFilterName,
    /// Comparison operator applied to the graph filter value.
    #[serde(rename = "operator")]
    pub operator: crate::datadogV2::model::ProductAnalyticsJourneySearchGraphFilterOperator,
    /// A reference to a step, or a range of steps, in the journey.
    /// Use a `node` target to name a single step, or a `path` target to name the range
    /// between two steps.
    #[serde(rename = "target")]
    pub target: Option<crate::datadogV2::model::ProductAnalyticsJourneyTarget>,
    /// Value compared against the metric. Durations are expressed in milliseconds.
    #[serde(rename = "value")]
    pub value: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsJourneySearchGraphFilter {
    pub fn new(
        name: crate::datadogV2::model::ProductAnalyticsJourneySearchGraphFilterName,
        operator: crate::datadogV2::model::ProductAnalyticsJourneySearchGraphFilterOperator,
        value: i64,
    ) -> ProductAnalyticsJourneySearchGraphFilter {
        ProductAnalyticsJourneySearchGraphFilter {
            name,
            operator,
            target: None,
            value,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn target(mut self, value: crate::datadogV2::model::ProductAnalyticsJourneyTarget) -> Self {
        self.target = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsJourneySearchGraphFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsJourneySearchGraphFilterVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsJourneySearchGraphFilterVisitor {
            type Value = ProductAnalyticsJourneySearchGraphFilter;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<
                    crate::datadogV2::model::ProductAnalyticsJourneySearchGraphFilterName,
                > = None;
                let mut operator: Option<
                    crate::datadogV2::model::ProductAnalyticsJourneySearchGraphFilterOperator,
                > = None;
                let mut target: Option<crate::datadogV2::model::ProductAnalyticsJourneyTarget> =
                    None;
                let mut value: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _name) = name {
                                match _name {
                                    crate::datadogV2::model::ProductAnalyticsJourneySearchGraphFilterName::UnparsedObject(_name) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "operator" => {
                            operator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _operator) = operator {
                                match _operator {
                                    crate::datadogV2::model::ProductAnalyticsJourneySearchGraphFilterOperator::UnparsedObject(_operator) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "target" => {
                            if v.is_null() {
                                continue;
                            }
                            target = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _target) = target {
                                match _target {
                                    crate::datadogV2::model::ProductAnalyticsJourneyTarget::UnparsedObject(_target) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let operator = operator.ok_or_else(|| M::Error::missing_field("operator"))?;
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = ProductAnalyticsJourneySearchGraphFilter {
                    name,
                    operator,
                    target,
                    value,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsJourneySearchGraphFilterVisitor)
    }
}
