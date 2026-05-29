// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a permanent RUM retention filter.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumPermanentRetentionFilterAttributes {
    /// The configuration for cross-product retention filters.
    #[serde(rename = "cross_product_sampling")]
    pub cross_product_sampling: Option<crate::datadogV2::model::RumCrossProductSampling>,
    /// A description of what the filter retains.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Indicates which cross-product fields of a permanent RUM retention filter can be updated.
    #[serde(rename = "editability")]
    pub editability: Option<crate::datadogV2::model::RumPermanentRetentionFilterEditability>,
    /// The display name of the permanent retention filter.
    #[serde(rename = "name")]
    pub name: Option<String>,
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
            description: None,
            editability: None,
            name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cross_product_sampling(
        mut self,
        value: crate::datadogV2::model::RumCrossProductSampling,
    ) -> Self {
        self.cross_product_sampling = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn editability(
        mut self,
        value: crate::datadogV2::model::RumPermanentRetentionFilterEditability,
    ) -> Self {
        self.editability = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
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
                    crate::datadogV2::model::RumCrossProductSampling,
                > = None;
                let mut description: Option<String> = None;
                let mut editability: Option<
                    crate::datadogV2::model::RumPermanentRetentionFilterEditability,
                > = None;
                let mut name: Option<String> = None;
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
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "editability" => {
                            if v.is_null() {
                                continue;
                            }
                            editability =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                    description,
                    editability,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumPermanentRetentionFilterAttributesVisitor)
    }
}
