// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `spike_protection` processor learns the normal ingestion volume for the logs that match a specified filter and takes action when the volume spikes unexpectedly.
///
/// **Supported pipeline types:** logs
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineSpikeProtectionProcessor {
    /// How to handle logs that are missing one of the `partition_fields`. Options:
/// - `ignore`: Forward the event without applying spike protection.
/// - `default_bucket`: Assign the event to a default partition.
    #[serde(rename = "action_when_missing_partitions")]
    pub action_when_missing_partitions: Option<crate::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorActionWhenMissingPartitions>,
    /// The display name for a component.
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    /// Indicates whether the processor is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The unique identifier for this component. Used in other parts of the pipeline to reference this component (for example, as the `input` to downstream components).
    #[serde(rename = "id")]
    pub id: String,
    /// A Datadog search query used to determine which logs this processor targets.
    #[serde(rename = "include")]
    pub include: String,
    /// The minimum volume, in bytes, that must be observed within 24 hours before spike protection can activate. Defaults to 100 GiB.
    #[serde(rename = "minimal_threshold")]
    pub minimal_threshold: Option<i64>,
    /// A list of per-partition overrides for the minimal threshold, identified by matching field values. Requires `partition_fields` to be set.
    #[serde(rename = "minimal_threshold_overrides")]
    pub minimal_threshold_overrides: Option<Vec<crate::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorThresholdOverride>>,
    /// Name of the spike protection rule.
    #[serde(rename = "name")]
    pub name: String,
    /// A list of fields used to segment log traffic. Volume is learned and evaluated independently for each unique combination of these field values. A maximum of five fields is supported.
    #[serde(rename = "partition_fields")]
    pub partition_fields: Option<Vec<String>>,
    /// The action to take with matching logs while spike protection is active. Options:
/// - `drop`: Drop the event.
/// - `no_action`: Let the event pass through.
/// - `overflow_routing`: Route to an overflow destination.
///
/// When `overflow_routing` is used, there must be a destination whose `inputs` reference this processor with the `<processor-id>.overflow_events` suffix to route the overflowing events.
    #[serde(rename = "spike_action")]
    pub spike_action: crate::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorSpikeAction,
    /// The number of standard deviations above the learned mean volume that must be exceeded before a spike is detected. Defaults to `5`.
    #[serde(rename = "spike_threshold")]
    pub spike_threshold: Option<f64>,
    /// The processor type. The value should always be `spike_protection`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ObservabilityPipelineSpikeProtectionProcessor {
    pub fn new(
        enabled: bool,
        id: String,
        include: String,
        name: String,
        spike_action: crate::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorSpikeAction,
        type_: crate::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorType,
    ) -> ObservabilityPipelineSpikeProtectionProcessor {
        ObservabilityPipelineSpikeProtectionProcessor {
            action_when_missing_partitions: None,
            display_name: None,
            enabled,
            id,
            include,
            minimal_threshold: None,
            minimal_threshold_overrides: None,
            name,
            partition_fields: None,
            spike_action,
            spike_threshold: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn action_when_missing_partitions(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorActionWhenMissingPartitions,
    ) -> Self {
        self.action_when_missing_partitions = Some(value);
        self
    }

    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }

    pub fn minimal_threshold(mut self, value: i64) -> Self {
        self.minimal_threshold = Some(value);
        self
    }

    pub fn minimal_threshold_overrides(
        mut self,
        value: Vec<
            crate::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorThresholdOverride,
        >,
    ) -> Self {
        self.minimal_threshold_overrides = Some(value);
        self
    }

    pub fn partition_fields(mut self, value: Vec<String>) -> Self {
        self.partition_fields = Some(value);
        self
    }

    pub fn spike_threshold(mut self, value: f64) -> Self {
        self.spike_threshold = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineSpikeProtectionProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineSpikeProtectionProcessorVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineSpikeProtectionProcessorVisitor {
            type Value = ObservabilityPipelineSpikeProtectionProcessor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut action_when_missing_partitions: Option<crate::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorActionWhenMissingPartitions> = None;
                let mut display_name: Option<String> = None;
                let mut enabled: Option<bool> = None;
                let mut id: Option<String> = None;
                let mut include: Option<String> = None;
                let mut minimal_threshold: Option<i64> = None;
                let mut minimal_threshold_overrides: Option<Vec<crate::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorThresholdOverride>> = None;
                let mut name: Option<String> = None;
                let mut partition_fields: Option<Vec<String>> = None;
                let mut spike_action: Option<crate::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorSpikeAction> = None;
                let mut spike_threshold: Option<f64> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "action_when_missing_partitions" => {
                            if v.is_null() {
                                continue;
                            }
                            action_when_missing_partitions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _action_when_missing_partitions) =
                                action_when_missing_partitions
                            {
                                match _action_when_missing_partitions {
                                    crate::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorActionWhenMissingPartitions::UnparsedObject(_action_when_missing_partitions) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "display_name" => {
                            if v.is_null() {
                                continue;
                            }
                            display_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include" => {
                            include = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "minimal_threshold" => {
                            if v.is_null() {
                                continue;
                            }
                            minimal_threshold =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "minimal_threshold_overrides" => {
                            if v.is_null() {
                                continue;
                            }
                            minimal_threshold_overrides =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "partition_fields" => {
                            if v.is_null() {
                                continue;
                            }
                            partition_fields =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "spike_action" => {
                            spike_action =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _spike_action) = spike_action {
                                match _spike_action {
                                    crate::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorSpikeAction::UnparsedObject(_spike_action) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "spike_threshold" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            spike_threshold =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineSpikeProtectionProcessorType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let include = include.ok_or_else(|| M::Error::missing_field("include"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let spike_action =
                    spike_action.ok_or_else(|| M::Error::missing_field("spike_action"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineSpikeProtectionProcessor {
                    action_when_missing_partitions,
                    display_name,
                    enabled,
                    id,
                    include,
                    minimal_threshold,
                    minimal_threshold_overrides,
                    name,
                    partition_fields,
                    spike_action,
                    spike_threshold,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineSpikeProtectionProcessorVisitor)
    }
}
