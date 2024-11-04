// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Mapping of billing dimensions to endpoint keys.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BillingDimensionsMappingBodyItemAttributes {
    /// List of supported endpoints with their keys mapped to the billing_dimension.
    #[serde(rename = "endpoints")]
    pub endpoints: Option<
        Vec<crate::datadogV2::model::BillingDimensionsMappingBodyItemAttributesEndpointsItems>,
    >,
    /// Label used for the billing dimension in the Plan & Usage charts.
    #[serde(rename = "in_app_label")]
    pub in_app_label: Option<String>,
    /// Month in ISO-8601 format, UTC, and precise to the second: `[YYYY-MM-DDThh:mm:ss]`.
    #[serde(rename = "timestamp")]
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl BillingDimensionsMappingBodyItemAttributes {
    pub fn new() -> BillingDimensionsMappingBodyItemAttributes {
        BillingDimensionsMappingBodyItemAttributes {
            endpoints: None,
            in_app_label: None,
            timestamp: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn endpoints(
        mut self,
        value: Vec<
            crate::datadogV2::model::BillingDimensionsMappingBodyItemAttributesEndpointsItems,
        >,
    ) -> Self {
        self.endpoints = Some(value);
        self
    }

    pub fn in_app_label(mut self, value: String) -> Self {
        self.in_app_label = Some(value);
        self
    }

    pub fn timestamp(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.timestamp = Some(value);
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

impl Default for BillingDimensionsMappingBodyItemAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for BillingDimensionsMappingBodyItemAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BillingDimensionsMappingBodyItemAttributesVisitor;
        impl<'a> Visitor<'a> for BillingDimensionsMappingBodyItemAttributesVisitor {
            type Value = BillingDimensionsMappingBodyItemAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut endpoints: Option<Vec<crate::datadogV2::model::BillingDimensionsMappingBodyItemAttributesEndpointsItems>> = None;
                let mut in_app_label: Option<String> = None;
                let mut timestamp: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "endpoints" => {
                            if v.is_null() {
                                continue;
                            }
                            endpoints = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "in_app_label" => {
                            if v.is_null() {
                                continue;
                            }
                            in_app_label =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp" => {
                            if v.is_null() {
                                continue;
                            }
                            timestamp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = BillingDimensionsMappingBodyItemAttributes {
                    endpoints,
                    in_app_label,
                    timestamp,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(BillingDimensionsMappingBodyItemAttributesVisitor)
    }
}
