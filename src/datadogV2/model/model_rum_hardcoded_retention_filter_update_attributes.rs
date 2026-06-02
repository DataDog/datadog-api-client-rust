// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a hardcoded retention filter that can be updated.
/// Only fields whose matching flag in `cross_product_sampling_editability` is `true` can be modified.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumHardcodedRetentionFilterUpdateAttributes {
    /// Partial update for cross-product retention of a hardcoded retention filter.
    /// Only fields whose matching flag in `cross_product_sampling_editability` is `true` can be updated.
    #[serde(rename = "cross_product_sampling")]
    pub cross_product_sampling:
        Option<crate::datadogV2::model::RumHardcodedCrossProductSamplingUpdate>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumHardcodedRetentionFilterUpdateAttributes {
    pub fn new() -> RumHardcodedRetentionFilterUpdateAttributes {
        RumHardcodedRetentionFilterUpdateAttributes {
            cross_product_sampling: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cross_product_sampling(
        mut self,
        value: crate::datadogV2::model::RumHardcodedCrossProductSamplingUpdate,
    ) -> Self {
        self.cross_product_sampling = Some(value);
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

impl Default for RumHardcodedRetentionFilterUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RumHardcodedRetentionFilterUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumHardcodedRetentionFilterUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for RumHardcodedRetentionFilterUpdateAttributesVisitor {
            type Value = RumHardcodedRetentionFilterUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cross_product_sampling: Option<
                    crate::datadogV2::model::RumHardcodedCrossProductSamplingUpdate,
                > = None;
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = RumHardcodedRetentionFilterUpdateAttributes {
                    cross_product_sampling,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumHardcodedRetentionFilterUpdateAttributesVisitor)
    }
}
