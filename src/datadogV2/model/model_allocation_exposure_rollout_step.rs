// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Exposure progression step details.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AllocationExposureRolloutStep {
    /// The progressive rollout ID this step belongs to.
    #[serde(rename = "allocation_exposure_schedule_id")]
    pub allocation_exposure_schedule_id: uuid::Uuid,
    /// The timestamp when the progression step was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The exposure ratio for this step.
    #[serde(rename = "exposure_ratio")]
    pub exposure_ratio: f64,
    /// Logical index grouping related steps.
    #[serde(rename = "grouped_step_index")]
    pub grouped_step_index: i64,
    /// The unique identifier of the progression step.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// Step duration in milliseconds.
    #[serde(
        rename = "interval_ms",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub interval_ms: Option<Option<i64>>,
    /// Whether this step represents a pause record.
    #[serde(rename = "is_pause_record")]
    pub is_pause_record: bool,
    /// Sort order for the progression step.
    #[serde(rename = "order_position")]
    pub order_position: i64,
    /// The timestamp when the progression step was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AllocationExposureRolloutStep {
    pub fn new(
        allocation_exposure_schedule_id: uuid::Uuid,
        created_at: chrono::DateTime<chrono::Utc>,
        exposure_ratio: f64,
        grouped_step_index: i64,
        id: uuid::Uuid,
        is_pause_record: bool,
        order_position: i64,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> AllocationExposureRolloutStep {
        AllocationExposureRolloutStep {
            allocation_exposure_schedule_id,
            created_at,
            exposure_ratio,
            grouped_step_index,
            id,
            interval_ms: None,
            is_pause_record,
            order_position,
            updated_at,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn interval_ms(mut self, value: Option<i64>) -> Self {
        self.interval_ms = Some(value);
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

impl<'de> Deserialize<'de> for AllocationExposureRolloutStep {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AllocationExposureRolloutStepVisitor;
        impl<'a> Visitor<'a> for AllocationExposureRolloutStepVisitor {
            type Value = AllocationExposureRolloutStep;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut allocation_exposure_schedule_id: Option<uuid::Uuid> = None;
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut exposure_ratio: Option<f64> = None;
                let mut grouped_step_index: Option<i64> = None;
                let mut id: Option<uuid::Uuid> = None;
                let mut interval_ms: Option<Option<i64>> = None;
                let mut is_pause_record: Option<bool> = None;
                let mut order_position: Option<i64> = None;
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
                        "exposure_ratio" => {
                            exposure_ratio =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "grouped_step_index" => {
                            grouped_step_index =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "interval_ms" => {
                            interval_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_pause_record" => {
                            is_pause_record =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "order_position" => {
                            order_position =
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
                let exposure_ratio =
                    exposure_ratio.ok_or_else(|| M::Error::missing_field("exposure_ratio"))?;
                let grouped_step_index = grouped_step_index
                    .ok_or_else(|| M::Error::missing_field("grouped_step_index"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let is_pause_record =
                    is_pause_record.ok_or_else(|| M::Error::missing_field("is_pause_record"))?;
                let order_position =
                    order_position.ok_or_else(|| M::Error::missing_field("order_position"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = AllocationExposureRolloutStep {
                    allocation_exposure_schedule_id,
                    created_at,
                    exposure_ratio,
                    grouped_step_index,
                    id,
                    interval_ms,
                    is_pause_record,
                    order_position,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AllocationExposureRolloutStepVisitor)
    }
}
