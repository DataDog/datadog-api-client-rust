// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Variant selected for an experiment, with its identity and traffic allocation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsCreateExperimentV2RequestDataAttributesVariantsItems {
    /// Stable backing flag variant ID. Required for Datadog-backed experiments and omitted for Warehouse-only experiments.
    #[serde(rename = "feature_flag_variant_id")]
    pub feature_flag_variant_id: Option<uuid::Uuid>,
    /// Whether this variant participates in the experiment. Responses always include this field. On writes an included variant defaults to active; omit its row to remove or unselect it. Explicit false supports sending an unchanged response back.
    #[serde(rename = "is_active")]
    pub is_active: Option<bool>,
    /// Whether this is the single control variant.
    #[serde(rename = "is_control")]
    pub is_control: bool,
    /// Assignment value. Datadog flag variant keys are server-owned.
    #[serde(rename = "key")]
    pub key: String,
    /// Display name. Datadog flag variant names are server-owned.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option")]
    pub name: Option<Option<String>>,
    /// Traffic allocation percentage. Existing variant weights cannot change through the public API after the experiment starts.
    #[serde(rename = "weight")]
    pub weight: f64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsCreateExperimentV2RequestDataAttributesVariantsItems {
    pub fn new(
        is_control: bool,
        key: String,
        weight: f64,
    ) -> ExperimentsCreateExperimentV2RequestDataAttributesVariantsItems {
        ExperimentsCreateExperimentV2RequestDataAttributesVariantsItems {
            feature_flag_variant_id: None,
            is_active: None,
            is_control,
            key,
            name: None,
            weight,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn feature_flag_variant_id(mut self, value: uuid::Uuid) -> Self {
        self.feature_flag_variant_id = Some(value);
        self
    }

    pub fn is_active(mut self, value: bool) -> Self {
        self.is_active = Some(value);
        self
    }

    pub fn name(mut self, value: Option<String>) -> Self {
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

impl<'de> Deserialize<'de> for ExperimentsCreateExperimentV2RequestDataAttributesVariantsItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsCreateExperimentV2RequestDataAttributesVariantsItemsVisitor;
        impl<'a> Visitor<'a> for ExperimentsCreateExperimentV2RequestDataAttributesVariantsItemsVisitor {
            type Value = ExperimentsCreateExperimentV2RequestDataAttributesVariantsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut feature_flag_variant_id: Option<uuid::Uuid> = None;
                let mut is_active: Option<bool> = None;
                let mut is_control: Option<bool> = None;
                let mut key: Option<String> = None;
                let mut name: Option<Option<String>> = None;
                let mut weight: Option<f64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "feature_flag_variant_id" => {
                            if v.is_null() {
                                continue;
                            }
                            feature_flag_variant_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_active" => {
                            if v.is_null() {
                                continue;
                            }
                            is_active = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_control" => {
                            is_control = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key" => {
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "weight" => {
                            weight = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let is_control = is_control.ok_or_else(|| M::Error::missing_field("is_control"))?;
                let key = key.ok_or_else(|| M::Error::missing_field("key"))?;
                let weight = weight.ok_or_else(|| M::Error::missing_field("weight"))?;

                let content = ExperimentsCreateExperimentV2RequestDataAttributesVariantsItems {
                    feature_flag_variant_id,
                    is_active,
                    is_control,
                    key,
                    name,
                    weight,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(ExperimentsCreateExperimentV2RequestDataAttributesVariantsItemsVisitor)
    }
}
