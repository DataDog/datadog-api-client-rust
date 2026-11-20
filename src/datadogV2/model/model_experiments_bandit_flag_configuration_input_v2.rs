// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// The flag and environment the bandit serves on, and who it serves there. The server chooses the rule's `allocation_key`. On create, `feature_flag_id` and `environment_id` are required, and `null` is the same as omitting it. On update, omitted fields are unchanged, and `null` takes a draft off its flag, which also clears its variants and scheduled end. Moving a draft to another flag clears its variants unless the request sets them. Only a draft can change it.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsBanditFlagConfigurationInputV2 {
    /// The environment the bandit serves in.
    #[serde(rename = "environment_id")]
    pub environment_id: Option<uuid::Uuid>,
    /// The flag the bandit serves through.
    #[serde(rename = "feature_flag_id")]
    pub feature_flag_id: Option<uuid::Uuid>,
    /// Who the bandit serves. When empty, the bandit serves everyone in the environment.
    #[serde(rename = "targeting_rules")]
    pub targeting_rules: Option<Vec<crate::datadogV2::model::ExperimentsTargetingRule>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsBanditFlagConfigurationInputV2 {
    pub fn new() -> ExperimentsBanditFlagConfigurationInputV2 {
        ExperimentsBanditFlagConfigurationInputV2 {
            environment_id: None,
            feature_flag_id: None,
            targeting_rules: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn environment_id(mut self, value: uuid::Uuid) -> Self {
        self.environment_id = Some(value);
        self
    }
    
    pub fn feature_flag_id(mut self, value: uuid::Uuid) -> Self {
        self.feature_flag_id = Some(value);
        self
    }
    
    pub fn targeting_rules(mut self, value: Vec<crate::datadogV2::model::ExperimentsTargetingRule>) -> Self {
        self.targeting_rules = Some(value);
        self
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}


impl Default for ExperimentsBanditFlagConfigurationInputV2 {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ExperimentsBanditFlagConfigurationInputV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsBanditFlagConfigurationInputV2Visitor;
        impl<'a> Visitor<'a> for ExperimentsBanditFlagConfigurationInputV2Visitor {
            type Value = ExperimentsBanditFlagConfigurationInputV2;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut environment_id: Option<uuid::Uuid> = None;
                let mut feature_flag_id: Option<uuid::Uuid> = None;
                let mut targeting_rules: Option<Vec<crate::datadogV2::model::ExperimentsTargetingRule>> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "environment_id" => {
                            if v.is_null() {
                                continue;
                            }
                            environment_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "feature_flag_id" => {
                            if v.is_null() {
                                continue;
                            }
                            feature_flag_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "targeting_rules" => {
                            if v.is_null() {
                                continue;
                            }
                            targeting_rules = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }

                let content = ExperimentsBanditFlagConfigurationInputV2 {
                    environment_id,
                    feature_flag_id,
                    targeting_rules,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsBanditFlagConfigurationInputV2Visitor)
    }
}
