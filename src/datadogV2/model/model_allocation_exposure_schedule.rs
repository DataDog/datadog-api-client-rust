// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Progressive release details for a targeting rule allocation.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AllocationExposureSchedule {
    /// The absolute UTC start time for this schedule.
    #[serde(
        rename = "absolute_start_time",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub absolute_start_time: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// The targeting rule allocation ID this progressive rollout belongs to.
    #[serde(rename = "allocation_id")]
    pub allocation_id: uuid::Uuid,
    /// The control variant ID used for experiment comparisons.
    #[serde(
        rename = "control_variant_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub control_variant_id: Option<Option<String>>,
    /// The timestamp when the schedule was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last guardrail action triggered for this schedule.
    #[serde(
        rename = "guardrail_triggered_action",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub guardrail_triggered_action: Option<Option<String>>,
    /// Guardrail trigger records for this schedule.
    #[serde(rename = "guardrail_triggers")]
    pub guardrail_triggers: Vec<crate::datadogV2::model::AllocationExposureGuardrailTrigger>,
    /// The unique identifier of the progressive rollout.
    #[serde(rename = "id")]
    pub id: Option<uuid::Uuid>,
    /// Applied progression options for a progressive rollout.
    #[serde(rename = "rollout_options")]
    pub rollout_options: crate::datadogV2::model::RolloutOptions,
    /// Ordered progression steps for exposure.
    #[serde(rename = "rollout_steps")]
    pub rollout_steps: Vec<crate::datadogV2::model::AllocationExposureRolloutStep>,
    /// The timestamp when the schedule was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AllocationExposureSchedule {
    pub fn new(
        allocation_id: uuid::Uuid,
        created_at: chrono::DateTime<chrono::Utc>,
        guardrail_triggers: Vec<crate::datadogV2::model::AllocationExposureGuardrailTrigger>,
        rollout_options: crate::datadogV2::model::RolloutOptions,
        rollout_steps: Vec<crate::datadogV2::model::AllocationExposureRolloutStep>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> AllocationExposureSchedule {
        AllocationExposureSchedule {
            absolute_start_time: None,
            allocation_id,
            control_variant_id: None,
            created_at,
            guardrail_triggered_action: None,
            guardrail_triggers,
            id: None,
            rollout_options,
            rollout_steps,
            updated_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn absolute_start_time(mut self, value: Option<chrono::DateTime<chrono::Utc>>) -> Self {
        self.absolute_start_time = Some(value);
        self
    }

    pub fn control_variant_id(mut self, value: Option<String>) -> Self {
        self.control_variant_id = Some(value);
        self
    }

    pub fn guardrail_triggered_action(mut self, value: Option<String>) -> Self {
        self.guardrail_triggered_action = Some(value);
        self
    }

    pub fn id(mut self, value: uuid::Uuid) -> Self {
        self.id = Some(value);
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

impl<'de> Deserialize<'de> for AllocationExposureSchedule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AllocationExposureScheduleVisitor;
        impl<'a> Visitor<'a> for AllocationExposureScheduleVisitor {
            type Value = AllocationExposureSchedule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut absolute_start_time: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut allocation_id: Option<uuid::Uuid> = None;
                let mut control_variant_id: Option<Option<String>> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut guardrail_triggered_action: Option<Option<String>> = None;
                let mut guardrail_triggers: Option<
                    Vec<crate::datadogV2::model::AllocationExposureGuardrailTrigger>,
                > = None;
                let mut id: Option<uuid::Uuid> = None;
                let mut rollout_options: Option<crate::datadogV2::model::RolloutOptions> = None;
                let mut rollout_steps: Option<
                    Vec<crate::datadogV2::model::AllocationExposureRolloutStep>,
                > = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "absolute_start_time" => {
                            absolute_start_time =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "allocation_id" => {
                            allocation_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "control_variant_id" => {
                            control_variant_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "guardrail_triggered_action" => {
                            guardrail_triggered_action =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "guardrail_triggers" => {
                            guardrail_triggers =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rollout_options" => {
                            rollout_options =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rollout_steps" => {
                            rollout_steps =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let allocation_id =
                    allocation_id.ok_or_else(|| M::Error::missing_field("allocation_id"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let guardrail_triggers = guardrail_triggers
                    .ok_or_else(|| M::Error::missing_field("guardrail_triggers"))?;
                let rollout_options =
                    rollout_options.ok_or_else(|| M::Error::missing_field("rollout_options"))?;
                let rollout_steps =
                    rollout_steps.ok_or_else(|| M::Error::missing_field("rollout_steps"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = AllocationExposureSchedule {
                    absolute_start_time,
                    allocation_id,
                    control_variant_id,
                    created_at,
                    guardrail_triggered_action,
                    guardrail_triggers,
                    id,
                    rollout_options,
                    rollout_steps,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AllocationExposureScheduleVisitor)
    }
}
