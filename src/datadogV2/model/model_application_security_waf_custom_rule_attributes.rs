// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A WAF custom rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApplicationSecurityWafCustomRuleAttributes {
    /// The definition of `ApplicationSecurityWafCustomRuleAction` object.
    #[serde(rename = "action")]
    pub action: Option<crate::datadogV2::model::ApplicationSecurityWafCustomRuleAction>,
    /// Indicates whether the WAF custom rule will block the request.
    #[serde(rename = "blocking")]
    pub blocking: bool,
    /// Conditions for which the WAF Custom Rule will triggers, all conditions needs to match in order for the WAF
    /// rule to trigger.
    #[serde(rename = "conditions")]
    pub conditions: Vec<crate::datadogV2::model::ApplicationSecurityWafCustomRuleCondition>,
    /// Indicates whether the WAF custom rule is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// Metadata associated with the WAF Custom Rule.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV2::model::ApplicationSecurityWafCustomRuleMetadata>,
    /// The Name of the WAF custom rule.
    #[serde(rename = "name")]
    pub name: String,
    /// The path glob for the WAF custom rule.
    #[serde(rename = "path_glob")]
    pub path_glob: Option<String>,
    /// The scope of the WAF custom rule.
    #[serde(rename = "scope")]
    pub scope: Option<Vec<crate::datadogV2::model::ApplicationSecurityWafCustomRuleScope>>,
    /// Tags associated with the WAF Custom Rule. The concatenation of category and type will form the security
    /// activity field associated with the traces.
    #[serde(rename = "tags")]
    pub tags: crate::datadogV2::model::ApplicationSecurityWafCustomRuleTags,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApplicationSecurityWafCustomRuleAttributes {
    pub fn new(
        blocking: bool,
        conditions: Vec<crate::datadogV2::model::ApplicationSecurityWafCustomRuleCondition>,
        enabled: bool,
        name: String,
        tags: crate::datadogV2::model::ApplicationSecurityWafCustomRuleTags,
    ) -> ApplicationSecurityWafCustomRuleAttributes {
        ApplicationSecurityWafCustomRuleAttributes {
            action: None,
            blocking,
            conditions,
            enabled,
            metadata: None,
            name,
            path_glob: None,
            scope: None,
            tags,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn action(
        mut self,
        value: crate::datadogV2::model::ApplicationSecurityWafCustomRuleAction,
    ) -> Self {
        self.action = Some(value);
        self
    }

    pub fn metadata(
        mut self,
        value: crate::datadogV2::model::ApplicationSecurityWafCustomRuleMetadata,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn path_glob(mut self, value: String) -> Self {
        self.path_glob = Some(value);
        self
    }

    pub fn scope(
        mut self,
        value: Vec<crate::datadogV2::model::ApplicationSecurityWafCustomRuleScope>,
    ) -> Self {
        self.scope = Some(value);
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

impl<'de> Deserialize<'de> for ApplicationSecurityWafCustomRuleAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApplicationSecurityWafCustomRuleAttributesVisitor;
        impl<'a> Visitor<'a> for ApplicationSecurityWafCustomRuleAttributesVisitor {
            type Value = ApplicationSecurityWafCustomRuleAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action: Option<
                    crate::datadogV2::model::ApplicationSecurityWafCustomRuleAction,
                > = None;
                let mut blocking: Option<bool> = None;
                let mut conditions: Option<
                    Vec<crate::datadogV2::model::ApplicationSecurityWafCustomRuleCondition>,
                > = None;
                let mut enabled: Option<bool> = None;
                let mut metadata: Option<
                    crate::datadogV2::model::ApplicationSecurityWafCustomRuleMetadata,
                > = None;
                let mut name: Option<String> = None;
                let mut path_glob: Option<String> = None;
                let mut scope: Option<
                    Vec<crate::datadogV2::model::ApplicationSecurityWafCustomRuleScope>,
                > = None;
                let mut tags: Option<
                    crate::datadogV2::model::ApplicationSecurityWafCustomRuleTags,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "action" => {
                            if v.is_null() {
                                continue;
                            }
                            action = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "blocking" => {
                            blocking = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "conditions" => {
                            conditions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "path_glob" => {
                            if v.is_null() {
                                continue;
                            }
                            path_glob = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let blocking = blocking.ok_or_else(|| M::Error::missing_field("blocking"))?;
                let conditions = conditions.ok_or_else(|| M::Error::missing_field("conditions"))?;
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let tags = tags.ok_or_else(|| M::Error::missing_field("tags"))?;

                let content = ApplicationSecurityWafCustomRuleAttributes {
                    action,
                    blocking,
                    conditions,
                    enabled,
                    metadata,
                    name,
                    path_glob,
                    scope,
                    tags,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApplicationSecurityWafCustomRuleAttributesVisitor)
    }
}
