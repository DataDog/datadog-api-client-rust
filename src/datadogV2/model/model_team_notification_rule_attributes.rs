// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Team notification rule attributes
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamNotificationRuleAttributes {
    /// Email notification settings for the team
    #[serde(rename = "email")]
    pub email: Option<crate::datadogV2::model::TeamNotificationRuleAttributesEmail>,
    /// MS Teams notification settings for the team
    #[serde(rename = "ms_teams")]
    pub ms_teams: Option<crate::datadogV2::model::TeamNotificationRuleAttributesMsTeams>,
    /// PagerDuty notification settings for the team
    #[serde(rename = "pagerduty")]
    pub pagerduty: Option<crate::datadogV2::model::TeamNotificationRuleAttributesPagerduty>,
    /// Slack notification settings for the team
    #[serde(rename = "slack")]
    pub slack: Option<crate::datadogV2::model::TeamNotificationRuleAttributesSlack>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TeamNotificationRuleAttributes {
    pub fn new() -> TeamNotificationRuleAttributes {
        TeamNotificationRuleAttributes {
            email: None,
            ms_teams: None,
            pagerduty: None,
            slack: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn email(
        mut self,
        value: crate::datadogV2::model::TeamNotificationRuleAttributesEmail,
    ) -> Self {
        self.email = Some(value);
        self
    }

    pub fn ms_teams(
        mut self,
        value: crate::datadogV2::model::TeamNotificationRuleAttributesMsTeams,
    ) -> Self {
        self.ms_teams = Some(value);
        self
    }

    pub fn pagerduty(
        mut self,
        value: crate::datadogV2::model::TeamNotificationRuleAttributesPagerduty,
    ) -> Self {
        self.pagerduty = Some(value);
        self
    }

    pub fn slack(
        mut self,
        value: crate::datadogV2::model::TeamNotificationRuleAttributesSlack,
    ) -> Self {
        self.slack = Some(value);
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

impl Default for TeamNotificationRuleAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TeamNotificationRuleAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TeamNotificationRuleAttributesVisitor;
        impl<'a> Visitor<'a> for TeamNotificationRuleAttributesVisitor {
            type Value = TeamNotificationRuleAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut email: Option<
                    crate::datadogV2::model::TeamNotificationRuleAttributesEmail,
                > = None;
                let mut ms_teams: Option<
                    crate::datadogV2::model::TeamNotificationRuleAttributesMsTeams,
                > = None;
                let mut pagerduty: Option<
                    crate::datadogV2::model::TeamNotificationRuleAttributesPagerduty,
                > = None;
                let mut slack: Option<
                    crate::datadogV2::model::TeamNotificationRuleAttributesSlack,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "email" => {
                            if v.is_null() {
                                continue;
                            }
                            email = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ms_teams" => {
                            if v.is_null() {
                                continue;
                            }
                            ms_teams = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "pagerduty" => {
                            if v.is_null() {
                                continue;
                            }
                            pagerduty = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "slack" => {
                            if v.is_null() {
                                continue;
                            }
                            slack = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = TeamNotificationRuleAttributes {
                    email,
                    ms_teams,
                    pagerduty,
                    slack,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TeamNotificationRuleAttributesVisitor)
    }
}
