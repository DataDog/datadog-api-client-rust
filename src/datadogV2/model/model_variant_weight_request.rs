// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Variant weight request payload.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct VariantWeightRequest {
    /// The percentage weight for this variant.
    #[serde(rename = "value")]
    pub value: f64,
    /// The variant ID to assign weight to.
    #[serde(rename = "variant_id")]
    pub variant_id: Option<uuid::Uuid>,
    /// The variant key to assign weight to.
    #[serde(rename = "variant_key")]
    pub variant_key: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl VariantWeightRequest {
    pub fn new(value: f64) -> VariantWeightRequest {
        VariantWeightRequest {
            value,
            variant_id: None,
            variant_key: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn variant_id(mut self, value: uuid::Uuid) -> Self {
        self.variant_id = Some(value);
        self
    }

    pub fn variant_key(mut self, value: String) -> Self {
        self.variant_key = Some(value);
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

impl<'de> Deserialize<'de> for VariantWeightRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VariantWeightRequestVisitor;
        impl<'a> Visitor<'a> for VariantWeightRequestVisitor {
            type Value = VariantWeightRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut value: Option<f64> = None;
                let mut variant_id: Option<uuid::Uuid> = None;
                let mut variant_key: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variant_id" => {
                            if v.is_null() {
                                continue;
                            }
                            variant_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variant_key" => {
                            if v.is_null() {
                                continue;
                            }
                            variant_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;

                let content = VariantWeightRequest {
                    value,
                    variant_id,
                    variant_key,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(VariantWeightRequestVisitor)
    }
}
