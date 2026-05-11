// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a feature flag.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FeatureFlagAttributes {
    /// The timestamp when the feature flag was archived.
    #[serde(
        rename = "archived_at",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub archived_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// The timestamp when the feature flag was created.
    #[serde(rename = "created_at")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The ID of the user who created the feature flag.
    #[serde(rename = "created_by")]
    pub created_by: Option<uuid::Uuid>,
    /// The description of the feature flag.
    #[serde(rename = "description")]
    pub description: String,
    /// Distribution channel for the feature flag.
    #[serde(rename = "distribution_channel")]
    pub distribution_channel: Option<String>,
    /// Environment-specific settings for the feature flag.
    #[serde(rename = "feature_flag_environments")]
    pub feature_flag_environments: Option<Vec<crate::datadogV2::model::FeatureFlagEnvironment>>,
    /// JSON schema for validation when value_type is JSON.
    #[serde(
        rename = "json_schema",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub json_schema: Option<Option<String>>,
    /// The unique key of the feature flag.
    #[serde(rename = "key")]
    pub key: String,
    /// The ID of the user who last updated the feature flag.
    #[serde(rename = "last_updated_by")]
    pub last_updated_by: Option<uuid::Uuid>,
    /// The name of the feature flag.
    #[serde(rename = "name")]
    pub name: String,
    /// Indicates whether this feature flag requires approval for changes.
    #[serde(rename = "require_approval")]
    pub require_approval: Option<bool>,
    /// Indicates the whether a feature flag is stale or not.
    #[serde(rename = "staleness_status")]
    pub staleness_status: Option<String>,
    /// Tags associated with the feature flag.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The timestamp when the feature flag was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The type of values for the feature flag variants.
    #[serde(rename = "value_type")]
    pub value_type: crate::datadogV2::model::ValueType,
    /// The variants of the feature flag.
    #[serde(rename = "variants")]
    pub variants: Vec<crate::datadogV2::model::Variant>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FeatureFlagAttributes {
    pub fn new(
        description: String,
        key: String,
        name: String,
        value_type: crate::datadogV2::model::ValueType,
        variants: Vec<crate::datadogV2::model::Variant>,
    ) -> FeatureFlagAttributes {
        FeatureFlagAttributes {
            archived_at: None,
            created_at: None,
            created_by: None,
            description,
            distribution_channel: None,
            feature_flag_environments: None,
            json_schema: None,
            key,
            last_updated_by: None,
            name,
            require_approval: None,
            staleness_status: None,
            tags: None,
            updated_at: None,
            value_type,
            variants,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn archived_at(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.archived_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: uuid::Uuid) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn distribution_channel(mut self, value: String) -> Self {
        self.distribution_channel = Some(value);
        self
    }

    pub fn feature_flag_environments(
        mut self,
        value: Vec<crate::datadogV2::model::FeatureFlagEnvironment>,
    ) -> Self {
        self.feature_flag_environments = Some(value);
        self
    }

    pub fn json_schema(mut self, value: Option<String>) -> Self {
        self.json_schema = Some(value);
        self
    }

    pub fn last_updated_by(mut self, value: uuid::Uuid) -> Self {
        self.last_updated_by = Some(value);
        self
    }

    pub fn require_approval(mut self, value: bool) -> Self {
        self.require_approval = Some(value);
        self
    }

    pub fn staleness_status(mut self, value: String) -> Self {
        self.staleness_status = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn updated_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.updated_at = Some(value);
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

impl<'de> Deserialize<'de> for FeatureFlagAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FeatureFlagAttributesVisitor;
        impl<'a> Visitor<'a> for FeatureFlagAttributesVisitor {
            type Value = FeatureFlagAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut archived_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<uuid::Uuid> = None;
                let mut description: Option<String> = None;
                let mut distribution_channel: Option<String> = None;
                let mut feature_flag_environments: Option<
                    Vec<crate::datadogV2::model::FeatureFlagEnvironment>,
                > = None;
                let mut json_schema: Option<Option<String>> = None;
                let mut key: Option<String> = None;
                let mut last_updated_by: Option<uuid::Uuid> = None;
                let mut name: Option<String> = None;
                let mut require_approval: Option<bool> = None;
                let mut staleness_status: Option<String> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut value_type: Option<crate::datadogV2::model::ValueType> = None;
                let mut variants: Option<Vec<crate::datadogV2::model::Variant>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "archived_at" => {
                            archived_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            if v.is_null() {
                                continue;
                            }
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            if v.is_null() {
                                continue;
                            }
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "distribution_channel" => {
                            if v.is_null() {
                                continue;
                            }
                            distribution_channel =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "feature_flag_environments" => {
                            if v.is_null() {
                                continue;
                            }
                            feature_flag_environments =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "json_schema" => {
                            json_schema =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key" => {
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_updated_by" => {
                            if v.is_null() {
                                continue;
                            }
                            last_updated_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "require_approval" => {
                            if v.is_null() {
                                continue;
                            }
                            require_approval =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "staleness_status" => {
                            if v.is_null() {
                                continue;
                            }
                            staleness_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            if v.is_null() {
                                continue;
                            }
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "value_type" => {
                            value_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _value_type) = value_type {
                                match _value_type {
                                    crate::datadogV2::model::ValueType::UnparsedObject(
                                        _value_type,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "variants" => {
                            variants = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let key = key.ok_or_else(|| M::Error::missing_field("key"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let value_type = value_type.ok_or_else(|| M::Error::missing_field("value_type"))?;
                let variants = variants.ok_or_else(|| M::Error::missing_field("variants"))?;

                let content = FeatureFlagAttributes {
                    archived_at,
                    created_at,
                    created_by,
                    description,
                    distribution_channel,
                    feature_flag_environments,
                    json_schema,
                    key,
                    last_updated_by,
                    name,
                    require_approval,
                    staleness_status,
                    tags,
                    updated_at,
                    value_type,
                    variants,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FeatureFlagAttributesVisitor)
    }
}
