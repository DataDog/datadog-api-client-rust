// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Variant in an experiment, with its identity and traffic allocation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsExperimentV2DTODataAttributesVariantsItems {
    /// Backing feature flag variant ID. Present for Datadog feature flag experiments and omitted for Warehouse experiments.
    #[serde(rename = "feature_flag_variant_id")]
    pub feature_flag_variant_id: Option<String>,
    /// Whether this variant participates in the experiment.
    #[serde(rename = "is_active")]
    pub is_active: bool,
    /// Whether this is the single control variant.
    #[serde(rename = "is_control")]
    pub is_control: bool,
    /// Value recorded in exposure data for this variant.
    #[serde(rename = "key")]
    pub key: String,
    /// Display name of the experiment variant.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option")]
    pub name: Option<Option<String>>,
    /// Traffic allocation percentage.
    #[serde(rename = "weight")]
    pub weight: f64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsExperimentV2DTODataAttributesVariantsItems {
    pub fn new(
        is_active: bool,
        is_control: bool,
        key: String,
        weight: f64,
    ) -> ExperimentsExperimentV2DTODataAttributesVariantsItems {
        ExperimentsExperimentV2DTODataAttributesVariantsItems {
            feature_flag_variant_id: None,
            is_active,
            is_control,
            key,
            name: None,
            weight,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn feature_flag_variant_id(mut self, value: String) -> Self {
        self.feature_flag_variant_id = Some(value);
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

impl<'de> Deserialize<'de> for ExperimentsExperimentV2DTODataAttributesVariantsItems {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsExperimentV2DTODataAttributesVariantsItemsVisitor;
        impl<'a> Visitor<'a> for ExperimentsExperimentV2DTODataAttributesVariantsItemsVisitor {
            type Value = ExperimentsExperimentV2DTODataAttributesVariantsItems;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut feature_flag_variant_id: Option<String> = None;
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
                let is_active = is_active.ok_or_else(|| M::Error::missing_field("is_active"))?;
                let is_control = is_control.ok_or_else(|| M::Error::missing_field("is_control"))?;
                let key = key.ok_or_else(|| M::Error::missing_field("key"))?;
                let weight = weight.ok_or_else(|| M::Error::missing_field("weight"))?;

                let content = ExperimentsExperimentV2DTODataAttributesVariantsItems {
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

        deserializer.deserialize_any(ExperimentsExperimentV2DTODataAttributesVariantsItemsVisitor)
    }
}
