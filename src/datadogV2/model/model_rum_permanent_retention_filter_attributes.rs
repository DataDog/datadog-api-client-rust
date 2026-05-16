// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a permanent retention filter.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumPermanentRetentionFilterAttributes {
    /// Cross-product retention settings for a permanent retention filter.
    #[serde(rename = "cross_product_sampling")]
    pub cross_product_sampling: Option<crate::datadogV2::model::RumPermanentCrossProductSampling>,
    /// Flags indicating which `cross_product_sampling` rates can be updated for this filter. Read-only.
    #[serde(rename = "cross_product_sampling_editability")]
    pub cross_product_sampling_editability:
        Option<crate::datadogV2::model::RumPermanentCrossProductSamplingEditability>,
    /// Indicates whether the permanent retention filter is active. Read-only.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The type of RUM events the filter applies to. Read-only.
    #[serde(rename = "event_type")]
    pub event_type: Option<crate::datadogV2::model::RumPermanentRetentionFilterEventType>,
    /// The name of a permanent retention filter. Read-only.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The query string for a permanent retention filter. Read-only.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// The retention sample rate for a permanent retention filter, from 0 to 100. Read-only.
    #[serde(rename = "sample_rate")]
    pub sample_rate: Option<f64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumPermanentRetentionFilterAttributes {
    pub fn new() -> RumPermanentRetentionFilterAttributes {
        RumPermanentRetentionFilterAttributes {
            cross_product_sampling: None,
            cross_product_sampling_editability: None,
            enabled: None,
            event_type: None,
            name: None,
            query: None,
            sample_rate: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cross_product_sampling(
        mut self,
        value: crate::datadogV2::model::RumPermanentCrossProductSampling,
    ) -> Self {
        self.cross_product_sampling = Some(value);
        self
    }

    pub fn cross_product_sampling_editability(
        mut self,
        value: crate::datadogV2::model::RumPermanentCrossProductSamplingEditability,
    ) -> Self {
        self.cross_product_sampling_editability = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn event_type(
        mut self,
        value: crate::datadogV2::model::RumPermanentRetentionFilterEventType,
    ) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn sample_rate(mut self, value: f64) -> Self {
        self.sample_rate = Some(value);
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

impl Default for RumPermanentRetentionFilterAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RumPermanentRetentionFilterAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumPermanentRetentionFilterAttributesVisitor;
        impl<'a> Visitor<'a> for RumPermanentRetentionFilterAttributesVisitor {
            type Value = RumPermanentRetentionFilterAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cross_product_sampling: Option<
                    crate::datadogV2::model::RumPermanentCrossProductSampling,
                > = None;
                let mut cross_product_sampling_editability: Option<
                    crate::datadogV2::model::RumPermanentCrossProductSamplingEditability,
                > = None;
                let mut enabled: Option<bool> = None;
                let mut event_type: Option<
                    crate::datadogV2::model::RumPermanentRetentionFilterEventType,
                > = None;
                let mut name: Option<String> = None;
                let mut query: Option<String> = None;
                let mut sample_rate: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cross_product_sampling" => {
                            if v.is_null() {
                                continue;
                            }
                            cross_product_sampling =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cross_product_sampling_editability" => {
                            if v.is_null() {
                                continue;
                            }
                            cross_product_sampling_editability =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event_type" => {
                            if v.is_null() {
                                continue;
                            }
                            event_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _event_type) = event_type {
                                match _event_type {
                                    crate::datadogV2::model::RumPermanentRetentionFilterEventType::UnparsedObject(_event_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sample_rate" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            sample_rate =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RumPermanentRetentionFilterAttributes {
                    cross_product_sampling,
                    cross_product_sampling_editability,
                    enabled,
                    event_type,
                    name,
                    query,
                    sample_rate,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumPermanentRetentionFilterAttributesVisitor)
    }
}
