// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The chart link to remove. It must match the stored link exactly:
/// `chart_id` alone for a saved chart, or `chart_id` and `dashboard_id` for a dashboard widget.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsMonitorUnlinkAttributes {
    /// ID of the chart the monitor is linked to.
    #[serde(rename = "chart_id")]
    pub chart_id: String,
    /// ID of the dashboard the monitor is linked to.
    #[serde(rename = "dashboard_id")]
    pub dashboard_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsMonitorUnlinkAttributes {
    pub fn new(chart_id: String) -> ProductAnalyticsMonitorUnlinkAttributes {
        ProductAnalyticsMonitorUnlinkAttributes {
            chart_id,
            dashboard_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn dashboard_id(mut self, value: String) -> Self {
        self.dashboard_id = Some(value);
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

impl<'de> Deserialize<'de> for ProductAnalyticsMonitorUnlinkAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsMonitorUnlinkAttributesVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsMonitorUnlinkAttributesVisitor {
            type Value = ProductAnalyticsMonitorUnlinkAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut chart_id: Option<String> = None;
                let mut dashboard_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "chart_id" => {
                            chart_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dashboard_id" => {
                            if v.is_null() {
                                continue;
                            }
                            dashboard_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let chart_id = chart_id.ok_or_else(|| M::Error::missing_field("chart_id"))?;

                let content = ProductAnalyticsMonitorUnlinkAttributes {
                    chart_id,
                    dashboard_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsMonitorUnlinkAttributesVisitor)
    }
}
