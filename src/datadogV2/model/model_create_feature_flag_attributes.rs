// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating a new feature flag.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CreateFeatureFlagAttributes {
    /// The key of the default variant.
    #[serde(
        rename = "default_variant_key",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub default_variant_key: Option<Option<String>>,
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
    /// The unique key of the feature flag.
    #[serde(rename = "key")]
    pub key: String,
    /// The name of the feature flag.
    #[serde(rename = "name")]
    pub name: String,
    /// Indicates whether this feature flag requires approval for changes.
    #[serde(rename = "require_approval")]
    pub require_approval: Option<bool>,
    /// The staleness status for the feature flag at creation.
    #[serde(rename = "staleness_status")]
    pub staleness_status: Option<crate::datadogV2::model::CreateFeatureFlagStalenessStatus>,
    /// Tags associated with the feature flag.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The type of values for the feature flag variants.
    #[serde(rename = "value_type")]
    pub value_type: crate::datadogV2::model::ValueType,
    /// The variants of the feature flag.
    #[serde(rename = "variants")]
    pub variants: Vec<crate::datadogV2::model::CreateVariant>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CreateFeatureFlagAttributes {
    pub fn new(
        key: String,
        name: String,
        value_type: crate::datadogV2::model::ValueType,
        variants: Vec<crate::datadogV2::model::CreateVariant>,
    ) -> CreateFeatureFlagAttributes {
        CreateFeatureFlagAttributes {
            default_variant_key: None,
            description: None,
            distribution_channel: None,
            json_schema: None,
            key,
            name,
            require_approval: None,
            staleness_status: None,
            tags: None,
            value_type,
            variants,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn default_variant_key(mut self, value: Option<String>) -> Self {
        self.default_variant_key = Some(value);
        self
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

    pub fn require_approval(mut self, value: bool) -> Self {
        self.require_approval = Some(value);
        self
    }

    pub fn staleness_status(
        mut self,
        value: crate::datadogV2::model::CreateFeatureFlagStalenessStatus,
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

impl<'de> Deserialize<'de> for CreateFeatureFlagAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CreateFeatureFlagAttributesVisitor;
        impl<'a> Visitor<'a> for CreateFeatureFlagAttributesVisitor {
            type Value = CreateFeatureFlagAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut default_variant_key: Option<Option<String>> = None;
                let mut description: Option<String> = None;
                let mut distribution_channel: Option<
                    crate::datadogV2::model::FeatureFlagDistributionChannel,
                > = None;
                let mut json_schema: Option<Option<String>> = None;
                let mut key: Option<String> = None;
                let mut name: Option<String> = None;
                let mut require_approval: Option<bool> = None;
                let mut staleness_status: Option<
                    crate::datadogV2::model::CreateFeatureFlagStalenessStatus,
                > = None;
                let mut tags: Option<Vec<String>> = None;
                let mut value_type: Option<crate::datadogV2::model::ValueType> = None;
                let mut variants: Option<Vec<crate::datadogV2::model::CreateVariant>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "default_variant_key" => {
                            default_variant_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                        "key" => {
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                            if let Some(ref _staleness_status) = staleness_status {
                                match _staleness_status {
                                    crate::datadogV2::model::CreateFeatureFlagStalenessStatus::UnparsedObject(_staleness_status) => {
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
                let key = key.ok_or_else(|| M::Error::missing_field("key"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let value_type = value_type.ok_or_else(|| M::Error::missing_field("value_type"))?;
                let variants = variants.ok_or_else(|| M::Error::missing_field("variants"))?;

                let content = CreateFeatureFlagAttributes {
                    default_variant_key,
                    description,
                    distribution_channel,
                    json_schema,
                    key,
                    name,
                    require_approval,
                    staleness_status,
                    tags,
                    value_type,
                    variants,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CreateFeatureFlagAttributesVisitor)
    }
}
