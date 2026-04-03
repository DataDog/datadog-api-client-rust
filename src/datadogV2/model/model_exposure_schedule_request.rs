// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Progressive release request payload.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExposureScheduleRequest {
    /// The absolute UTC start time for this schedule.
    #[serde(
        rename = "absolute_start_time",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub absolute_start_time: Option<Option<chrono::DateTime<chrono::Utc>>>,
    /// The control variant ID used for experiment comparisons.
    #[serde(
        rename = "control_variant_id",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub control_variant_id: Option<Option<String>>,
    /// The control variant key used during creation workflows.
    #[serde(
        rename = "control_variant_key",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub control_variant_key: Option<Option<String>>,
    /// The unique identifier of the progressive rollout.
    #[serde(rename = "id")]
    pub id: Option<uuid::Uuid>,
    /// Rollout options request payload.
    #[serde(rename = "rollout_options")]
    pub rollout_options: crate::datadogV2::model::RolloutOptionsRequest,
    /// Ordered progression steps for exposure.
    #[serde(rename = "rollout_steps")]
    pub rollout_steps: Vec<crate::datadogV2::model::ExposureRolloutStepRequest>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExposureScheduleRequest {
    pub fn new(
        rollout_options: crate::datadogV2::model::RolloutOptionsRequest,
        rollout_steps: Vec<crate::datadogV2::model::ExposureRolloutStepRequest>,
    ) -> ExposureScheduleRequest {
        ExposureScheduleRequest {
            absolute_start_time: None,
            control_variant_id: None,
            control_variant_key: None,
            id: None,
            rollout_options,
            rollout_steps,
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

    pub fn control_variant_key(mut self, value: Option<String>) -> Self {
        self.control_variant_key = Some(value);
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

impl<'de> Deserialize<'de> for ExposureScheduleRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExposureScheduleRequestVisitor;
        impl<'a> Visitor<'a> for ExposureScheduleRequestVisitor {
            type Value = ExposureScheduleRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut absolute_start_time: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut control_variant_id: Option<Option<String>> = None;
                let mut control_variant_key: Option<Option<String>> = None;
                let mut id: Option<uuid::Uuid> = None;
                let mut rollout_options: Option<crate::datadogV2::model::RolloutOptionsRequest> =
                    None;
                let mut rollout_steps: Option<
                    Vec<crate::datadogV2::model::ExposureRolloutStepRequest>,
                > = None;
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
                        "control_variant_id" => {
                            control_variant_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "control_variant_key" => {
                            control_variant_key =
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
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let rollout_options =
                    rollout_options.ok_or_else(|| M::Error::missing_field("rollout_options"))?;
                let rollout_steps =
                    rollout_steps.ok_or_else(|| M::Error::missing_field("rollout_steps"))?;

                let content = ExposureScheduleRequest {
                    absolute_start_time,
                    control_variant_id,
                    control_variant_key,
                    id,
                    rollout_options,
                    rollout_steps,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExposureScheduleRequestVisitor)
    }
}
