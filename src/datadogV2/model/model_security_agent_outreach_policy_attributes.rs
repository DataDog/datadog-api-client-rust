// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of the organization's outreach policy.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityAgentOutreachPolicyAttributes {
    /// Email domains user outreach can be sent to, at most 20. `null` when none are allowed.
    #[serde(
        rename = "allowed_domains",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub allowed_domains: Option<Option<Vec<String>>>,
    /// The organization's outreach mode. `off` disables user outreach. `manual` only sends outreach an analyst
    /// triggers. `automated` auto-sends automated interviews for rules with outreach enabled, to users whose email
    /// domain is allowed.
    #[serde(rename = "mode")]
    pub mode: crate::datadogV2::model::SecurityAgentOutreachMode,
    /// The stored per-rule automation state, including rules that are explicitly disabled. `null` when the
    /// organization has no stored rule state.
    #[serde(rename = "rules", default, with = "::serde_with::rust::double_option")]
    pub rules: Option<Option<Vec<crate::datadogV2::model::SecurityAgentOutreachRule>>>,
    /// Domains the caller could allow next: their own email domain when it is not allowed yet and is not a
    /// consumer mailbox provider. Read-only, never stored. `null` when there is no suggestion.
    #[serde(
        rename = "suggested_domains",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub suggested_domains: Option<Option<Vec<String>>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityAgentOutreachPolicyAttributes {
    pub fn new(
        mode: crate::datadogV2::model::SecurityAgentOutreachMode,
    ) -> SecurityAgentOutreachPolicyAttributes {
        SecurityAgentOutreachPolicyAttributes {
            allowed_domains: None,
            mode,
            rules: None,
            suggested_domains: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn allowed_domains(mut self, value: Option<Vec<String>>) -> Self {
        self.allowed_domains = Some(value);
        self
    }

    pub fn rules(
        mut self,
        value: Option<Vec<crate::datadogV2::model::SecurityAgentOutreachRule>>,
    ) -> Self {
        self.rules = Some(value);
        self
    }

    pub fn suggested_domains(mut self, value: Option<Vec<String>>) -> Self {
        self.suggested_domains = Some(value);
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

impl<'de> Deserialize<'de> for SecurityAgentOutreachPolicyAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityAgentOutreachPolicyAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityAgentOutreachPolicyAttributesVisitor {
            type Value = SecurityAgentOutreachPolicyAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allowed_domains: Option<Option<Vec<String>>> = None;
                let mut mode: Option<crate::datadogV2::model::SecurityAgentOutreachMode> = None;
                let mut rules: Option<
                    Option<Vec<crate::datadogV2::model::SecurityAgentOutreachRule>>,
                > = None;
                let mut suggested_domains: Option<Option<Vec<String>>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allowed_domains" => {
                            allowed_domains =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mode" => {
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mode) = mode {
                                match _mode {
                                    crate::datadogV2::model::SecurityAgentOutreachMode::UnparsedObject(_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "rules" => {
                            rules = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "suggested_domains" => {
                            suggested_domains =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let mode = mode.ok_or_else(|| M::Error::missing_field("mode"))?;

                let content = SecurityAgentOutreachPolicyAttributes {
                    allowed_domains,
                    mode,
                    rules,
                    suggested_domains,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityAgentOutreachPolicyAttributesVisitor)
    }
}
