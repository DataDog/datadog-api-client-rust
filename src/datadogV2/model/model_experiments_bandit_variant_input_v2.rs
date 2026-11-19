// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A flag variant the bandit uses, named by its ID. The variant's `key` and `name` come from the flag and can't be set.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsBanditVariantInputV2 {
    /// The variant's ID on the flag.
    #[serde(rename = "feature_flag_variant_id")]
    pub feature_flag_variant_id: uuid::Uuid,
    /// Whether the variant is one of the arms the bandit chooses between. Defaults to `true`.
    #[serde(rename = "is_active")]
    pub is_active: Option<bool>,
    /// Whether the variant is served when the bandit reaches its scheduled end. At most one variant is the fallback, and it doesn't have to be an arm. Defaults to `false`.
    #[serde(rename = "is_fallback")]
    pub is_fallback: Option<bool>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsBanditVariantInputV2 {
    pub fn new(feature_flag_variant_id: uuid::Uuid) -> ExperimentsBanditVariantInputV2 {
        ExperimentsBanditVariantInputV2 {
            feature_flag_variant_id,
            is_active: None,
            is_fallback: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn is_fallback(mut self, value: bool) -> Self {
        self.is_fallback = Some(value);
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

impl<'de> Deserialize<'de> for ExperimentsBanditVariantInputV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsBanditVariantInputV2Visitor;
        impl<'a> Visitor<'a> for ExperimentsBanditVariantInputV2Visitor {
            type Value = ExperimentsBanditVariantInputV2;

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
                            if v.is_null() {
                                continue;
                            }
                            is_active = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_fallback" => {
                            if v.is_null() {
                                continue;
                            }
                            is_fallback =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = ExperimentsBanditVariantInputV2 {
                    feature_flag_variant_id,
                    is_active,
                    is_fallback,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsBanditVariantInputV2Visitor)
    }
}
