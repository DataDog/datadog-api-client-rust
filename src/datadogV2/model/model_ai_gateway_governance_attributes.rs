// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Organization-wide AI Gateway governance controls.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayGovernanceAttributes {
    /// Controls whether AI Gateway captures request and response content in addition to metadata.
    #[serde(rename = "data_capture_mode")]
    pub data_capture_mode: crate::datadogV2::model::AiGatewayDataCaptureMode,
    /// Items Metadata-only capture adds, each listed once. An item requires the items whose information it already reveals: `tool_output_size`, `tool_names` and `conversation_shape` require `tool_usage`, and `input_size_by_role` requires `tool_output_size`. A set that leaves out a required item is rejected. When several Metadata-only overrides match a request, it gets only the items all of them select.
    #[serde(rename = "metadata_capture")]
    pub metadata_capture: Option<Vec<crate::datadogV2::model::AiGatewayMetadataCaptureItem>>,
    /// Zero data retention requirement for the organization's requests.
    #[serde(rename = "zdr_mode")]
    pub zdr_mode: crate::datadogV2::model::AiGatewayZDRMode,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayGovernanceAttributes {
    pub fn new(
        data_capture_mode: crate::datadogV2::model::AiGatewayDataCaptureMode,
        zdr_mode: crate::datadogV2::model::AiGatewayZDRMode,
    ) -> AiGatewayGovernanceAttributes {
        AiGatewayGovernanceAttributes {
            data_capture_mode,
            metadata_capture: None,
            zdr_mode,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn metadata_capture(
        mut self,
        value: Vec<crate::datadogV2::model::AiGatewayMetadataCaptureItem>,
    ) -> Self {
        self.metadata_capture = Some(value);
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

impl<'de> Deserialize<'de> for AiGatewayGovernanceAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayGovernanceAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayGovernanceAttributesVisitor {
            type Value = AiGatewayGovernanceAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_capture_mode: Option<
                    crate::datadogV2::model::AiGatewayDataCaptureMode,
                > = None;
                let mut metadata_capture: Option<
                    Vec<crate::datadogV2::model::AiGatewayMetadataCaptureItem>,
                > = None;
                let mut zdr_mode: Option<crate::datadogV2::model::AiGatewayZDRMode> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_capture_mode" => {
                            data_capture_mode =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_capture_mode) = data_capture_mode {
                                match _data_capture_mode {
                                    crate::datadogV2::model::AiGatewayDataCaptureMode::UnparsedObject(_data_capture_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "metadata_capture" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata_capture =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "zdr_mode" => {
                            zdr_mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _zdr_mode) = zdr_mode {
                                match _zdr_mode {
                                    crate::datadogV2::model::AiGatewayZDRMode::UnparsedObject(
                                        _zdr_mode,
                                    ) => {
                                        _unparsed = true;
                                    }
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
                let data_capture_mode = data_capture_mode
                    .ok_or_else(|| M::Error::missing_field("data_capture_mode"))?;
                let zdr_mode = zdr_mode.ok_or_else(|| M::Error::missing_field("zdr_mode"))?;

                let content = AiGatewayGovernanceAttributes {
                    data_capture_mode,
                    metadata_capture,
                    zdr_mode,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayGovernanceAttributesVisitor)
    }
}
