// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Feature flag, environment, and targeting configuration for the experiment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsPatchExperimentV2RequestDataAttributesDatadogFlagConfiguration {
    /// Datadog measure and filters used to select analyzed subjects.
    #[serde(rename = "entry_point", default, with = "::serde_with::rust::double_option")]
    pub entry_point: Option<Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationEntryPoint>>,
    /// ID of the feature flag environment used by the experiment.
    #[serde(rename = "environment_id")]
    pub environment_id: Option<String>,
    /// ID of the feature flag linked to the experiment.
    #[serde(rename = "feature_flag_id")]
    pub feature_flag_id: Option<String>,
    /// Set true to replace an existing draft allocation when feature_flag_id changes. The replacement resets all flag-bound randomization state.
    #[serde(rename = "reset_on_feature_flag_change")]
    pub reset_on_feature_flag_change: Option<bool>,
    /// Omit to keep the stored rules. Use an empty array to remove targeting rules.
    #[serde(rename = "targeting_rules")]
    pub targeting_rules: Option<Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesDatadogFlagConfigurationTargetingRulesItems>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsPatchExperimentV2RequestDataAttributesDatadogFlagConfiguration {
    pub fn new() -> ExperimentsPatchExperimentV2RequestDataAttributesDatadogFlagConfiguration {
        ExperimentsPatchExperimentV2RequestDataAttributesDatadogFlagConfiguration {
            entry_point: None,
            environment_id: None,
            feature_flag_id: None,
            reset_on_feature_flag_change: None,
            targeting_rules: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn entry_point(
        mut self,
        value: Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationEntryPoint>,
    ) -> Self {
        self.entry_point = Some(value);
        self
    }

    pub fn environment_id(mut self, value: String) -> Self {
        self.environment_id = Some(value);
        self
    }

    pub fn feature_flag_id(mut self, value: String) -> Self {
        self.feature_flag_id = Some(value);
        self
    }

    pub fn reset_on_feature_flag_change(mut self, value: bool) -> Self {
        self.reset_on_feature_flag_change = Some(value);
        self
    }

    pub fn targeting_rules(
        mut self,
        value: Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesDatadogFlagConfigurationTargetingRulesItems>,
    ) -> Self {
        self.targeting_rules = Some(value);
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

impl Default for ExperimentsPatchExperimentV2RequestDataAttributesDatadogFlagConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de>
    for ExperimentsPatchExperimentV2RequestDataAttributesDatadogFlagConfiguration
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsPatchExperimentV2RequestDataAttributesDatadogFlagConfigurationVisitor;
        impl<'a> Visitor<'a>
            for ExperimentsPatchExperimentV2RequestDataAttributesDatadogFlagConfigurationVisitor
        {
            type Value = ExperimentsPatchExperimentV2RequestDataAttributesDatadogFlagConfiguration;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut entry_point: Option<Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationEntryPoint>> = None;
                let mut environment_id: Option<String> = None;
                let mut feature_flag_id: Option<String> = None;
                let mut reset_on_feature_flag_change: Option<bool> = None;
                let mut targeting_rules: Option<Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesDatadogFlagConfigurationTargetingRulesItems>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "entry_point" => {
                            entry_point =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "environment_id" => {
                            if v.is_null() {
                                continue;
                            }
                            environment_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "feature_flag_id" => {
                            if v.is_null() {
                                continue;
                            }
                            feature_flag_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "reset_on_feature_flag_change" => {
                            if v.is_null() {
                                continue;
                            }
                            reset_on_feature_flag_change =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "targeting_rules" => {
                            if v.is_null() {
                                continue;
                            }
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

                let content =
                    ExperimentsPatchExperimentV2RequestDataAttributesDatadogFlagConfiguration {
                        entry_point,
                        environment_id,
                        feature_flag_id,
                        reset_on_feature_flag_change,
                        targeting_rules,
                        additional_properties,
                        _unparsed,
                    };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ExperimentsPatchExperimentV2RequestDataAttributesDatadogFlagConfigurationVisitor,
        )
    }
}
