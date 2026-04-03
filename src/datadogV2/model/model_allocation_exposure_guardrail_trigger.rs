// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Guardrail trigger details for a progressive rollout.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AllocationExposureGuardrailTrigger {
    /// The progressive rollout ID this trigger belongs to.
    #[serde(rename = "allocation_exposure_schedule_id")]
    pub allocation_exposure_schedule_id: uuid::Uuid,
    /// The timestamp when this trigger was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The variant ID that triggered this event.
    #[serde(rename = "flagging_variant_id")]
    pub flagging_variant_id: uuid::Uuid,
    /// The unique identifier of the guardrail trigger.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// The metric ID associated with the trigger.
    #[serde(rename = "metric_id")]
    pub metric_id: String,
    /// The action that was triggered.
    #[serde(rename = "triggered_action")]
    pub triggered_action: String,
    /// The timestamp when this trigger was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AllocationExposureGuardrailTrigger {
    pub fn new(
        allocation_exposure_schedule_id: uuid::Uuid,
        created_at: chrono::DateTime<chrono::Utc>,
        flagging_variant_id: uuid::Uuid,
        id: uuid::Uuid,
        metric_id: String,
        triggered_action: String,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> AllocationExposureGuardrailTrigger {
        AllocationExposureGuardrailTrigger {
            allocation_exposure_schedule_id,
            created_at,
            flagging_variant_id,
            id,
            metric_id,
            triggered_action,
            updated_at,
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

impl<'de> Deserialize<'de> for AllocationExposureGuardrailTrigger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AllocationExposureGuardrailTriggerVisitor;
        impl<'a> Visitor<'a> for AllocationExposureGuardrailTriggerVisitor {
            type Value = AllocationExposureGuardrailTrigger;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allocation_exposure_schedule_id: Option<uuid::Uuid> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut flagging_variant_id: Option<uuid::Uuid> = None;
                let mut id: Option<uuid::Uuid> = None;
                let mut metric_id: Option<String> = None;
                let mut triggered_action: Option<String> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "allocation_exposure_schedule_id" => {
                            allocation_exposure_schedule_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "flagging_variant_id" => {
                            flagging_variant_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metric_id" => {
                            metric_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "triggered_action" => {
                            triggered_action =
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
                let allocation_exposure_schedule_id = allocation_exposure_schedule_id
                    .ok_or_else(|| M::Error::missing_field("allocation_exposure_schedule_id"))?;
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let flagging_variant_id = flagging_variant_id
                    .ok_or_else(|| M::Error::missing_field("flagging_variant_id"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let metric_id = metric_id.ok_or_else(|| M::Error::missing_field("metric_id"))?;
                let triggered_action =
                    triggered_action.ok_or_else(|| M::Error::missing_field("triggered_action"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = AllocationExposureGuardrailTrigger {
                    allocation_exposure_schedule_id,
                    created_at,
                    flagging_variant_id,
                    id,
                    metric_id,
                    triggered_action,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AllocationExposureGuardrailTriggerVisitor)
    }
}
