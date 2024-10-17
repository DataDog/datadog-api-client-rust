// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The attributes of the Application Security exclusion filter.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ApplicationSecurityExclusionFilterListAttributes {
    /// A description for the exclusion filter.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Indicates whether the exclusion filter is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The IPs list for the exclusion filter.
    #[serde(rename = "ip_list")]
    pub ip_list: Option<Vec<String>>,
    /// Metadata about the exclusion filter.
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::datadogV2::model::ApplicationSecurityExclusionFilterMetadata>,
    /// A list of parameters for the exclusion filter.
    #[serde(rename = "parameters")]
    pub parameters: Option<Vec<String>>,
    /// The path glob for the exclusion filter.
    #[serde(rename = "path_glob")]
    pub path_glob: Option<String>,
    /// A list of rules targeted by the exclusion filter.
    #[serde(rename = "rules_target")]
    pub rules_target:
        Option<Vec<crate::datadogV2::model::ApplicationSecurityExclusionFilterListRulesTarget>>,
    /// The scope of the exclusion filter.
    #[serde(rename = "scope")]
    pub scope: Option<Vec<crate::datadogV2::model::ApplicationSecurityExclusionFilterScope>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ApplicationSecurityExclusionFilterListAttributes {
    pub fn new() -> ApplicationSecurityExclusionFilterListAttributes {
        ApplicationSecurityExclusionFilterListAttributes {
            description: None,
            enabled: None,
            ip_list: None,
            metadata: None,
            parameters: None,
            path_glob: None,
            rules_target: None,
            scope: None,
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

    pub fn ip_list(mut self, value: Vec<String>) -> Self {
        self.ip_list = Some(value);
        self
    }

    pub fn metadata(
        mut self,
        value: crate::datadogV2::model::ApplicationSecurityExclusionFilterMetadata,
    ) -> Self {
        self.metadata = Some(value);
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
        value: Vec<crate::datadogV2::model::ApplicationSecurityExclusionFilterListRulesTarget>,
    ) -> Self {
        self.rules_target = Some(value);
        self
    }

    pub fn scope(
        mut self,
        value: Vec<crate::datadogV2::model::ApplicationSecurityExclusionFilterScope>,
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

impl Default for ApplicationSecurityExclusionFilterListAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ApplicationSecurityExclusionFilterListAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ApplicationSecurityExclusionFilterListAttributesVisitor;
        impl<'a> Visitor<'a> for ApplicationSecurityExclusionFilterListAttributesVisitor {
            type Value = ApplicationSecurityExclusionFilterListAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut description: Option<String> = None;
                let mut enabled: Option<bool> = None;
                let mut ip_list: Option<Vec<String>> = None;
                let mut metadata: Option<
                    crate::datadogV2::model::ApplicationSecurityExclusionFilterMetadata,
                > = None;
                let mut parameters: Option<Vec<String>> = None;
                let mut path_glob: Option<String> = None;
                let mut rules_target: Option<
                    Vec<crate::datadogV2::model::ApplicationSecurityExclusionFilterListRulesTarget>,
                > = None;
                let mut scope: Option<
                    Vec<crate::datadogV2::model::ApplicationSecurityExclusionFilterScope>,
                > = None;
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = ApplicationSecurityExclusionFilterListAttributes {
                    description,
                    enabled,
                    ip_list,
                    metadata,
                    parameters,
                    path_glob,
                    rules_target,
                    scope,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ApplicationSecurityExclusionFilterListAttributesVisitor)
    }
}
