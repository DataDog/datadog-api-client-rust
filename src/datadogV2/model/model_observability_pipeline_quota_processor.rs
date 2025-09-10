// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The Quota Processor measures logging traffic for logs that match a specified filter. When the configured daily quota is met, the processor can drop or alert.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineQuotaProcessor {
    /// If set to `true`, logs that matched the quota filter and sent after the quota has been met are dropped; only logs that did not match the filter query continue through the pipeline.
    #[serde(rename = "drop_events")]
    pub drop_events: bool,
    /// The processor passes through all events if it is set to `false`. Defaults to `true`.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// The unique identifier for this component. Used to reference this component in other parts of the pipeline (for example, as the `input` to downstream components).
    #[serde(rename = "id")]
    pub id: String,
    /// If `true`, the processor skips quota checks when partition fields are missing from the logs.
    #[serde(rename = "ignore_when_missing_partitions")]
    pub ignore_when_missing_partitions: Option<bool>,
    /// A Datadog search query used to determine which logs this processor targets.
    #[serde(rename = "include")]
    pub include: String,
    /// A list of component IDs whose output is used as the `input` for this component.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// The maximum amount of data or number of events allowed before the quota is enforced. Can be specified in bytes or events.
    #[serde(rename = "limit")]
    pub limit: crate::datadogV2::model::ObservabilityPipelineQuotaProcessorLimit,
    /// Name of the quota.
    #[serde(rename = "name")]
    pub name: String,
    /// The action to take when the quota is exceeded. Options:
    /// - `drop`: Drop the event.
    /// - `no_action`: Let the event pass through.
    /// - `overflow_routing`: Route to an overflow destination.
    ///
    #[serde(rename = "overflow_action")]
    pub overflow_action:
        Option<crate::datadogV2::model::ObservabilityPipelineQuotaProcessorOverflowAction>,
    /// A list of alternate quota rules that apply to specific sets of events, identified by matching field values. Each override can define a custom limit.
    #[serde(rename = "overrides")]
    pub overrides:
        Option<Vec<crate::datadogV2::model::ObservabilityPipelineQuotaProcessorOverride>>,
    /// A list of fields used to segment log traffic for quota enforcement. Quotas are tracked independently by unique combinations of these field values.
    #[serde(rename = "partition_fields")]
    pub partition_fields: Option<Vec<String>>,
    /// The processor type. The value should always be `quota`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineQuotaProcessorType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineQuotaProcessor {
    pub fn new(
        drop_events: bool,
        id: String,
        include: String,
        inputs: Vec<String>,
        limit: crate::datadogV2::model::ObservabilityPipelineQuotaProcessorLimit,
        name: String,
        type_: crate::datadogV2::model::ObservabilityPipelineQuotaProcessorType,
    ) -> ObservabilityPipelineQuotaProcessor {
        ObservabilityPipelineQuotaProcessor {
            drop_events,
            enabled: None,
            id,
            ignore_when_missing_partitions: None,
            include,
            inputs,
            limit,
            name,
            overflow_action: None,
            overrides: None,
            partition_fields: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn ignore_when_missing_partitions(mut self, value: bool) -> Self {
        self.ignore_when_missing_partitions = Some(value);
        self
    }

    pub fn overflow_action(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineQuotaProcessorOverflowAction,
    ) -> Self {
        self.overflow_action = Some(value);
        self
    }

    pub fn overrides(
        mut self,
        value: Vec<crate::datadogV2::model::ObservabilityPipelineQuotaProcessorOverride>,
    ) -> Self {
        self.overrides = Some(value);
        self
    }

    pub fn partition_fields(mut self, value: Vec<String>) -> Self {
        self.partition_fields = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineQuotaProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineQuotaProcessorVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineQuotaProcessorVisitor {
            type Value = ObservabilityPipelineQuotaProcessor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut drop_events: Option<bool> = None;
                let mut enabled: Option<bool> = None;
                let mut id: Option<String> = None;
                let mut ignore_when_missing_partitions: Option<bool> = None;
                let mut include: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut limit: Option<
                    crate::datadogV2::model::ObservabilityPipelineQuotaProcessorLimit,
                > = None;
                let mut name: Option<String> = None;
                let mut overflow_action: Option<
                    crate::datadogV2::model::ObservabilityPipelineQuotaProcessorOverflowAction,
                > = None;
                let mut overrides: Option<
                    Vec<crate::datadogV2::model::ObservabilityPipelineQuotaProcessorOverride>,
                > = None;
                let mut partition_fields: Option<Vec<String>> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineQuotaProcessorType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "drop_events" => {
                            drop_events =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ignore_when_missing_partitions" => {
                            if v.is_null() {
                                continue;
                            }
                            ignore_when_missing_partitions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include" => {
                            include = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputs" => {
                            inputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "limit" => {
                            limit = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "overflow_action" => {
                            if v.is_null() {
                                continue;
                            }
                            overflow_action =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _overflow_action) = overflow_action {
                                match _overflow_action {
                                    crate::datadogV2::model::ObservabilityPipelineQuotaProcessorOverflowAction::UnparsedObject(_overflow_action) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "overrides" => {
                            if v.is_null() {
                                continue;
                            }
                            overrides = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "partition_fields" => {
                            if v.is_null() {
                                continue;
                            }
                            partition_fields =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineQuotaProcessorType::UnparsedObject(_type_) => {
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
                let drop_events =
                    drop_events.ok_or_else(|| M::Error::missing_field("drop_events"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let include = include.ok_or_else(|| M::Error::missing_field("include"))?;
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;
                let limit = limit.ok_or_else(|| M::Error::missing_field("limit"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineQuotaProcessor {
                    drop_events,
                    enabled,
                    id,
                    ignore_when_missing_partitions,
                    include,
                    inputs,
                    limit,
                    name,
                    overflow_action,
                    overrides,
                    partition_fields,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineQuotaProcessorVisitor)
    }
}
