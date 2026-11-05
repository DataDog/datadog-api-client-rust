// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Saved data capture override with its request filters, capture mode, revision, and timestamps.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayCaptureOverride {
    /// Timestamp when the override was created.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// UUID identifying the override within the organization.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// Items Metadata-only capture adds, each listed once. An item requires the items whose information it already reveals: `tool_output_size`, `tool_names` and `conversation_shape` require `tool_usage`, and `input_size_by_role` requires `tool_output_size`. A set that leaves out a required item is rejected. When several Metadata-only overrides match a request, it gets only the items all of them select.
    #[serde(rename = "metadata_capture")]
    pub metadata_capture: Option<Vec<crate::datadogV2::model::AiGatewayMetadataCaptureItem>>,
    /// Capture mode for matching traffic.
    #[serde(rename = "mode")]
    pub mode: crate::datadogV2::model::AiGatewayCaptureOverrideMode,
    /// Display name of the override. Names need not be unique; identify overrides by `id`.
    #[serde(rename = "name")]
    pub name: String,
    /// Current override revision, required for replacement or deletion.
    #[serde(rename = "revision")]
    pub revision: i64,
    /// Visible request filters. Provider and model tags are withheld when the caller cannot view them. An empty object or empty tags can represent a fully restricted scope; scope_restricted identifies redaction.
    #[serde(rename = "scope")]
    pub scope: crate::datadogV2::model::AiGatewayCaptureResponseScope,
    /// Whether provider or model scope metadata was withheld. Omit unchanged scope fields on an unrelated update to preserve their saved values.
    #[serde(rename = "scope_restricted")]
    pub scope_restricted: bool,
    /// Timestamp when the override was last modified.
    #[serde(rename = "updated_at")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayCaptureOverride {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        id: uuid::Uuid,
        mode: crate::datadogV2::model::AiGatewayCaptureOverrideMode,
        name: String,
        revision: i64,
        scope: crate::datadogV2::model::AiGatewayCaptureResponseScope,
        scope_restricted: bool,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> AiGatewayCaptureOverride {
        AiGatewayCaptureOverride {
            created_at,
            id,
            metadata_capture: None,
            mode,
            name,
            revision,
            scope,
            scope_restricted,
            updated_at,
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

impl<'de> Deserialize<'de> for AiGatewayCaptureOverride {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayCaptureOverrideVisitor;
        impl<'a> Visitor<'a> for AiGatewayCaptureOverrideVisitor {
            type Value = AiGatewayCaptureOverride;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut id: Option<uuid::Uuid> = None;
                let mut metadata_capture: Option<
                    Vec<crate::datadogV2::model::AiGatewayMetadataCaptureItem>,
                > = None;
                let mut mode: Option<crate::datadogV2::model::AiGatewayCaptureOverrideMode> = None;
                let mut name: Option<String> = None;
                let mut revision: Option<i64> = None;
                let mut scope: Option<crate::datadogV2::model::AiGatewayCaptureResponseScope> =
                    None;
                let mut scope_restricted: Option<bool> = None;
                let mut updated_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV2::model::AiGatewayCaptureOverrideMode::UnparsedObject(_mode) => {
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
                        "updated_at" => {
                            updated_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let mode = mode.ok_or_else(|| M::Error::missing_field("mode"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let revision = revision.ok_or_else(|| M::Error::missing_field("revision"))?;
                let scope = scope.ok_or_else(|| M::Error::missing_field("scope"))?;
                let scope_restricted =
                    scope_restricted.ok_or_else(|| M::Error::missing_field("scope_restricted"))?;
                let updated_at = updated_at.ok_or_else(|| M::Error::missing_field("updated_at"))?;

                let content = AiGatewayCaptureOverride {
                    created_at,
                    id,
                    metadata_capture,
                    mode,
                    name,
                    revision,
                    scope,
                    scope_restricted,
                    updated_at,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayCaptureOverrideVisitor)
    }
}
