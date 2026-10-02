// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Feature flag, environment, and targeting configuration for a Datadog experiment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsCreateExperimentV2RequestDataAttributesDatadogFlagConfiguration {
    /// Datadog measure and filters used to select analyzed subjects.
	#[serialize_always]
    #[serde(rename = "entry_point")]
    pub entry_point: Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationEntryPoint>,
    /// Identifier of the feature flag environment.
    #[serde(rename = "environment_id")]
    pub environment_id: uuid::Uuid,
    /// Identifier of the Datadog feature flag used by the experiment.
    #[serde(rename = "feature_flag_id")]
    pub feature_flag_id: uuid::Uuid,
    /// Accepted on create but has no effect.
    #[serde(rename = "reset_on_feature_flag_change")]
    pub reset_on_feature_flag_change: Option<bool>,
    /// Use an empty array when no targeting rules apply.
    #[serde(rename = "targeting_rules")]
    pub targeting_rules: Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesDatadogFlagConfigurationTargetingRulesItems>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsCreateExperimentV2RequestDataAttributesDatadogFlagConfiguration {
    pub fn new(
        entry_point: Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationEntryPoint>,
        environment_id: uuid::Uuid,
        feature_flag_id: uuid::Uuid,
        targeting_rules: Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesDatadogFlagConfigurationTargetingRulesItems>,
    ) -> ExperimentsCreateExperimentV2RequestDataAttributesDatadogFlagConfiguration {
        ExperimentsCreateExperimentV2RequestDataAttributesDatadogFlagConfiguration {
            entry_point,
            environment_id,
            feature_flag_id,
            reset_on_feature_flag_change: None,
            targeting_rules,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn reset_on_feature_flag_change(mut self, value: bool) -> Self {
        self.reset_on_feature_flag_change = Some(value);
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

impl<'de> Deserialize<'de>
    for ExperimentsCreateExperimentV2RequestDataAttributesDatadogFlagConfiguration
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsCreateExperimentV2RequestDataAttributesDatadogFlagConfigurationVisitor;
        impl<'a> Visitor<'a>
            for ExperimentsCreateExperimentV2RequestDataAttributesDatadogFlagConfigurationVisitor
        {
            type Value = ExperimentsCreateExperimentV2RequestDataAttributesDatadogFlagConfiguration;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut entry_point: Option<Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationEntryPoint>> = None;
                let mut environment_id: Option<uuid::Uuid> = None;
                let mut feature_flag_id: Option<uuid::Uuid> = None;
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
                            environment_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "feature_flag_id" => {
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
                let entry_point =
                    entry_point.ok_or_else(|| M::Error::missing_field("entry_point"))?;
                let environment_id =
                    environment_id.ok_or_else(|| M::Error::missing_field("environment_id"))?;
                let feature_flag_id =
                    feature_flag_id.ok_or_else(|| M::Error::missing_field("feature_flag_id"))?;
                let targeting_rules =
                    targeting_rules.ok_or_else(|| M::Error::missing_field("targeting_rules"))?;

                let content =
                    ExperimentsCreateExperimentV2RequestDataAttributesDatadogFlagConfiguration {
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
            ExperimentsCreateExperimentV2RequestDataAttributesDatadogFlagConfigurationVisitor,
        )
    }
}
