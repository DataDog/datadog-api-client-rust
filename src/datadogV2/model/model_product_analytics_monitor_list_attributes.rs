// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A list of Product Analytics monitors.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsMonitorListAttributes {
    /// The monitors.
    #[serde(rename = "monitors")]
    pub monitors: Vec<crate::datadogV2::model::ProductAnalyticsMonitorListItem>,
    /// Total number of monitors that match, across all pages.
    #[serde(rename = "total")]
    pub total: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsMonitorListAttributes {
    pub fn new(
        monitors: Vec<crate::datadogV2::model::ProductAnalyticsMonitorListItem>,
        total: i64,
    ) -> ProductAnalyticsMonitorListAttributes {
        ProductAnalyticsMonitorListAttributes {
            monitors,
            total,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsMonitorListAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsMonitorListAttributesVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsMonitorListAttributesVisitor {
            type Value = ProductAnalyticsMonitorListAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut monitors: Option<
                    Vec<crate::datadogV2::model::ProductAnalyticsMonitorListItem>,
                > = None;
                let mut total: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "monitors" => {
                            monitors = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total" => {
                            total = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let monitors = monitors.ok_or_else(|| M::Error::missing_field("monitors"))?;
                let total = total.ok_or_else(|| M::Error::missing_field("total"))?;

                let content = ProductAnalyticsMonitorListAttributes {
                    monitors,
                    total,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsMonitorListAttributesVisitor)
    }
}
