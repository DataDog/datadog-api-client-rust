// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Request to create or update a targeting rule (allocation) for a feature flag environment.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UpsertAllocationRequest {
    /// The experiment ID for experiment-linked allocations.
    #[serde(
        rename = "experiment_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub experiment_id: Option<Option<String>>,
    /// Progressive release request payload.
    #[serde(rename = "exposure_schedule")]
    pub exposure_schedule: Option<crate::datadogV2::model::ExposureScheduleRequest>,
    /// Guardrail metrics used to monitor and auto-pause or abort.
    #[serde(rename = "guardrail_metrics")]
    pub guardrail_metrics: Option<Vec<crate::datadogV2::model::GuardrailMetricRequest>>,
    /// The unique identifier of the targeting rule allocation.
    #[serde(rename = "id")]
    pub id: Option<uuid::Uuid>,
    /// The unique key of the targeting rule allocation.
    #[serde(rename = "key")]
    pub key: String,
    /// The display name of the targeting rule.
    #[serde(rename = "name")]
    pub name: String,
    /// Targeting rules that determine audience eligibility.
    #[serde(rename = "targeting_rules")]
    pub targeting_rules: Option<Vec<crate::datadogV2::model::TargetingRuleRequest>>,
    /// The type of targeting rule (called allocation in the API model).
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::AllocationType,
    /// Variant distribution weights.
    #[serde(rename = "variant_weights")]
    pub variant_weights: Option<Vec<crate::datadogV2::model::VariantWeightRequest>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UpsertAllocationRequest {
    pub fn new(
        key: String,
        name: String,
        type_: crate::datadogV2::model::AllocationType,
    ) -> UpsertAllocationRequest {
        UpsertAllocationRequest {
            experiment_id: None,
            exposure_schedule: None,
            guardrail_metrics: None,
            id: None,
            key,
            name,
            targeting_rules: None,
            type_,
            variant_weights: None,
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
        value: crate::datadogV2::model::ExposureScheduleRequest,
    ) -> Self {
        self.exposure_schedule = Some(value);
        self
    }

    pub fn guardrail_metrics(
        mut self,
        value: Vec<crate::datadogV2::model::GuardrailMetricRequest>,
    ) -> Self {
        self.guardrail_metrics = Some(value);
        self
    }

    pub fn id(mut self, value: uuid::Uuid) -> Self {
        self.id = Some(value);
        self
    }

    pub fn targeting_rules(
        mut self,
        value: Vec<crate::datadogV2::model::TargetingRuleRequest>,
    ) -> Self {
        self.targeting_rules = Some(value);
        self
    }

    pub fn variant_weights(
        mut self,
        value: Vec<crate::datadogV2::model::VariantWeightRequest>,
    ) -> Self {
        self.variant_weights = Some(value);
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

impl<'de> Deserialize<'de> for UpsertAllocationRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UpsertAllocationRequestVisitor;
        impl<'a> Visitor<'a> for UpsertAllocationRequestVisitor {
            type Value = UpsertAllocationRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut experiment_id: Option<Option<String>> = None;
                let mut exposure_schedule: Option<
                    crate::datadogV2::model::ExposureScheduleRequest,
                > = None;
                let mut guardrail_metrics: Option<
                    Vec<crate::datadogV2::model::GuardrailMetricRequest>,
                > = None;
                let mut id: Option<uuid::Uuid> = None;
                let mut key: Option<String> = None;
                let mut name: Option<String> = None;
                let mut targeting_rules: Option<
                    Vec<crate::datadogV2::model::TargetingRuleRequest>,
                > = None;
                let mut type_: Option<crate::datadogV2::model::AllocationType> = None;
                let mut variant_weights: Option<
                    Vec<crate::datadogV2::model::VariantWeightRequest>,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                            if v.is_null() {
                                continue;
                            }
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
                        "targeting_rules" => {
                            if v.is_null() {
                                continue;
                            }
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
                        "variant_weights" => {
                            if v.is_null() {
                                continue;
                            }
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
                let key = key.ok_or_else(|| M::Error::missing_field("key"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = UpsertAllocationRequest {
                    experiment_id,
                    exposure_schedule,
                    guardrail_metrics,
                    id,
                    key,
                    name,
                    targeting_rules,
                    type_,
                    variant_weights,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UpsertAllocationRequestVisitor)
    }
}
