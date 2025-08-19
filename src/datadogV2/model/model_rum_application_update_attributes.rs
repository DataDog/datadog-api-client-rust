// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// RUM application update attributes.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RUMApplicationUpdateAttributes {
    /// Name of the RUM application.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Controls the retention policy for Product Analytics data derived from RUM events.
    #[serde(rename = "product_analytics_retention_state")]
    pub product_analytics_retention_state:
        Option<crate::datadogV2::model::RUMProductAnalyticsRetentionState>,
    /// Configures which RUM events are processed and stored for the application.
    #[serde(rename = "rum_event_processing_state")]
    pub rum_event_processing_state: Option<crate::datadogV2::model::RUMEventProcessingState>,
    /// Type of the RUM application. Supported values are `browser`, `ios`, `android`, `react-native`, `flutter`, `roku`, `electron`, `unity`, `kotlin-multiplatform`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RUMApplicationUpdateAttributes {
    pub fn new() -> RUMApplicationUpdateAttributes {
        RUMApplicationUpdateAttributes {
            name: None,
            product_analytics_retention_state: None,
            rum_event_processing_state: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn product_analytics_retention_state(
        mut self,
        value: crate::datadogV2::model::RUMProductAnalyticsRetentionState,
    ) -> Self {
        self.product_analytics_retention_state = Some(value);
        self
    }

    pub fn rum_event_processing_state(
        mut self,
        value: crate::datadogV2::model::RUMEventProcessingState,
    ) -> Self {
        self.rum_event_processing_state = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
        self.type_ = Some(value);
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

impl Default for RUMApplicationUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RUMApplicationUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RUMApplicationUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for RUMApplicationUpdateAttributesVisitor {
            type Value = RUMApplicationUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut product_analytics_retention_state: Option<
                    crate::datadogV2::model::RUMProductAnalyticsRetentionState,
                > = None;
                let mut rum_event_processing_state: Option<
                    crate::datadogV2::model::RUMEventProcessingState,
                > = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product_analytics_retention_state" => {
                            if v.is_null() {
                                continue;
                            }
                            product_analytics_retention_state =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _product_analytics_retention_state) =
                                product_analytics_retention_state
                            {
                                match _product_analytics_retention_state {
                                    crate::datadogV2::model::RUMProductAnalyticsRetentionState::UnparsedObject(_product_analytics_retention_state) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "rum_event_processing_state" => {
                            if v.is_null() {
                                continue;
                            }
                            rum_event_processing_state =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _rum_event_processing_state) =
                                rum_event_processing_state
                            {
                                match _rum_event_processing_state {
                                    crate::datadogV2::model::RUMEventProcessingState::UnparsedObject(_rum_event_processing_state) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RUMApplicationUpdateAttributes {
                    name,
                    product_analytics_retention_state,
                    rum_event_processing_state,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RUMApplicationUpdateAttributesVisitor)
    }
}
