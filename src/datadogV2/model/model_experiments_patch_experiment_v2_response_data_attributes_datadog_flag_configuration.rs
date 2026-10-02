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
pub struct ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfiguration {
    /// Key of the feature flag allocation linked to the experiment.
    #[serde(rename = "allocation_key")]
    pub allocation_key: Option<String>,
    /// Datadog measure and filters used to select analyzed subjects.
	#[serialize_always]
    #[serde(rename = "entry_point")]
    pub entry_point: Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationEntryPoint>,
    /// ID of the feature flag environment used by the experiment.
    #[serde(rename = "environment_id")]
    pub environment_id: Option<String>,
    /// ID of the feature flag linked to the experiment.
    #[serde(rename = "feature_flag_id")]
    pub feature_flag_id: Option<String>,
    /// Rules that select subjects for the experiment.
    #[serde(rename = "targeting_rules")]
    pub targeting_rules: Option<Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesDatadogFlagConfigurationTargetingRulesItems>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfiguration {
    pub fn new(
        entry_point: Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationEntryPoint>,
    ) -> ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfiguration {
        ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfiguration {
            allocation_key: None,
            entry_point,
            environment_id: None,
            feature_flag_id: None,
            targeting_rules: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn allocation_key(mut self, value: String) -> Self {
        self.allocation_key = Some(value);
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

impl<'de> Deserialize<'de>
    for ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfiguration
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationVisitor;
        impl<'a> Visitor<'a>
            for ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationVisitor
        {
            type Value = ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfiguration;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allocation_key: Option<String> = None;
                let mut entry_point: Option<Option<crate::datadogV2::model::ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationEntryPoint>> = None;
                let mut environment_id: Option<String> = None;
                let mut feature_flag_id: Option<String> = None;
                let mut targeting_rules: Option<Vec<crate::datadogV2::model::ExperimentsCreateExperimentV2RequestDataAttributesDatadogFlagConfigurationTargetingRulesItems>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allocation_key" => {
                            if v.is_null() {
                                continue;
                            }
                            allocation_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                let entry_point =
                    entry_point.ok_or_else(|| M::Error::missing_field("entry_point"))?;

                let content =
                    ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfiguration {
                        allocation_key,
                        entry_point,
                        environment_id,
                        feature_flag_id,
                        targeting_rules,
                        additional_properties,
                        _unparsed,
                    };

                Ok(content)
            }
        }

        deserializer.deserialize_any(
            ExperimentsPatchExperimentV2ResponseDataAttributesDatadogFlagConfigurationVisitor,
        )
    }
}
