// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An arm's observations and subjects in total.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsBanditTrafficVariantV2 {
    /// The arm's variant ID on the flag.
    #[serde(rename = "feature_flag_variant_id")]
    pub feature_flag_variant_id: uuid::Uuid,
    /// The arm's valid observations, mature or not.
    #[serde(rename = "observations")]
    pub observations: i64,
    /// Subjects whose first valid observation is on this arm. A subject counts once, on the arm its reward goes to.
    #[serde(rename = "subjects")]
    pub subjects: i64,
    /// The arm's variant key.
    #[serialize_always]
    #[serde(rename = "variant_key")]
    pub variant_key: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsBanditTrafficVariantV2 {
    pub fn new(
        feature_flag_variant_id: uuid::Uuid,
        observations: i64,
        subjects: i64,
        variant_key: Option<String>,
    ) -> ExperimentsBanditTrafficVariantV2 {
        ExperimentsBanditTrafficVariantV2 {
            feature_flag_variant_id,
            observations,
            subjects,
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

impl<'de> Deserialize<'de> for ExperimentsBanditTrafficVariantV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsBanditTrafficVariantV2Visitor;
        impl<'a> Visitor<'a> for ExperimentsBanditTrafficVariantV2Visitor {
            type Value = ExperimentsBanditTrafficVariantV2;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut feature_flag_variant_id: Option<uuid::Uuid> = None;
                let mut observations: Option<i64> = None;
                let mut subjects: Option<i64> = None;
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
                        "observations" => {
                            observations =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "subjects" => {
                            subjects = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let observations =
                    observations.ok_or_else(|| M::Error::missing_field("observations"))?;
                let subjects = subjects.ok_or_else(|| M::Error::missing_field("subjects"))?;
                let variant_key =
                    variant_key.ok_or_else(|| M::Error::missing_field("variant_key"))?;

                let content = ExperimentsBanditTrafficVariantV2 {
                    feature_flag_variant_id,
                    observations,
                    subjects,
                    variant_key,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsBanditTrafficVariantV2Visitor)
    }
}
