// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a suggested action for a security signal. The available fields depend on the action type.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringSignalSuggestedActionAttributes {
    /// The name of the investigation log query.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The log query filter for the investigation.
    #[serde(rename = "query_filter")]
    pub query_filter: Option<String>,
    /// Template variables applied to the investigation log query, mapping attribute paths to values extracted from the signal.
    #[serde(rename = "template_variables")]
    pub template_variables: Option<std::collections::BTreeMap<String, Vec<String>>>,
    /// The title of the recommended blog post.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// The URL of the suggested action.
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringSignalSuggestedActionAttributes {
    pub fn new() -> SecurityMonitoringSignalSuggestedActionAttributes {
        SecurityMonitoringSignalSuggestedActionAttributes {
            name: None,
            query_filter: None,
            template_variables: None,
            title: None,
            url: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn query_filter(mut self, value: String) -> Self {
        self.query_filter = Some(value);
        self
    }

    pub fn template_variables(
        mut self,
        value: std::collections::BTreeMap<String, Vec<String>>,
    ) -> Self {
        self.template_variables = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    pub fn url(mut self, value: String) -> Self {
        self.url = Some(value);
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

impl Default for SecurityMonitoringSignalSuggestedActionAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringSignalSuggestedActionAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringSignalSuggestedActionAttributesVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringSignalSuggestedActionAttributesVisitor {
            type Value = SecurityMonitoringSignalSuggestedActionAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut name: Option<String> = None;
                let mut query_filter: Option<String> = None;
                let mut template_variables: Option<
                    std::collections::BTreeMap<String, Vec<String>>,
                > = None;
                let mut title: Option<String> = None;
                let mut url: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_filter" => {
                            if v.is_null() {
                                continue;
                            }
                            query_filter =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "template_variables" => {
                            if v.is_null() {
                                continue;
                            }
                            template_variables =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "url" => {
                            if v.is_null() {
                                continue;
                            }
                            url = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SecurityMonitoringSignalSuggestedActionAttributes {
                    name,
                    query_filter,
                    template_variables,
                    title,
                    url,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringSignalSuggestedActionAttributesVisitor)
    }
}
