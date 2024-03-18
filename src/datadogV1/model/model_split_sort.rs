// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Controls the order in which graphs appear in the split.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SplitSort {
    /// Defines the metric and aggregation used as the sort value.
    #[serde(rename = "compute")]
    pub compute: Option<crate::datadogV1::model::SplitConfigSortCompute>,
    /// Widget sorting methods.
    #[serde(rename = "order")]
    pub order: crate::datadogV1::model::WidgetSort,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SplitSort {
    pub fn new(order: crate::datadogV1::model::WidgetSort) -> SplitSort {
        SplitSort {
            compute: None,
            order,
            _unparsed: false,
        }
    }

    pub fn compute(mut self, value: crate::datadogV1::model::SplitConfigSortCompute) -> Self {
        self.compute = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for SplitSort {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SplitSortVisitor;
        impl<'a> Visitor<'a> for SplitSortVisitor {
            type Value = SplitSort;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<crate::datadogV1::model::SplitConfigSortCompute> = None;
                let mut order: Option<crate::datadogV1::model::WidgetSort> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compute" => {
                            if v.is_null() {
                                continue;
                            }
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let order = order.ok_or_else(|| M::Error::missing_field("order"))?;

                let content = SplitSort {
                    compute,
                    order,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SplitSortVisitor)
    }
}
