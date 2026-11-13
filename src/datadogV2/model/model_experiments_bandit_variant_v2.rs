// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A flag variant the bandit uses.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsBanditVariantV2 {
    /// The variant's ID on the flag.
    #[serde(rename = "feature_flag_variant_id")]
    pub feature_flag_variant_id: uuid::Uuid,
    /// Whether the variant is one of the arms the bandit chooses between.
    #[serde(rename = "is_active")]
    pub is_active: bool,
    /// Whether the variant is served when the bandit reaches its scheduled end. At most one variant is the fallback.
    #[serde(rename = "is_fallback")]
    pub is_fallback: bool,
    /// The variant's key. An arm keeps the key it had when it was added, because exposures are matched on it. Null only for a fallback deleted from the flag.
    #[serialize_always]
    #[serde(rename = "key")]
    pub key: Option<String>,
    /// The variant's name on the flag. Null when it is no longer on the flag.
    #[serialize_always]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsBanditVariantV2 {
    pub fn new(
        feature_flag_variant_id: uuid::Uuid,
        is_active: bool,
        is_fallback: bool,
        key: Option<String>,
        name: Option<String>,
    ) -> ExperimentsBanditVariantV2 {
        ExperimentsBanditVariantV2 {
            feature_flag_variant_id,
            is_active,
            is_fallback,
            key,
            name,
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

impl<'de> Deserialize<'de> for ExperimentsBanditVariantV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsBanditVariantV2Visitor;
        impl<'a> Visitor<'a> for ExperimentsBanditVariantV2Visitor {
            type Value = ExperimentsBanditVariantV2;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut feature_flag_variant_id: Option<uuid::Uuid> = None;
                let mut is_active: Option<bool> = None;
                let mut is_fallback: Option<bool> = None;
                let mut key: Option<Option<String>> = None;
                let mut name: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "feature_flag_variant_id" => {
                            feature_flag_variant_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_active" => {
                            is_active = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_fallback" => {
                            is_fallback =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key" => {
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let feature_flag_variant_id = feature_flag_variant_id
                    .ok_or_else(|| M::Error::missing_field("feature_flag_variant_id"))?;
                let is_active = is_active.ok_or_else(|| M::Error::missing_field("is_active"))?;
                let is_fallback =
                    is_fallback.ok_or_else(|| M::Error::missing_field("is_fallback"))?;
                let key = key.ok_or_else(|| M::Error::missing_field("key"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;

                let content = ExperimentsBanditVariantV2 {
                    feature_flag_variant_id,
                    is_active,
                    is_fallback,
                    key,
                    name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsBanditVariantV2Visitor)
    }
}
