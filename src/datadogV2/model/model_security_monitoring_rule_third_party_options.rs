// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Options on third party detection method.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringRuleThirdPartyOptions {
    /// Notification targets for the logs that do not correspond to any of the cases.
    #[serde(rename = "defaultNotifications")]
    pub default_notifications: Option<Vec<String>>,
    /// Severity of the Security Signal.
    #[serde(rename = "defaultStatus")]
    pub default_status: Option<crate::datadogV2::model::SecurityMonitoringRuleSeverity>,
    /// Queries to be combined with third party case queries. Each of them can have different group by fields, to aggregate differently based on the type of alert.
    #[serde(rename = "rootQueries")]
    pub root_queries: Option<Vec<crate::datadogV2::model::SecurityMonitoringThirdPartyRootQuery>>,
    /// A template for the signal title; if omitted, the title is generated based on the case name.
    #[serde(rename = "signalTitleTemplate")]
    pub signal_title_template: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringRuleThirdPartyOptions {
    pub fn new() -> SecurityMonitoringRuleThirdPartyOptions {
        SecurityMonitoringRuleThirdPartyOptions {
            default_notifications: None,
            default_status: None,
            root_queries: None,
            signal_title_template: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn default_notifications(mut self, value: Vec<String>) -> Self {
        self.default_notifications = Some(value);
        self
    }

    pub fn default_status(
        mut self,
        value: crate::datadogV2::model::SecurityMonitoringRuleSeverity,
    ) -> Self {
        self.default_status = Some(value);
        self
    }

    pub fn root_queries(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringThirdPartyRootQuery>,
    ) -> Self {
        self.root_queries = Some(value);
        self
    }

    pub fn signal_title_template(mut self, value: String) -> Self {
        self.signal_title_template = Some(value);
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

impl Default for SecurityMonitoringRuleThirdPartyOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for SecurityMonitoringRuleThirdPartyOptions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringRuleThirdPartyOptionsVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringRuleThirdPartyOptionsVisitor {
            type Value = SecurityMonitoringRuleThirdPartyOptions;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut default_notifications: Option<Vec<String>> = None;
                let mut default_status: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleSeverity,
                > = None;
                let mut root_queries: Option<
                    Vec<crate::datadogV2::model::SecurityMonitoringThirdPartyRootQuery>,
                > = None;
                let mut signal_title_template: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "defaultNotifications" => {
                            if v.is_null() {
                                continue;
                            }
                            default_notifications =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "defaultStatus" => {
                            if v.is_null() {
                                continue;
                            }
                            default_status =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _default_status) = default_status {
                                match _default_status {
                                    crate::datadogV2::model::SecurityMonitoringRuleSeverity::UnparsedObject(_default_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "rootQueries" => {
                            if v.is_null() {
                                continue;
                            }
                            root_queries =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "signalTitleTemplate" => {
                            if v.is_null() {
                                continue;
                            }
                            signal_title_template =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = SecurityMonitoringRuleThirdPartyOptions {
                    default_notifications,
                    default_status,
                    root_queries,
                    signal_title_template,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringRuleThirdPartyOptionsVisitor)
    }
}
