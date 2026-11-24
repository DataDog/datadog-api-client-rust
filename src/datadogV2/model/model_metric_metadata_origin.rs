// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Where the metric comes from. Each value is the display label shown on the Metrics Summary page. The set of labels can grow as Datadog adds products. Each field is `null` when it isn't known.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MetricMetadataOrigin {
    /// Product that reports the metric, such as `Agent`.
    #[serialize_always]
    #[serde(rename = "product")]
    pub product: Option<String>,
    /// Finer-grained origin within the sub-product, such as `Cpu`.
    #[serialize_always]
    #[serde(rename = "product_detail")]
    pub product_detail: Option<String>,
    /// Part of the product that reports the metric, such as `Integration`.
    #[serialize_always]
    #[serde(rename = "sub_product")]
    pub sub_product: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MetricMetadataOrigin {
    pub fn new(
        product: Option<String>,
        product_detail: Option<String>,
        sub_product: Option<String>,
    ) -> MetricMetadataOrigin {
        MetricMetadataOrigin {
            product,
            product_detail,
            sub_product,
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

impl<'de> Deserialize<'de> for MetricMetadataOrigin {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MetricMetadataOriginVisitor;
        impl<'a> Visitor<'a> for MetricMetadataOriginVisitor {
            type Value = MetricMetadataOrigin;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut product: Option<Option<String>> = None;
                let mut product_detail: Option<Option<String>> = None;
                let mut sub_product: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "product" => {
                            product = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "product_detail" => {
                            product_detail =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "sub_product" => {
                            sub_product =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let product = product.ok_or_else(|| M::Error::missing_field("product"))?;
                let product_detail =
                    product_detail.ok_or_else(|| M::Error::missing_field("product_detail"))?;
                let sub_product =
                    sub_product.ok_or_else(|| M::Error::missing_field("sub_product"))?;

                let content = MetricMetadataOrigin {
                    product,
                    product_detail,
                    sub_product,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MetricMetadataOriginVisitor)
    }
}
