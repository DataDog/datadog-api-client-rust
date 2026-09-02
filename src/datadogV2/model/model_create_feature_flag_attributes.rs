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
    pub description: String,
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
    /// Query used to determine which change events on this feature flag trigger notifications
    /// to `rule_targets`. Uses Datadog log search syntax (`AND`, `OR`, `NOT`, parentheses) to
    /// match against the `notification_type` facet.
    ///
    /// Supported `notification_type` values for a feature flag are: `flag_enabled_disabled`,
    /// `flag_archived`, `flag_approval_required`, `rollout_started`, `rollout_scheduled`,
    /// `rollout_step_started`, `rollout_paused_guardrail`, `rollout_paused_user`,
    /// `rollout_aborted_guardrail`, `rollout_aborted_user`, `targeting_rule_created`,
    /// `targeting_rule_updated`, `targeting_rule_updated_via_filter`, and
    /// `targeting_rule_deleted`.
    #[serde(
        rename = "notification_rule_query",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub notification_rule_query: Option<Option<String>>,
    /// Targets to notify about changes to this feature flag that match `notification_rule_query`.
    #[serde(rename = "rule_targets")]
    pub rule_targets: Option<Vec<crate::datadogV2::model::NotificationRuleTarget>>,
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
        description: String,
        key: String,
        name: String,
        value_type: crate::datadogV2::model::ValueType,
        variants: Vec<crate::datadogV2::model::CreateVariant>,
    ) -> CreateFeatureFlagAttributes {
        CreateFeatureFlagAttributes {
            default_variant_key: None,
            description,
            json_schema: None,
            key,
            name,
            notification_rule_query: None,
            rule_targets: None,
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

    pub fn json_schema(mut self, value: Option<String>) -> Self {
        self.json_schema = Some(value);
        self
    }

    pub fn notification_rule_query(mut self, value: Option<String>) -> Self {
        self.notification_rule_query = Some(value);
        self
    }

    pub fn rule_targets(
        mut self,
        value: Vec<crate::datadogV2::model::NotificationRuleTarget>,
    ) -> Self {
        self.rule_targets = Some(value);
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
                let mut json_schema: Option<Option<String>> = None;
                let mut key: Option<String> = None;
                let mut name: Option<String> = None;
                let mut notification_rule_query: Option<Option<String>> = None;
                let mut rule_targets: Option<Vec<crate::datadogV2::model::NotificationRuleTarget>> =
                    None;
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
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "notification_rule_query" => {
                            notification_rule_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule_targets" => {
                            if v.is_null() {
                                continue;
                            }
                            rule_targets =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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

                let content = CreateFeatureFlagAttributes {
                    default_variant_key,
                    description,
                    json_schema,
                    key,
                    name,
                    notification_rule_query,
                    rule_targets,
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
