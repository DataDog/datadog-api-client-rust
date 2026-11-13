// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A variant's share of traffic under a policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsBanditPolicyVariantV2 {
    /// The variant's ID on the flag.
    #[serde(rename = "feature_flag_variant_id")]
    pub feature_flag_variant_id: uuid::Uuid,
    /// The share of traffic the variant gets, from 0 to 1.
    #[serde(rename = "probability")]
    pub probability: f64,
    /// The variant's key. Null when it can't be resolved.
    #[serialize_always]
    #[serde(rename = "variant_key")]
    pub variant_key: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsBanditPolicyVariantV2 {
    pub fn new(
        feature_flag_variant_id: uuid::Uuid,
        probability: f64,
        variant_key: Option<String>,
    ) -> ExperimentsBanditPolicyVariantV2 {
        ExperimentsBanditPolicyVariantV2 {
            feature_flag_variant_id,
            probability,
            variant_key,
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

impl<'de> Deserialize<'de> for ExperimentsBanditPolicyVariantV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsBanditPolicyVariantV2Visitor;
        impl<'a> Visitor<'a> for ExperimentsBanditPolicyVariantV2Visitor {
            type Value = ExperimentsBanditPolicyVariantV2;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut feature_flag_variant_id: Option<uuid::Uuid> = None;
                let mut probability: Option<f64> = None;
                let mut variant_key: Option<Option<String>> = None;
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
                        "probability" => {
                            probability =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variant_key" => {
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
                let feature_flag_variant_id = feature_flag_variant_id
                    .ok_or_else(|| M::Error::missing_field("feature_flag_variant_id"))?;
                let probability =
                    probability.ok_or_else(|| M::Error::missing_field("probability"))?;
                let variant_key =
                    variant_key.ok_or_else(|| M::Error::missing_field("variant_key"))?;

                let content = ExperimentsBanditPolicyVariantV2 {
                    feature_flag_variant_id,
                    probability,
                    variant_key,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsBanditPolicyVariantV2Visitor)
    }
}
