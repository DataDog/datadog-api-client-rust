// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Targeting rule (allocation) details for a feature flag environment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Allocation {
    /// The timestamp when the targeting rule allocation was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Environment IDs associated with this targeting rule allocation.
    #[serde(rename = "environment_ids")]
    pub environment_ids: Vec<uuid::Uuid>,
    /// The experiment ID linked to this targeting rule allocation.
    #[serde(
        rename = "experiment_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub experiment_id: Option<Option<String>>,
    /// Progressive release details for a targeting rule allocation.
    #[serde(rename = "exposure_schedule")]
    pub exposure_schedule: Option<crate::datadogV2::model::AllocationExposureSchedule>,
    /// Guardrail metrics associated with this targeting rule allocation.
    #[serde(rename = "guardrail_metrics")]
    pub guardrail_metrics: Vec<crate::datadogV2::model::GuardrailMetric>,
    /// The unique identifier of the targeting rule allocation.
    #[serde(rename = "id")]
    pub id: Option<uuid::Uuid>,
    /// The unique key of the targeting rule allocation.
    #[serde(rename = "key")]
    pub key: String,
    /// The display name of the targeting rule.
    #[serde(rename = "name")]
    pub name: String,
    /// Sort order position within the environment.
    #[serde(rename = "order_position")]
    pub order_position: i64,
    /// Conditions associated with this targeting rule allocation.
    #[serde(rename = "targeting_rules")]
    pub targeting_rules: Vec<crate::datadogV2::model::TargetingRule>,
    /// The type of targeting rule (called allocation in the API model).
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::AllocationType,
    /// The timestamp when the targeting rule allocation was last updated.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Weighted variant assignments for this targeting rule allocation.
    #[serde(rename = "variant_weights")]
    pub variant_weights: Vec<crate::datadogV2::model::VariantWeight>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl Allocation {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        environment_ids: Vec<uuid::Uuid>,
        guardrail_metrics: Vec<crate::datadogV2::model::GuardrailMetric>,
        key: String,
        name: String,
        order_position: i64,
        targeting_rules: Vec<crate::datadogV2::model::TargetingRule>,
        type_: crate::datadogV2::model::AllocationType,
        updated_at: chrono::DateTime<chrono::Utc>,
        variant_weights: Vec<crate::datadogV2::model::VariantWeight>,
    ) -> Allocation {
        Allocation {
            created_at,
            environment_ids,
            experiment_id: None,
            exposure_schedule: None,
            guardrail_metrics,
            id: None,
            key,
            name,
            order_position,
            targeting_rules,
            type_,
            updated_at,
            variant_weights,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn experiment_id(mut self, value: Option<String>) -> Self {
        self.experiment_id = Some(value);
        self
    }

    pub fn exposure_schedule(
        mut self,
        value: crate::datadogV2::model::AllocationExposureSchedule,
    ) -> Self {
        self.exposure_schedule = Some(value);
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

impl<'de> Deserialize<'de> for Allocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AllocationVisitor;
        impl<'a> Visitor<'a> for AllocationVisitor {
            type Value = Allocation;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut environment_ids: Option<Vec<uuid::Uuid>> = None;
                let mut experiment_id: Option<Option<String>> = None;
                let mut exposure_schedule: Option<
                    crate::datadogV2::model::AllocationExposureSchedule,
                > = None;
                let mut guardrail_metrics: Option<Vec<crate::datadogV2::model::GuardrailMetric>> =
                    None;
                let mut id: Option<uuid::Uuid> = None;
                let mut key: Option<String> = None;
                let mut name: Option<String> = None;
                let mut order_position: Option<i64> = None;
                let mut targeting_rules: Option<Vec<crate::datadogV2::model::TargetingRule>> = None;
                let mut type_: Option<crate::datadogV2::model::AllocationType> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut variant_weights: Option<Vec<crate::datadogV2::model::VariantWeight>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "environment_ids" => {
                            environment_ids =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "experiment_id" => {
                            experiment_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "exposure_schedule" => {
                            if v.is_null() {
                                continue;
                            }
                            exposure_schedule =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "guardrail_metrics" => {
                            guardrail_metrics =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "key" => {
                            key = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "order_position" => {
                            order_position =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "targeting_rules" => {
                            targeting_rules =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::AllocationType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variant_weights" => {
                            variant_weights =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let environment_ids =
                    environment_ids.ok_or_else(|| M::Error::missing_field("environment_ids"))?;
                let guardrail_metrics = guardrail_metrics
                    .ok_or_else(|| M::Error::missing_field("guardrail_metrics"))?;
                let key = key.ok_or_else(|| M::Error::missing_field("key"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let order_position =
                    order_position.ok_or_else(|| M::Error::missing_field("order_position"))?;
                let targeting_rules =
                    targeting_rules.ok_or_else(|| M::Error::missing_field("targeting_rules"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;
                let variant_weights =
                    variant_weights.ok_or_else(|| M::Error::missing_field("variant_weights"))?;

                let content = Allocation {
                    created_at,
                    environment_ids,
                    experiment_id,
                    exposure_schedule,
                    guardrail_metrics,
                    id,
                    key,
                    name,
                    order_position,
                    targeting_rules,
                    type_,
                    updated_at,
                    variant_weights,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AllocationVisitor)
    }
}
