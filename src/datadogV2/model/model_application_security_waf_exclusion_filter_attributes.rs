// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes describing a WAF exclusion filter.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApplicationSecurityWafExclusionFilterAttributes {
    /// A description for the exclusion filter.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Indicates whether the exclusion filter is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The event query matched by the legacy exclusion filter. Cannot be created nor updated.
    #[serde(rename = "event_query")]
    pub event_query: Option<String>,
    /// The client IP addresses matched by the exclusion filter (CIDR notation is supported).
    #[serde(rename = "ip_list")]
    pub ip_list: Option<Vec<String>>,
    /// Extra information about the exclusion filter.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV2::model::ApplicationSecurityWafExclusionFilterMetadata>,
    /// The action taken when the exclusion filter matches. When set to `monitor`, security traces are emitted but the requests are not blocked. By default, security traces are not emitted and the requests are not blocked.
    #[serde(rename = "on_match")]
    pub on_match: Option<crate::datadogV2::model::ApplicationSecurityWafExclusionFilterOnMatch>,
    /// A list of parameters matched by the exclusion filter in the HTTP query string and HTTP request body. Nested parameters can be matched by joining fields with a dot character.
    #[serde(rename = "parameters")]
    pub parameters: Option<Vec<String>>,
    /// The HTTP path glob expression matched by the exclusion filter.
    #[serde(rename = "path_glob")]
    pub path_glob: Option<String>,
    /// The WAF rules targeted by the exclusion filter.
    #[serde(rename = "rules_target")]
    pub rules_target:
        Option<Vec<crate::datadogV2::model::ApplicationSecurityWafExclusionFilterRulesTarget>>,
    /// The services where the exclusion filter is deployed.
    #[serde(rename = "scope")]
    pub scope: Option<Vec<crate::datadogV2::model::ApplicationSecurityWafExclusionFilterScope>>,
    /// Generated event search query for traces matching the exclusion filter.
    #[serde(rename = "search_query")]
    pub search_query: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApplicationSecurityWafExclusionFilterAttributes {
    pub fn new() -> ApplicationSecurityWafExclusionFilterAttributes {
        ApplicationSecurityWafExclusionFilterAttributes {
            description: None,
            enabled: None,
            event_query: None,
            ip_list: None,
            metadata: None,
            on_match: None,
            parameters: None,
            path_glob: None,
            rules_target: None,
            scope: None,
            search_query: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn event_query(mut self, value: String) -> Self {
        self.event_query = Some(value);
        self
    }

    pub fn ip_list(mut self, value: Vec<String>) -> Self {
        self.ip_list = Some(value);
        self
    }

    pub fn metadata(
        mut self,
        value: crate::datadogV2::model::ApplicationSecurityWafExclusionFilterMetadata,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn on_match(
        mut self,
        value: crate::datadogV2::model::ApplicationSecurityWafExclusionFilterOnMatch,
    ) -> Self {
        self.on_match = Some(value);
        self
    }

    pub fn parameters(mut self, value: Vec<String>) -> Self {
        self.parameters = Some(value);
        self
    }

    pub fn path_glob(mut self, value: String) -> Self {
        self.path_glob = Some(value);
        self
    }

    pub fn rules_target(
        mut self,
        value: Vec<crate::datadogV2::model::ApplicationSecurityWafExclusionFilterRulesTarget>,
    ) -> Self {
        self.rules_target = Some(value);
        self
    }

    pub fn scope(
        mut self,
        value: Vec<crate::datadogV2::model::ApplicationSecurityWafExclusionFilterScope>,
    ) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn search_query(mut self, value: String) -> Self {
        self.search_query = Some(value);
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

impl Default for ApplicationSecurityWafExclusionFilterAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ApplicationSecurityWafExclusionFilterAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApplicationSecurityWafExclusionFilterAttributesVisitor;
        impl<'a> Visitor<'a> for ApplicationSecurityWafExclusionFilterAttributesVisitor {
            type Value = ApplicationSecurityWafExclusionFilterAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut enabled: Option<bool> = None;
                let mut event_query: Option<String> = None;
                let mut ip_list: Option<Vec<String>> = None;
                let mut metadata: Option<
                    crate::datadogV2::model::ApplicationSecurityWafExclusionFilterMetadata,
                > = None;
                let mut on_match: Option<
                    crate::datadogV2::model::ApplicationSecurityWafExclusionFilterOnMatch,
                > = None;
                let mut parameters: Option<Vec<String>> = None;
                let mut path_glob: Option<String> = None;
                let mut rules_target: Option<
                    Vec<crate::datadogV2::model::ApplicationSecurityWafExclusionFilterRulesTarget>,
                > = None;
                let mut scope: Option<
                    Vec<crate::datadogV2::model::ApplicationSecurityWafExclusionFilterScope>,
                > = None;
                let mut search_query: Option<String> = None;
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
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event_query" => {
                            if v.is_null() {
                                continue;
                            }
                            event_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ip_list" => {
                            if v.is_null() {
                                continue;
                            }
                            ip_list = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metadata" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "on_match" => {
                            if v.is_null() {
                                continue;
                            }
                            on_match = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _on_match) = on_match {
                                match _on_match {
                                    crate::datadogV2::model::ApplicationSecurityWafExclusionFilterOnMatch::UnparsedObject(_on_match) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "parameters" => {
                            if v.is_null() {
                                continue;
                            }
                            parameters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "path_glob" => {
                            if v.is_null() {
                                continue;
                            }
                            path_glob = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rules_target" => {
                            if v.is_null() {
                                continue;
                            }
                            rules_target =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search_query" => {
                            if v.is_null() {
                                continue;
                            }
                            search_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ApplicationSecurityWafExclusionFilterAttributes {
                    description,
                    enabled,
                    event_query,
                    ip_list,
                    metadata,
                    on_match,
                    parameters,
                    path_glob,
                    rules_target,
                    scope,
                    search_query,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApplicationSecurityWafExclusionFilterAttributesVisitor)
    }
}
