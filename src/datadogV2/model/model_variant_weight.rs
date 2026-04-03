// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Variant weight details.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct VariantWeight {
    /// The timestamp when the variant weight was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Unique identifier of the variant weight assignment.
    #[serde(rename = "id")]
    pub id: Option<uuid::Uuid>,
    /// The timestamp when the variant weight was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The percentage weight for the variant.
    #[serde(rename = "value")]
    pub value: f64,
    /// A variant of a feature flag.
    #[serde(rename = "variant")]
    pub variant: Option<crate::datadogV2::model::Variant>,
    /// The variant ID.
    #[serde(rename = "variant_id")]
    pub variant_id: uuid::Uuid,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl VariantWeight {
    pub fn new(value: f64, variant_id: uuid::Uuid) -> VariantWeight {
        VariantWeight {
            created_at: None,
            id: None,
            updated_at: None,
            value,
            variant: None,
            variant_id,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: uuid::Uuid) -> Self {
        self.id = Some(value);
        self
    }

    pub fn updated_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn variant(mut self, value: crate::datadogV2::model::Variant) -> Self {
        self.variant = Some(value);
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

impl<'de> Deserialize<'de> for VariantWeight {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct VariantWeightVisitor;
        impl<'a> Visitor<'a> for VariantWeightVisitor {
            type Value = VariantWeight;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut id: Option<uuid::Uuid> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut value: Option<f64> = None;
                let mut variant: Option<crate::datadogV2::model::Variant> = None;
                let mut variant_id: Option<uuid::Uuid> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value" => {
                            value = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variant" => {
                            if v.is_null() {
                                continue;
                            }
                            variant = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variant_id" => {
                            variant_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let value = value.ok_or_else(|| M::Error::missing_field("value"))?;
                let variant_id = variant_id.ok_or_else(|| M::Error::missing_field("variant_id"))?;

                let content = VariantWeight {
                    created_at,
                    id,
                    updated_at,
                    value,
                    variant,
                    variant_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(VariantWeightVisitor)
    }
}
