// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Server-generated reference to an experiment action requiring approval.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FeatureFlagSuggestionExperimentAction {
    /// The proposed experiment action.
    #[serde(rename = "action")]
    pub action: String,
    /// The ID of the experiment.
    #[serde(rename = "experiment_id")]
    pub experiment_id: uuid::Uuid,
    /// The version of the experiment action reference.
    #[serde(rename = "version")]
    pub version: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FeatureFlagSuggestionExperimentAction {
    pub fn new(
        action: String,
        experiment_id: uuid::Uuid,
        version: i64,
    ) -> FeatureFlagSuggestionExperimentAction {
        FeatureFlagSuggestionExperimentAction {
            action,
            experiment_id,
            version,
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

impl<'de> Deserialize<'de> for FeatureFlagSuggestionExperimentAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FeatureFlagSuggestionExperimentActionVisitor;
        impl<'a> Visitor<'a> for FeatureFlagSuggestionExperimentActionVisitor {
            type Value = FeatureFlagSuggestionExperimentAction;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<String> = None;
                let mut experiment_id: Option<uuid::Uuid> = None;
                let mut version: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "action" => {
                            action = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "experiment_id" => {
                            experiment_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let action = action.ok_or_else(|| M::Error::missing_field("action"))?;
                let experiment_id =
                    experiment_id.ok_or_else(|| M::Error::missing_field("experiment_id"))?;
                let version = version.ok_or_else(|| M::Error::missing_field("version"))?;

                let content = FeatureFlagSuggestionExperimentAction {
                    action,
                    experiment_id,
                    version,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FeatureFlagSuggestionExperimentActionVisitor)
    }
}
