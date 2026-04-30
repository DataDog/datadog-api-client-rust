// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of a permanent retention filter that can be updated.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumPermanentRetentionFilterUpdateAttributes {
    /// Partial update of the cross-product sample rates for a permanent retention filter.
    /// Only rates with a matching `cross_product_sampling_editability` flag set to `true` can be updated.
    #[serde(rename = "cross_product_sampling")]
    pub cross_product_sampling:
        Option<crate::datadogV2::model::RumPermanentCrossProductSamplingUpdate>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumPermanentRetentionFilterUpdateAttributes {
    pub fn new() -> RumPermanentRetentionFilterUpdateAttributes {
        RumPermanentRetentionFilterUpdateAttributes {
            cross_product_sampling: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn cross_product_sampling(
        mut self,
        value: crate::datadogV2::model::RumPermanentCrossProductSamplingUpdate,
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

impl Default for RumPermanentRetentionFilterUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for RumPermanentRetentionFilterUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumPermanentRetentionFilterUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for RumPermanentRetentionFilterUpdateAttributesVisitor {
            type Value = RumPermanentRetentionFilterUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cross_product_sampling: Option<
                    crate::datadogV2::model::RumPermanentCrossProductSamplingUpdate,
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

                let content = RumPermanentRetentionFilterUpdateAttributes {
                    cross_product_sampling,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumPermanentRetentionFilterUpdateAttributesVisitor)
    }
}
