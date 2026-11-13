// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The flag rule the bandit serves through. Absent until a draft is put on a flag, and when the caller lacks feature flag and environment read permissions (`meta.warnings` then says so).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsBanditFlagConfigurationV2 {
    /// The key of the bandit's rule on the flag. Exposures log it.
    #[serde(rename = "allocation_key")]
    pub allocation_key: String,
    /// The environment the bandit serves in.
    #[serde(rename = "environment_id")]
    pub environment_id: uuid::Uuid,
    /// The flag the bandit serves through.
    #[serde(rename = "feature_flag_id")]
    pub feature_flag_id: uuid::Uuid,
    /// Who the bandit serves. Empty serves everyone in the environment.
    #[serde(rename = "targeting_rules")]
    pub targeting_rules: Vec<crate::datadogV2::model::ExperimentsTargetingRule>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsBanditFlagConfigurationV2 {
    pub fn new(
        allocation_key: String,
        environment_id: uuid::Uuid,
        feature_flag_id: uuid::Uuid,
        targeting_rules: Vec<crate::datadogV2::model::ExperimentsTargetingRule>,
    ) -> ExperimentsBanditFlagConfigurationV2 {
        ExperimentsBanditFlagConfigurationV2 {
            allocation_key,
            environment_id,
            feature_flag_id,
            targeting_rules,
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

impl<'de> Deserialize<'de> for ExperimentsBanditFlagConfigurationV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsBanditFlagConfigurationV2Visitor;
        impl<'a> Visitor<'a> for ExperimentsBanditFlagConfigurationV2Visitor {
            type Value = ExperimentsBanditFlagConfigurationV2;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allocation_key: Option<String> = None;
                let mut environment_id: Option<uuid::Uuid> = None;
                let mut feature_flag_id: Option<uuid::Uuid> = None;
                let mut targeting_rules: Option<
                    Vec<crate::datadogV2::model::ExperimentsTargetingRule>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allocation_key" => {
                            allocation_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "environment_id" => {
                            environment_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "feature_flag_id" => {
                            feature_flag_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "targeting_rules" => {
                            targeting_rules =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let allocation_key =
                    allocation_key.ok_or_else(|| M::Error::missing_field("allocation_key"))?;
                let environment_id =
                    environment_id.ok_or_else(|| M::Error::missing_field("environment_id"))?;
                let feature_flag_id =
                    feature_flag_id.ok_or_else(|| M::Error::missing_field("feature_flag_id"))?;
                let targeting_rules =
                    targeting_rules.ok_or_else(|| M::Error::missing_field("targeting_rules"))?;

                let content = ExperimentsBanditFlagConfigurationV2 {
                    allocation_key,
                    environment_id,
                    feature_flag_id,
                    targeting_rules,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsBanditFlagConfigurationV2Visitor)
    }
}
