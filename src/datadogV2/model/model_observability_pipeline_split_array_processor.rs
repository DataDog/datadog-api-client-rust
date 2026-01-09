// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `split_array` processor splits array fields into separate events based on configured rules.
///
/// **Supported pipeline types:** logs
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineSplitArrayProcessor {
    /// A list of array split configurations.
    #[serde(rename = "arrays")]
    pub arrays: Vec<crate::datadogV2::model::ObservabilityPipelineSplitArrayProcessorArrayConfig>,
    /// The display name for a component.
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    /// Indicates whether the processor is enabled.
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The unique identifier for this component. Used in other parts of the pipeline to reference this component (for example, as the `input` to downstream components).
    #[serde(rename = "id")]
    pub id: String,
    /// A Datadog search query used to determine which logs this processor targets. For split_array, this should typically be `*`.
    #[serde(rename = "include")]
    pub include: String,
    /// The processor type. The value should always be `split_array`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineSplitArrayProcessorType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineSplitArrayProcessor {
    pub fn new(
        arrays: Vec<crate::datadogV2::model::ObservabilityPipelineSplitArrayProcessorArrayConfig>,
        enabled: bool,
        id: String,
        include: String,
        type_: crate::datadogV2::model::ObservabilityPipelineSplitArrayProcessorType,
    ) -> ObservabilityPipelineSplitArrayProcessor {
        ObservabilityPipelineSplitArrayProcessor {
            arrays,
            display_name: None,
            enabled,
            id,
            include,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineSplitArrayProcessor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineSplitArrayProcessorVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineSplitArrayProcessorVisitor {
            type Value = ObservabilityPipelineSplitArrayProcessor;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut arrays: Option<Vec<crate::datadogV2::model::ObservabilityPipelineSplitArrayProcessorArrayConfig>> = None;
                let mut display_name: Option<String> = None;
                let mut enabled: Option<bool> = None;
                let mut id: Option<String> = None;
                let mut include: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineSplitArrayProcessorType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "arrays" => {
                            arrays = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineSplitArrayProcessorType::UnparsedObject(_type_) => {
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
                let arrays = arrays.ok_or_else(|| M::Error::missing_field("arrays"))?;
                let enabled = enabled.ok_or_else(|| M::Error::missing_field("enabled"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let include = include.ok_or_else(|| M::Error::missing_field("include"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineSplitArrayProcessor {
                    arrays,
                    display_name,
                    enabled,
                    id,
                    include,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineSplitArrayProcessorVisitor)
    }
}
