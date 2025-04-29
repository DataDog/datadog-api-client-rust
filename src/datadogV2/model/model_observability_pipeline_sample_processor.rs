// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `sample` processor allows probabilistic sampling of logs at a fixed rate.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineSampleProcessor {
    /// The unique identifier for this component. Used to reference this component in other parts of the pipeline (for example, as the `input` to downstream components).
    #[serde(rename = "id")]
    pub id: String,
    /// A Datadog search query used to determine which logs this processor targets.
    #[serde(rename = "include")]
    pub include: String,
    /// A list of component IDs whose output is used as the `input` for this component.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// The percentage of logs to sample.
    #[serde(rename = "percentage")]
    pub percentage: Option<f64>,
    /// Number of events to sample (1 in N).
    #[serde(rename = "rate")]
    pub rate: Option<i64>,
    /// The processor type. The value should always be `sample`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineSampleProcessorType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineSampleProcessor {
    pub fn new(
        id: String,
        include: String,
        inputs: Vec<String>,
        type_: crate::datadogV2::model::ObservabilityPipelineSampleProcessorType,
    ) -> ObservabilityPipelineSampleProcessor {
        ObservabilityPipelineSampleProcessor {
            id,
            include,
            inputs,
            percentage: None,
            rate: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn percentage(mut self, value: f64) -> Self {
        self.percentage = Some(value);
        self
    }

    pub fn rate(mut self, value: i64) -> Self {
        self.rate = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineSampleProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineSampleProcessorVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineSampleProcessorVisitor {
            type Value = ObservabilityPipelineSampleProcessor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut id: Option<String> = None;
                let mut include: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut percentage: Option<f64> = None;
                let mut rate: Option<i64> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineSampleProcessorType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "include" => {
                            include = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputs" => {
                            inputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "percentage" => {
                            if v.is_null() {
                                continue;
                            }
                            percentage = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rate" => {
                            if v.is_null() {
                                continue;
                            }
                            rate = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineSampleProcessorType::UnparsedObject(_type_) => {
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
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let include = include.ok_or_else(|| M::Error::missing_field("include"))?;
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineSampleProcessor {
                    id,
                    include,
                    inputs,
                    percentage,
                    rate,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineSampleProcessorVisitor)
    }
}
