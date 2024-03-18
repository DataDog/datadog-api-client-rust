// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The object containing the aggregates.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageAttributionAggregatesBody {
    /// The aggregate type.
    #[serde(rename = "agg_type")]
    pub agg_type: Option<String>,
    /// The field.
    #[serde(rename = "field")]
    pub field: Option<String>,
    /// The value for a given field.
    #[serde(rename = "value")]
    pub value: Option<f64>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageAttributionAggregatesBody {
    pub fn new() -> UsageAttributionAggregatesBody {
        UsageAttributionAggregatesBody {
            agg_type: None,
            field: None,
            value: None,
            _unparsed: false,
        }
    }

    pub fn agg_type(mut self, value: String) -> Self {
        self.agg_type = Some(value);
        self
    }

    pub fn field(mut self, value: String) -> Self {
        self.field = Some(value);
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }
}

impl Default for UsageAttributionAggregatesBody {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageAttributionAggregatesBody {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageAttributionAggregatesBodyVisitor;
        impl<'a> Visitor<'a> for UsageAttributionAggregatesBodyVisitor {
            type Value = UsageAttributionAggregatesBody;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut agg_type: Option<String> = None;
                let mut field: Option<String> = None;
                let mut value: Option<f64> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "agg_type" => {
                            if v.is_null() {
                                continue;
                            }
                            agg_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "field" => {
                            if v.is_null() {
                                continue;
                            }
                            field = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            if v.is_null() {
                                continue;
                            }
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UsageAttributionAggregatesBody {
                    agg_type,
                    field,
                    value,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageAttributionAggregatesBodyVisitor)
    }
}
