// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes to change. Attributes left out keep their value; `mention_targets` and `message_customization`
/// replace the stored value when sent.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CostRecommendationNotificationRuleUpdateAttributes {
    /// The Slack channel ID, starting with `C` or `G`.
    #[serde(rename = "channel_id")]
    pub channel_id: Option<String>,
    /// Where the digest is sent. `slack` requires `workspace_id` and `channel_id`; `teams` requires the
    /// `teams_*` attributes. Defaults to `slack`.
    #[serde(rename = "destination_type")]
    pub destination_type:
        Option<crate::datadogV2::model::CostRecommendationNotificationRuleDestinationType>,
    /// Whether the digest is sent.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// How often the digest is sent. `weekly` and `biweekly` use `weekly_execution_day` and `weekly_execution_time`;
    /// `every_30` and `every_90` (days) use `interval_execution_date` and `interval_execution_time`.
    #[serde(rename = "frequency")]
    pub frequency: Option<crate::datadogV2::model::CostRecommendationNotificationRuleFrequency>,
    /// The first date (`YYYY-MM-DD`) of an `every_30` or `every_90` schedule.
    #[serde(rename = "interval_execution_date")]
    pub interval_execution_date: Option<String>,
    /// The time of day (`HH:MM`, in `timezone`) of an `every_30` or `every_90` schedule.
    #[serde(rename = "interval_execution_time")]
    pub interval_execution_time: Option<String>,
    /// Slack member IDs to mention in the digest. Only valid for `slack` rules.
    #[serde(rename = "mention_targets")]
    pub mention_targets: Option<Vec<String>>,
    /// Which sections the digest shows. Sent as a whole: sections left out of a request use their defaults, which
    /// show everything.
    #[serde(rename = "message_customization")]
    pub message_customization:
        Option<crate::datadogV2::model::CostRecommendationNotificationRuleMessageCustomization>,
    /// The name of the rule, unique within the organization.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The recommendation search query that selects what the digest includes. Empty selects every recommendation.
    #[serde(rename = "scope_query")]
    pub scope_query: Option<String>,
    /// The Microsoft Teams channel ID.
    #[serde(rename = "teams_channel_id")]
    pub teams_channel_id: Option<String>,
    /// The Microsoft Teams channel name.
    #[serde(rename = "teams_channel_name")]
    pub teams_channel_name: Option<String>,
    /// The Microsoft Teams team ID.
    #[serde(rename = "teams_team_id")]
    pub teams_team_id: Option<String>,
    /// The Microsoft Teams team name.
    #[serde(rename = "teams_team_name")]
    pub teams_team_name: Option<String>,
    /// The Microsoft Teams tenant ID.
    #[serde(rename = "teams_tenant_id")]
    pub teams_tenant_id: Option<String>,
    /// The Microsoft Teams tenant name.
    #[serde(rename = "teams_tenant_name")]
    pub teams_tenant_name: Option<String>,
    /// The IANA time zone of the schedule.
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    /// The day of the week of a `weekly` or `biweekly` schedule, such as `monday`.
    #[serde(rename = "weekly_execution_day")]
    pub weekly_execution_day: Option<String>,
    /// The time of day (`HH:MM`, in `timezone`) of a `weekly` or `biweekly` schedule.
    #[serde(rename = "weekly_execution_time")]
    pub weekly_execution_time: Option<String>,
    /// The Slack workspace ID, starting with `T`.
    #[serde(rename = "workspace_id")]
    pub workspace_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CostRecommendationNotificationRuleUpdateAttributes {
    pub fn new() -> CostRecommendationNotificationRuleUpdateAttributes {
        CostRecommendationNotificationRuleUpdateAttributes {
            channel_id: None,
            destination_type: None,
            enabled: None,
            frequency: None,
            interval_execution_date: None,
            interval_execution_time: None,
            mention_targets: None,
            message_customization: None,
            name: None,
            scope_query: None,
            teams_channel_id: None,
            teams_channel_name: None,
            teams_team_id: None,
            teams_team_name: None,
            teams_tenant_id: None,
            teams_tenant_name: None,
            timezone: None,
            weekly_execution_day: None,
            weekly_execution_time: None,
            workspace_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn channel_id(mut self, value: String) -> Self {
        self.channel_id = Some(value);
        self
    }

    pub fn destination_type(
        mut self,
        value: crate::datadogV2::model::CostRecommendationNotificationRuleDestinationType,
    ) -> Self {
        self.destination_type = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn frequency(
        mut self,
        value: crate::datadogV2::model::CostRecommendationNotificationRuleFrequency,
    ) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn interval_execution_date(mut self, value: String) -> Self {
        self.interval_execution_date = Some(value);
        self
    }

    pub fn interval_execution_time(mut self, value: String) -> Self {
        self.interval_execution_time = Some(value);
        self
    }

    pub fn mention_targets(mut self, value: Vec<String>) -> Self {
        self.mention_targets = Some(value);
        self
    }

    pub fn message_customization(
        mut self,
        value: crate::datadogV2::model::CostRecommendationNotificationRuleMessageCustomization,
    ) -> Self {
        self.message_customization = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn scope_query(mut self, value: String) -> Self {
        self.scope_query = Some(value);
        self
    }

    pub fn teams_channel_id(mut self, value: String) -> Self {
        self.teams_channel_id = Some(value);
        self
    }

    pub fn teams_channel_name(mut self, value: String) -> Self {
        self.teams_channel_name = Some(value);
        self
    }

    pub fn teams_team_id(mut self, value: String) -> Self {
        self.teams_team_id = Some(value);
        self
    }

    pub fn teams_team_name(mut self, value: String) -> Self {
        self.teams_team_name = Some(value);
        self
    }

    pub fn teams_tenant_id(mut self, value: String) -> Self {
        self.teams_tenant_id = Some(value);
        self
    }

    pub fn teams_tenant_name(mut self, value: String) -> Self {
        self.teams_tenant_name = Some(value);
        self
    }

    pub fn timezone(mut self, value: String) -> Self {
        self.timezone = Some(value);
        self
    }

    pub fn weekly_execution_day(mut self, value: String) -> Self {
        self.weekly_execution_day = Some(value);
        self
    }

    pub fn weekly_execution_time(mut self, value: String) -> Self {
        self.weekly_execution_time = Some(value);
        self
    }

    pub fn workspace_id(mut self, value: String) -> Self {
        self.workspace_id = Some(value);
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

impl Default for CostRecommendationNotificationRuleUpdateAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CostRecommendationNotificationRuleUpdateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CostRecommendationNotificationRuleUpdateAttributesVisitor;
        impl<'a> Visitor<'a> for CostRecommendationNotificationRuleUpdateAttributesVisitor {
            type Value = CostRecommendationNotificationRuleUpdateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut channel_id: Option<String> = None;
                let mut destination_type: Option<
                    crate::datadogV2::model::CostRecommendationNotificationRuleDestinationType,
                > = None;
                let mut enabled: Option<bool> = None;
                let mut frequency: Option<
                    crate::datadogV2::model::CostRecommendationNotificationRuleFrequency,
                > = None;
                let mut interval_execution_date: Option<String> = None;
                let mut interval_execution_time: Option<String> = None;
                let mut mention_targets: Option<Vec<String>> = None;
                let mut message_customization: Option<
                    crate::datadogV2::model::CostRecommendationNotificationRuleMessageCustomization,
                > = None;
                let mut name: Option<String> = None;
                let mut scope_query: Option<String> = None;
                let mut teams_channel_id: Option<String> = None;
                let mut teams_channel_name: Option<String> = None;
                let mut teams_team_id: Option<String> = None;
                let mut teams_team_name: Option<String> = None;
                let mut teams_tenant_id: Option<String> = None;
                let mut teams_tenant_name: Option<String> = None;
                let mut timezone: Option<String> = None;
                let mut weekly_execution_day: Option<String> = None;
                let mut weekly_execution_time: Option<String> = None;
                let mut workspace_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "channel_id" => {
                            if v.is_null() {
                                continue;
                            }
                            channel_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "destination_type" => {
                            if v.is_null() {
                                continue;
                            }
                            destination_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _destination_type) = destination_type {
                                match _destination_type {
                                    crate::datadogV2::model::CostRecommendationNotificationRuleDestinationType::UnparsedObject(_destination_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "frequency" => {
                            if v.is_null() {
                                continue;
                            }
                            frequency = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _frequency) = frequency {
                                match _frequency {
                                    crate::datadogV2::model::CostRecommendationNotificationRuleFrequency::UnparsedObject(_frequency) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "interval_execution_date" => {
                            if v.is_null() {
                                continue;
                            }
                            interval_execution_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "interval_execution_time" => {
                            if v.is_null() {
                                continue;
                            }
                            interval_execution_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mention_targets" => {
                            if v.is_null() {
                                continue;
                            }
                            mention_targets =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message_customization" => {
                            if v.is_null() {
                                continue;
                            }
                            message_customization =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope_query" => {
                            if v.is_null() {
                                continue;
                            }
                            scope_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "teams_channel_id" => {
                            if v.is_null() {
                                continue;
                            }
                            teams_channel_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "teams_channel_name" => {
                            if v.is_null() {
                                continue;
                            }
                            teams_channel_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "teams_team_id" => {
                            if v.is_null() {
                                continue;
                            }
                            teams_team_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "teams_team_name" => {
                            if v.is_null() {
                                continue;
                            }
                            teams_team_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "teams_tenant_id" => {
                            if v.is_null() {
                                continue;
                            }
                            teams_tenant_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "teams_tenant_name" => {
                            if v.is_null() {
                                continue;
                            }
                            teams_tenant_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timezone" => {
                            if v.is_null() {
                                continue;
                            }
                            timezone = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "weekly_execution_day" => {
                            if v.is_null() {
                                continue;
                            }
                            weekly_execution_day =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "weekly_execution_time" => {
                            if v.is_null() {
                                continue;
                            }
                            weekly_execution_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "workspace_id" => {
                            if v.is_null() {
                                continue;
                            }
                            workspace_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = CostRecommendationNotificationRuleUpdateAttributes {
                    channel_id,
                    destination_type,
                    enabled,
                    frequency,
                    interval_execution_date,
                    interval_execution_time,
                    mention_targets,
                    message_customization,
                    name,
                    scope_query,
                    teams_channel_id,
                    teams_channel_name,
                    teams_team_id,
                    teams_team_name,
                    teams_tenant_id,
                    teams_tenant_name,
                    timezone,
                    weekly_execution_day,
                    weekly_execution_time,
                    workspace_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CostRecommendationNotificationRuleUpdateAttributesVisitor)
    }
}
