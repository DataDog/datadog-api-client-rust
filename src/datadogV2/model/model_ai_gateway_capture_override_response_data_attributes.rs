// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Saved name, capture mode, request filters, and revision of the override.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayCaptureOverrideResponseDataAttributes {
    /// Items Metadata-only capture adds, each listed once. An item requires the items whose information it already reveals: `tool_output_size`, `tool_names` and `conversation_shape` require `tool_usage`, and `input_size_by_role` requires `tool_output_size`. A set that leaves out a required item is rejected. When several Metadata-only overrides match a request, it gets only the items all of them select.
    #[serde(rename = "metadata_capture")]
    pub metadata_capture: Option<Vec<crate::datadogV2::model::AiGatewayMetadataCaptureItem>>,
    /// Capture mode for matching traffic.
    #[serde(rename = "mode")]
    pub mode: crate::datadogV2::model::AiGatewayCaptureOverrideResponseDataAttributesMode,
    /// Display name of the override. Names need not be unique; identify overrides by `id`.
    #[serde(rename = "name")]
    pub name: String,
    /// Current override revision, required for replacement or deletion.
    #[serde(rename = "revision")]
    pub revision: i64,
    /// Visible request filters. Provider and model tags are withheld when the caller cannot view them. An empty object or empty tags can represent a fully restricted scope; scope_restricted identifies redaction.
    #[serde(rename = "scope")]
    pub scope: crate::datadogV2::model::AiGatewayCaptureResponseScope,
    /// Whether provider or model scope metadata was withheld. Omit unchanged scope on replacement to preserve it.
    #[serde(rename = "scope_restricted")]
    pub scope_restricted: bool,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayCaptureOverrideResponseDataAttributes {
    pub fn new(
        mode: crate::datadogV2::model::AiGatewayCaptureOverrideResponseDataAttributesMode,
        name: String,
        revision: i64,
        scope: crate::datadogV2::model::AiGatewayCaptureResponseScope,
        scope_restricted: bool,
    ) -> AiGatewayCaptureOverrideResponseDataAttributes {
        AiGatewayCaptureOverrideResponseDataAttributes {
            metadata_capture: None,
            mode,
            name,
            revision,
            scope,
            scope_restricted,
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

impl<'de> Deserialize<'de> for AiGatewayCaptureOverrideResponseDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayCaptureOverrideResponseDataAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayCaptureOverrideResponseDataAttributesVisitor {
            type Value = AiGatewayCaptureOverrideResponseDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut metadata_capture: Option<
                    Vec<crate::datadogV2::model::AiGatewayMetadataCaptureItem>,
                > = None;
                let mut mode: Option<
                    crate::datadogV2::model::AiGatewayCaptureOverrideResponseDataAttributesMode,
                > = None;
                let mut name: Option<String> = None;
                let mut revision: Option<i64> = None;
                let mut scope: Option<crate::datadogV2::model::AiGatewayCaptureResponseScope> =
                    None;
                let mut scope_restricted: Option<bool> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "metadata_capture" => {
                            if v.is_null() {
                                continue;
                            }
                            metadata_capture =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mode" => {
                            mode = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _mode) = mode {
                                match _mode {
                                    crate::datadogV2::model::AiGatewayCaptureOverrideResponseDataAttributesMode::UnparsedObject(_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "revision" => {
                            revision = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope_restricted" => {
                            scope_restricted =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let mode = mode.ok_or_else(|| M::Error::missing_field("mode"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let revision = revision.ok_or_else(|| M::Error::missing_field("revision"))?;
                let scope = scope.ok_or_else(|| M::Error::missing_field("scope"))?;
                let scope_restricted =
                    scope_restricted.ok_or_else(|| M::Error::missing_field("scope_restricted"))?;

                let content = AiGatewayCaptureOverrideResponseDataAttributes {
                    metadata_capture,
                    mode,
                    name,
                    revision,
                    scope,
                    scope_restricted,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayCaptureOverrideResponseDataAttributesVisitor)
    }
}
