// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `reduce` processor aggregates and merges logs based on matching keys and merge strategies.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineReduceProcessor {
    /// A list of fields used to group log events for merging.
    #[serde(rename = "group_by")]
    pub group_by: Vec<String>,
    /// The unique identifier for this processor.
    #[serde(rename = "id")]
    pub id: String,
    /// A Datadog search query used to determine which logs this processor targets.
    #[serde(rename = "include")]
    pub include: String,
    /// A list of component IDs whose output is used as the input for this processor.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// List of merge strategies defining how values from grouped events should be combined.
    #[serde(rename = "merge_strategies")]
    pub merge_strategies:
        Vec<crate::datadogV2::model::ObservabilityPipelineReduceProcessorMergeStrategy>,
    /// The processor type. The value should always be `reduce`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineReduceProcessorType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineReduceProcessor {
    pub fn new(
        group_by: Vec<String>,
        id: String,
        include: String,
        inputs: Vec<String>,
        merge_strategies: Vec<
            crate::datadogV2::model::ObservabilityPipelineReduceProcessorMergeStrategy,
        >,
        type_: crate::datadogV2::model::ObservabilityPipelineReduceProcessorType,
    ) -> ObservabilityPipelineReduceProcessor {
        ObservabilityPipelineReduceProcessor {
            group_by,
            id,
            include,
            inputs,
            merge_strategies,
            type_,
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

impl<'de> Deserialize<'de> for ObservabilityPipelineReduceProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineReduceProcessorVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineReduceProcessorVisitor {
            type Value = ObservabilityPipelineReduceProcessor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut group_by: Option<Vec<String>> = None;
                let mut id: Option<String> = None;
                let mut include: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut merge_strategies: Option<
                    Vec<crate::datadogV2::model::ObservabilityPipelineReduceProcessorMergeStrategy>,
                > = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineReduceProcessorType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "group_by" => {
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include" => {
                            include = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputs" => {
                            inputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "merge_strategies" => {
                            merge_strategies =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineReduceProcessorType::UnparsedObject(_type_) => {
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
                let group_by = group_by.ok_or_else(|| M::Error::missing_field("group_by"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let include = include.ok_or_else(|| M::Error::missing_field("include"))?;
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;
                let merge_strategies =
                    merge_strategies.ok_or_else(|| M::Error::missing_field("merge_strategies"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineReduceProcessor {
                    group_by,
                    id,
                    include,
                    inputs,
                    merge_strategies,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineReduceProcessorVisitor)
    }
}
