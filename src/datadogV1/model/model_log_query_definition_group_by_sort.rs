// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Define a sorting method.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct LogQueryDefinitionGroupBySort {
    /// The aggregation method.
    #[serde(rename = "aggregation")]
    pub aggregation: String,
    /// Facet name.
    #[serde(rename = "facet")]
    pub facet: Option<String>,
    /// Widget sorting methods.
    #[serde(rename = "order")]
    pub order: crate::datadogV1::model::WidgetSort,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl LogQueryDefinitionGroupBySort {
    pub fn new(
        aggregation: String,
        order: crate::datadogV1::model::WidgetSort,
    ) -> LogQueryDefinitionGroupBySort {
        LogQueryDefinitionGroupBySort {
            aggregation,
            facet: None,
            order,
            _unparsed: false,
        }
    }

    pub fn facet(mut self, value: String) -> Self {
        self.facet = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for LogQueryDefinitionGroupBySort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LogQueryDefinitionGroupBySortVisitor;
        impl<'a> Visitor<'a> for LogQueryDefinitionGroupBySortVisitor {
            type Value = LogQueryDefinitionGroupBySort;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut aggregation: Option<String> = None;
                let mut facet: Option<String> = None;
                let mut order: Option<crate::datadogV1::model::WidgetSort> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "aggregation" => {
                            aggregation =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "facet" => {
                            if v.is_null() {
                                continue;
                            }
                            facet = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "order" => {
                            order = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _order) = order {
                                match _order {
                                    crate::datadogV1::model::WidgetSort::UnparsedObject(_order) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }
                let aggregation =
                    aggregation.ok_or_else(|| M::Error::missing_field("aggregation"))?;
                let order = order.ok_or_else(|| M::Error::missing_field("order"))?;

                let content = LogQueryDefinitionGroupBySort {
                    aggregation,
                    facet,
                    order,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(LogQueryDefinitionGroupBySortVisitor)
    }
}
