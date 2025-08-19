// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Product Scales configuration for the RUM application.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RUMProductScales {
    /// Product Analytics retention scale configuration.
    #[serde(rename = "product_analytics_retention_scale")]
    pub product_analytics_retention_scale:
        Option<crate::datadogV2::model::RUMProductAnalyticsRetentionScale>,
    /// RUM event processing scale configuration.
    #[serde(rename = "rum_event_processing_scale")]
    pub rum_event_processing_scale: Option<crate::datadogV2::model::RUMEventProcessingScale>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RUMProductScales {
    pub fn new() -> RUMProductScales {
        RUMProductScales {
            product_analytics_retention_scale: None,
            rum_event_processing_scale: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn product_analytics_retention_scale(
        mut self,
        value: crate::datadogV2::model::RUMProductAnalyticsRetentionScale,
    ) -> Self {
        self.product_analytics_retention_scale = Some(value);
        self
    }

    pub fn rum_event_processing_scale(
        mut self,
        value: crate::datadogV2::model::RUMEventProcessingScale,
    ) -> Self {
        self.rum_event_processing_scale = Some(value);
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

impl Default for RUMProductScales {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RUMProductScales {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RUMProductScalesVisitor;
        impl<'a> Visitor<'a> for RUMProductScalesVisitor {
            type Value = RUMProductScales;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut product_analytics_retention_scale: Option<
                    crate::datadogV2::model::RUMProductAnalyticsRetentionScale,
                > = None;
                let mut rum_event_processing_scale: Option<
                    crate::datadogV2::model::RUMEventProcessingScale,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "product_analytics_retention_scale" => {
                            if v.is_null() {
                                continue;
                            }
                            product_analytics_retention_scale =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rum_event_processing_scale" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_event_processing_scale =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RUMProductScales {
                    product_analytics_retention_scale,
                    rum_event_processing_scale,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RUMProductScalesVisitor)
    }
}
