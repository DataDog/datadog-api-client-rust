// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Replacement settings and revisions for updating the override.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayCaptureUpdateRequestDataAttributes {
    /// Opaque revision of the organization default and all overrides. Writes require the revision returned by a current read or preview.
    #[serde(rename = "configuration_revision")]
    pub configuration_revision: String,
    /// Set to `true` to confirm a change that can enable full capture, including a deletion that causes a `full_capture` override or the organization default to apply.
    #[serde(rename = "confirm_full_capture")]
    pub confirm_full_capture: Option<bool>,
    /// Items Metadata-only capture adds, each listed once. An item requires the items whose information it already reveals: `tool_output_size`, `tool_names` and `conversation_shape` require `tool_usage`, and `input_size_by_role` requires `tool_output_size`. A set that leaves out a required item is rejected. When several Metadata-only overrides match a request, it gets only the items all of them select.
    #[serde(rename = "metadata_capture")]
    pub metadata_capture: Option<Vec<crate::datadogV2::model::AiGatewayMetadataCaptureItem>>,
    /// Capture mode for matching traffic.
    #[serde(rename = "mode")]
    pub mode: crate::datadogV2::model::AiGatewayCaptureUpdateRequestDataAttributesMode,
    /// Display name of the override. Names need not be unique; identify overrides by `id`.
    #[serde(rename = "name")]
    pub name: String,
    /// Proposed operation for preview. Write endpoints infer it from the HTTP method.
    #[serde(rename = "operation")]
    pub operation:
        Option<crate::datadogV2::model::AiGatewayCaptureUpdateRequestDataAttributesOperation>,
    /// Current override revision, required for replacement or deletion.
    #[serde(rename = "revision")]
    pub revision: i64,
    /// All selected filters combine with AND, with one value per filter. The `model` and `provider` tags apply to each actual direct or routed attempt, including fallback destinations. Routes are not selectable filters. At least one filter is required.
    /// If any matching override uses `metadata_only`, that mode applies. Otherwise, if any matching override uses `full_capture`, that mode applies. The organization default applies only when no override matches. Overrides have no priority or specificity ordering.
    #[serde(rename = "scope")]
    pub scope: Option<crate::datadogV2::model::AiGatewayCaptureScope>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayCaptureUpdateRequestDataAttributes {
    pub fn new(
        configuration_revision: String,
        mode: crate::datadogV2::model::AiGatewayCaptureUpdateRequestDataAttributesMode,
        name: String,
        revision: i64,
    ) -> AiGatewayCaptureUpdateRequestDataAttributes {
        AiGatewayCaptureUpdateRequestDataAttributes {
            configuration_revision,
            confirm_full_capture: None,
            metadata_capture: None,
            mode,
            name,
            operation: None,
            revision,
            scope: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn confirm_full_capture(mut self, value: bool) -> Self {
        self.confirm_full_capture = Some(value);
        self
    }

    pub fn metadata_capture(
        mut self,
        value: Vec<crate::datadogV2::model::AiGatewayMetadataCaptureItem>,
    ) -> Self {
        self.metadata_capture = Some(value);
        self
    }

    pub fn operation(
        mut self,
        value: crate::datadogV2::model::AiGatewayCaptureUpdateRequestDataAttributesOperation,
    ) -> Self {
        self.operation = Some(value);
        self
    }

    pub fn scope(mut self, value: crate::datadogV2::model::AiGatewayCaptureScope) -> Self {
        self.scope = Some(value);
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

impl<'de> Deserialize<'de> for AiGatewayCaptureUpdateRequestDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayCaptureUpdateRequestDataAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayCaptureUpdateRequestDataAttributesVisitor {
            type Value = AiGatewayCaptureUpdateRequestDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut configuration_revision: Option<String> = None;
                let mut confirm_full_capture: Option<bool> = None;
                let mut metadata_capture: Option<
                    Vec<crate::datadogV2::model::AiGatewayMetadataCaptureItem>,
                > = None;
                let mut mode: Option<
                    crate::datadogV2::model::AiGatewayCaptureUpdateRequestDataAttributesMode,
                > = None;
                let mut name: Option<String> = None;
                let mut operation: Option<
                    crate::datadogV2::model::AiGatewayCaptureUpdateRequestDataAttributesOperation,
                > = None;
                let mut revision: Option<i64> = None;
                let mut scope: Option<crate::datadogV2::model::AiGatewayCaptureScope> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "configuration_revision" => {
                            configuration_revision =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "confirm_full_capture" => {
                            if v.is_null() {
                                continue;
                            }
                            confirm_full_capture =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
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
                                    crate::datadogV2::model::AiGatewayCaptureUpdateRequestDataAttributesMode::UnparsedObject(_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "operation" => {
                            if v.is_null() {
                                continue;
                            }
                            operation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _operation) = operation {
                                match _operation {
                                    crate::datadogV2::model::AiGatewayCaptureUpdateRequestDataAttributesOperation::UnparsedObject(_operation) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "revision" => {
                            revision = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scope" => {
                            if v.is_null() {
                                continue;
                            }
                            scope = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let configuration_revision = configuration_revision
                    .ok_or_else(|| M::Error::missing_field("configuration_revision"))?;
                let mode = mode.ok_or_else(|| M::Error::missing_field("mode"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let revision = revision.ok_or_else(|| M::Error::missing_field("revision"))?;

                let content = AiGatewayCaptureUpdateRequestDataAttributes {
                    configuration_revision,
                    confirm_full_capture,
                    metadata_capture,
                    mode,
                    name,
                    operation,
                    revision,
                    scope,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayCaptureUpdateRequestDataAttributesVisitor)
    }
}
