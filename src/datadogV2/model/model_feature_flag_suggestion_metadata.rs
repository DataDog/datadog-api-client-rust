// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata associated with a suggestion. Fields other than `variant_id` are set by the server.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FeatureFlagSuggestionMetadata {
    /// Whether the server allows this allocation suggestion to reorder persisted drafts.
    #[serde(rename = "allow_draft_reorder")]
    pub allow_draft_reorder: Option<bool>,
    /// Server-generated reference to an experiment action requiring approval.
    #[serde(rename = "experiment_action")]
    pub experiment_action: Option<crate::datadogV2::model::FeatureFlagSuggestionExperimentAction>,
    /// Server-generated fingerprint of the experiment action request.
    #[serde(rename = "experiment_action_request_fingerprint")]
    pub experiment_action_request_fingerprint: Option<String>,
    /// The ID of the variant targeted by the suggestion.
    #[serde(rename = "variant_id")]
    pub variant_id: Option<uuid::Uuid>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FeatureFlagSuggestionMetadata {
    pub fn new() -> FeatureFlagSuggestionMetadata {
        FeatureFlagSuggestionMetadata {
            allow_draft_reorder: None,
            experiment_action: None,
            experiment_action_request_fingerprint: None,
            variant_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn allow_draft_reorder(mut self, value: bool) -> Self {
        self.allow_draft_reorder = Some(value);
        self
    }

    pub fn experiment_action(
        mut self,
        value: crate::datadogV2::model::FeatureFlagSuggestionExperimentAction,
    ) -> Self {
        self.experiment_action = Some(value);
        self
    }

    pub fn experiment_action_request_fingerprint(mut self, value: String) -> Self {
        self.experiment_action_request_fingerprint = Some(value);
        self
    }

    pub fn variant_id(mut self, value: uuid::Uuid) -> Self {
        self.variant_id = Some(value);
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

impl Default for FeatureFlagSuggestionMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FeatureFlagSuggestionMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FeatureFlagSuggestionMetadataVisitor;
        impl<'a> Visitor<'a> for FeatureFlagSuggestionMetadataVisitor {
            type Value = FeatureFlagSuggestionMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allow_draft_reorder: Option<bool> = None;
                let mut experiment_action: Option<
                    crate::datadogV2::model::FeatureFlagSuggestionExperimentAction,
                > = None;
                let mut experiment_action_request_fingerprint: Option<String> = None;
                let mut variant_id: Option<uuid::Uuid> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allow_draft_reorder" => {
                            if v.is_null() {
                                continue;
                            }
                            allow_draft_reorder =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "experiment_action" => {
                            if v.is_null() {
                                continue;
                            }
                            experiment_action =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "experiment_action_request_fingerprint" => {
                            if v.is_null() {
                                continue;
                            }
                            experiment_action_request_fingerprint =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variant_id" => {
                            if v.is_null() {
                                continue;
                            }
                            variant_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FeatureFlagSuggestionMetadata {
                    allow_draft_reorder,
                    experiment_action,
                    experiment_action_request_fingerprint,
                    variant_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FeatureFlagSuggestionMetadataVisitor)
    }
}
