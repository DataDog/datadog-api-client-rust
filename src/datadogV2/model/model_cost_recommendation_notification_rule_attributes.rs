// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a recommendation notification rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CostRecommendationNotificationRuleAttributes {
    /// The Slack channel ID. Empty for `teams` rules.
    #[serde(rename = "channel_id")]
    pub channel_id: String,
    /// When the rule was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The UUID of the user who created the rule.
    #[serde(rename = "created_by")]
    pub created_by: String,
    /// Where the digest is sent. `slack` requires `workspace_id` and `channel_id`; `teams` requires the
    /// `teams_*` attributes. Defaults to `slack`.
    #[serde(rename = "destination_type")]
    pub destination_type:
        crate::datadogV2::model::CostRecommendationNotificationRuleDestinationType,
    /// Whether the digest is sent.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// How often the digest is sent. `weekly` and `biweekly` use `weekly_execution_day` and `weekly_execution_time`;
    /// `every_30` and `every_90` (days) use `interval_execution_date` and `interval_execution_time`.
    #[serde(rename = "frequency")]
    pub frequency: crate::datadogV2::model::CostRecommendationNotificationRuleFrequency,
    /// The first date (`YYYY-MM-DD`) of an `every_30` or `every_90` schedule.
    #[serialize_always]
    #[serde(rename = "interval_execution_date")]
    pub interval_execution_date: Option<String>,
    /// The time of day (`HH:MM`, in `timezone`) of an `every_30` or `every_90` schedule.
    #[serialize_always]
    #[serde(rename = "interval_execution_time")]
    pub interval_execution_time: Option<String>,
    /// Slack member IDs to mention in the digest. Only valid for `slack` rules.
    #[serde(rename = "mention_targets")]
    pub mention_targets: Vec<String>,
    /// Which sections the digest shows. Sent as a whole: sections left out of a request use their defaults, which
    /// show everything.
    #[serde(rename = "message_customization")]
    pub message_customization:
        crate::datadogV2::model::CostRecommendationNotificationRuleMessageCustomization,
    /// When the rule was last modified.
    #[serde(rename = "modified_at")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
    /// The UUID of the user who last modified the rule.
    #[serde(rename = "modified_by")]
    pub modified_by: String,
    /// The name of the rule, unique within the organization.
    #[serde(rename = "name")]
    pub name: String,
    /// The ID of the organization that owns the rule.
    #[serde(rename = "org_id")]
    pub org_id: i64,
    /// The recommendation search query that selects what the digest includes. Empty selects every recommendation.
    #[serde(rename = "scope_query")]
    pub scope_query: String,
    /// The Microsoft Teams channel ID. Empty for `slack` rules.
    #[serde(rename = "teams_channel_id")]
    pub teams_channel_id: String,
    /// The Microsoft Teams channel name. Empty for `slack` rules.
    #[serde(rename = "teams_channel_name")]
    pub teams_channel_name: String,
    /// The Microsoft Teams team ID. Empty for `slack` rules.
    #[serde(rename = "teams_team_id")]
    pub teams_team_id: String,
    /// The Microsoft Teams team name. Empty for `slack` rules.
    #[serde(rename = "teams_team_name")]
    pub teams_team_name: String,
    /// The Microsoft Teams tenant ID. Empty for `slack` rules.
    #[serde(rename = "teams_tenant_id")]
    pub teams_tenant_id: String,
    /// The Microsoft Teams tenant name. Empty for `slack` rules.
    #[serde(rename = "teams_tenant_name")]
    pub teams_tenant_name: String,
    /// The IANA time zone of the schedule.
    #[serde(rename = "timezone")]
    pub timezone: String,
    /// The day of the week of a `weekly` or `biweekly` schedule, such as `monday`.
    #[serialize_always]
    #[serde(rename = "weekly_execution_day")]
    pub weekly_execution_day: Option<String>,
    /// The time of day (`HH:MM`, in `timezone`) of a `weekly` or `biweekly` schedule.
    #[serialize_always]
    #[serde(rename = "weekly_execution_time")]
    pub weekly_execution_time: Option<String>,
    /// The Slack workspace ID. Empty for `teams` rules.
    #[serde(rename = "workspace_id")]
    pub workspace_id: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CostRecommendationNotificationRuleAttributes {
    pub fn new(
        channel_id: String,
        created_at: chrono::DateTime<chrono::Utc>,
        created_by: String,
        destination_type: crate::datadogV2::model::CostRecommendationNotificationRuleDestinationType,
        enabled: bool,
        frequency: crate::datadogV2::model::CostRecommendationNotificationRuleFrequency,
        interval_execution_date: Option<String>,
        interval_execution_time: Option<String>,
        mention_targets: Vec<String>,
        message_customization: crate::datadogV2::model::CostRecommendationNotificationRuleMessageCustomization,
        modified_at: chrono::DateTime<chrono::Utc>,
        modified_by: String,
        name: String,
        org_id: i64,
        scope_query: String,
        teams_channel_id: String,
        teams_channel_name: String,
        teams_team_id: String,
        teams_team_name: String,
        teams_tenant_id: String,
        teams_tenant_name: String,
        timezone: String,
        weekly_execution_day: Option<String>,
        weekly_execution_time: Option<String>,
        workspace_id: String,
    ) -> CostRecommendationNotificationRuleAttributes {
        CostRecommendationNotificationRuleAttributes {
            channel_id,
            created_at,
            created_by,
            destination_type,
            enabled,
            frequency,
            interval_execution_date,
            interval_execution_time,
            mention_targets,
            message_customization,
            modified_at,
            modified_by,
            name,
            org_id,
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
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for CostRecommendationNotificationRuleAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CostRecommendationNotificationRuleAttributesVisitor;
        impl<'a> Visitor<'a> for CostRecommendationNotificationRuleAttributesVisitor {
            type Value = CostRecommendationNotificationRuleAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut channel_id: Option<String> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut created_by: Option<String> = None;
                let mut destination_type: Option<
                    crate::datadogV2::model::CostRecommendationNotificationRuleDestinationType,
                > = None;
                let mut enabled: Option<bool> = None;
                let mut frequency: Option<
                    crate::datadogV2::model::CostRecommendationNotificationRuleFrequency,
                > = None;
                let mut interval_execution_date: Option<Option<String>> = None;
                let mut interval_execution_time: Option<Option<String>> = None;
                let mut mention_targets: Option<Vec<String>> = None;
                let mut message_customization: Option<
                    crate::datadogV2::model::CostRecommendationNotificationRuleMessageCustomization,
                > = None;
                let mut modified_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut modified_by: Option<String> = None;
                let mut name: Option<String> = None;
                let mut org_id: Option<i64> = None;
                let mut scope_query: Option<String> = None;
                let mut teams_channel_id: Option<String> = None;
                let mut teams_channel_name: Option<String> = None;
                let mut teams_team_id: Option<String> = None;
                let mut teams_team_name: Option<String> = None;
                let mut teams_tenant_id: Option<String> = None;
                let mut teams_tenant_name: Option<String> = None;
                let mut timezone: Option<String> = None;
                let mut weekly_execution_day: Option<Option<String>> = None;
                let mut weekly_execution_time: Option<Option<String>> = None;
                let mut workspace_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "channel_id" => {
                            channel_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_by" => {
                            created_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "destination_type" => {
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
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "frequency" => {
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
                            interval_execution_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "interval_execution_time" => {
                            interval_execution_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mention_targets" => {
                            mention_targets =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message_customization" => {
                            message_customization =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_at" => {
                            modified_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "modified_by" => {
                            modified_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope_query" => {
                            scope_query =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "teams_channel_id" => {
                            teams_channel_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "teams_channel_name" => {
                            teams_channel_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "teams_team_id" => {
                            teams_team_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "teams_team_name" => {
                            teams_team_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "teams_tenant_id" => {
                            teams_tenant_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "teams_tenant_name" => {
                            teams_tenant_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timezone" => {
                            timezone = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "weekly_execution_day" => {
                            weekly_execution_day =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "weekly_execution_time" => {
                            weekly_execution_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "workspace_id" => {
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
                let channel_id = channel_id.ok_or_else(|| M::Error::missing_field("channel_id"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let created_by = created_by.ok_or_else(|| M::Error::missing_field("created_by"))?;
                let destination_type =
                    destination_type.ok_or_else(|| M::Error::missing_field("destination_type"))?;
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let frequency = frequency.ok_or_else(|| M::Error::missing_field("frequency"))?;
                let interval_execution_date = interval_execution_date
                    .ok_or_else(|| M::Error::missing_field("interval_execution_date"))?;
                let interval_execution_time = interval_execution_time
                    .ok_or_else(|| M::Error::missing_field("interval_execution_time"))?;
                let mention_targets =
                    mention_targets.ok_or_else(|| M::Error::missing_field("mention_targets"))?;
                let message_customization = message_customization
                    .ok_or_else(|| M::Error::missing_field("message_customization"))?;
                let modified_at =
                    modified_at.ok_or_else(|| M::Error::missing_field("modified_at"))?;
                let modified_by =
                    modified_by.ok_or_else(|| M::Error::missing_field("modified_by"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let org_id = org_id.ok_or_else(|| M::Error::missing_field("org_id"))?;
                let scope_query =
                    scope_query.ok_or_else(|| M::Error::missing_field("scope_query"))?;
                let teams_channel_id =
                    teams_channel_id.ok_or_else(|| M::Error::missing_field("teams_channel_id"))?;
                let teams_channel_name = teams_channel_name
                    .ok_or_else(|| M::Error::missing_field("teams_channel_name"))?;
                let teams_team_id =
                    teams_team_id.ok_or_else(|| M::Error::missing_field("teams_team_id"))?;
                let teams_team_name =
                    teams_team_name.ok_or_else(|| M::Error::missing_field("teams_team_name"))?;
                let teams_tenant_id =
                    teams_tenant_id.ok_or_else(|| M::Error::missing_field("teams_tenant_id"))?;
                let teams_tenant_name = teams_tenant_name
                    .ok_or_else(|| M::Error::missing_field("teams_tenant_name"))?;
                let timezone = timezone.ok_or_else(|| M::Error::missing_field("timezone"))?;
                let weekly_execution_day = weekly_execution_day
                    .ok_or_else(|| M::Error::missing_field("weekly_execution_day"))?;
                let weekly_execution_time = weekly_execution_time
                    .ok_or_else(|| M::Error::missing_field("weekly_execution_time"))?;
                let workspace_id =
                    workspace_id.ok_or_else(|| M::Error::missing_field("workspace_id"))?;

                let content = CostRecommendationNotificationRuleAttributes {
                    channel_id,
                    created_at,
                    created_by,
                    destination_type,
                    enabled,
                    frequency,
                    interval_execution_date,
                    interval_execution_time,
                    mention_targets,
                    message_customization,
                    modified_at,
                    modified_by,
                    name,
                    org_id,
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

        deserializer.deserialize_any(CostRecommendationNotificationRuleAttributesVisitor)
    }
}
