// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for updating a feature flag.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpdateFeatureFlagAttributes {
    /// The description of the feature flag.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The distribution channel for the feature flag.
    #[serde(rename = "distribution_channel")]
    pub distribution_channel: Option<crate::datadogV2::model::FeatureFlagDistributionChannel>,
    /// JSON schema for validation when value_type is JSON.
    #[serde(
        rename = "json_schema",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub json_schema: Option<Option<String>>,
    /// The name of the feature flag.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The lifecycle to set. `ACTIVE` makes the flag temporary; `PERMANENT` excludes it from
    /// staleness detection. Either value clears any snooze; omit the field to keep the lifecycle.
    /// Do not copy the top-level `staleness_status` from a response into this field: that legacy
    /// response field is always `ACTIVE`.
    #[serde(rename = "staleness_status")]
    pub staleness_status: Option<crate::datadogV2::model::UpdateFeatureFlagStalenessStatus>,
    /// Tags associated with the feature flag. This field replaces the full set of
    /// existing tags; omit it to leave tags unchanged, or pass an empty array to
    /// clear all tags. The owning team is set by including a tag of the form
    /// `team:<team-handle>` in this array.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UpdateFeatureFlagAttributes {
    pub fn new() -> UpdateFeatureFlagAttributes {
        UpdateFeatureFlagAttributes {
            description: None,
            distribution_channel: None,
            json_schema: None,
            name: None,
            staleness_status: None,
            tags: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn distribution_channel(
        mut self,
        value: crate::datadogV2::model::FeatureFlagDistributionChannel,
    ) -> Self {
        self.distribution_channel = Some(value);
        self
    }

    pub fn json_schema(mut self, value: Option<String>) -> Self {
        self.json_schema = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn staleness_status(
        mut self,
        value: crate::datadogV2::model::UpdateFeatureFlagStalenessStatus,
    ) -> Self {
        self.staleness_status = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
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

impl Default for UpdateFeatureFlagAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UpdateFeatureFlagAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpdateFeatureFlagAttributesVisitor;
        impl<'a> Visitor<'a> for UpdateFeatureFlagAttributesVisitor {
            type Value = UpdateFeatureFlagAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut distribution_channel: Option<
                    crate::datadogV2::model::FeatureFlagDistributionChannel,
                > = None;
                let mut json_schema: Option<Option<String>> = None;
                let mut name: Option<String> = None;
                let mut staleness_status: Option<
                    crate::datadogV2::model::UpdateFeatureFlagStalenessStatus,
                > = None;
                let mut tags: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "distribution_channel" => {
                            if v.is_null() {
                                continue;
                            }
                            distribution_channel =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _distribution_channel) = distribution_channel {
                                match _distribution_channel {
                                    crate::datadogV2::model::FeatureFlagDistributionChannel::UnparsedObject(_distribution_channel) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "json_schema" => {
                            json_schema =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "staleness_status" => {
                            if v.is_null() {
                                continue;
                            }
                            staleness_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _staleness_status) = staleness_status {
                                match _staleness_status {
                                    crate::datadogV2::model::UpdateFeatureFlagStalenessStatus::UnparsedObject(_staleness_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = UpdateFeatureFlagAttributes {
                    description,
                    distribution_channel,
                    json_schema,
                    name,
                    staleness_status,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UpdateFeatureFlagAttributesVisitor)
    }
}
