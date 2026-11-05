// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Organization default mode, scoped overrides, and current configuration revision.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayCaptureConfigurationResponseDataAttributes {
    /// Opaque revision of the organization default and all overrides. Writes require the revision returned by a current read or preview.
    #[serde(rename = "configuration_revision")]
    pub configuration_revision: String,
    /// Items Metadata-only capture adds, each listed once. An item requires the items whose information it already reveals: `tool_output_size`, `tool_names` and `conversation_shape` require `tool_usage`, and `input_size_by_role` requires `tool_output_size`. A set that leaves out a required item is rejected. When several Metadata-only overrides match a request, it gets only the items all of them select.
    #[serde(rename = "default_metadata_capture")]
    pub default_metadata_capture:
        Option<Vec<crate::datadogV2::model::AiGatewayMetadataCaptureItem>>,
    /// Default capture mode for the organization. Change it in **Org defaults**.
    #[serde(rename = "default_mode")]
    pub default_mode:
        crate::datadogV2::model::AiGatewayCaptureConfigurationResponseDataAttributesDefaultMode,
    /// Scoped overrides currently configured for the organization.
    #[serde(rename = "overrides")]
    pub overrides: Vec<crate::datadogV2::model::AiGatewayCaptureOverride>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl AiGatewayCaptureConfigurationResponseDataAttributes {
    pub fn new(
        configuration_revision: String,
        default_mode: crate::datadogV2::model::AiGatewayCaptureConfigurationResponseDataAttributesDefaultMode,
        overrides: Vec<crate::datadogV2::model::AiGatewayCaptureOverride>,
    ) -> AiGatewayCaptureConfigurationResponseDataAttributes {
        AiGatewayCaptureConfigurationResponseDataAttributes {
            configuration_revision,
            default_metadata_capture: None,
            default_mode,
            overrides,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn default_metadata_capture(
        mut self,
        value: Vec<crate::datadogV2::model::AiGatewayMetadataCaptureItem>,
    ) -> Self {
        self.default_metadata_capture = Some(value);
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

impl<'de> Deserialize<'de> for AiGatewayCaptureConfigurationResponseDataAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayCaptureConfigurationResponseDataAttributesVisitor;
        impl<'a> Visitor<'a> for AiGatewayCaptureConfigurationResponseDataAttributesVisitor {
            type Value = AiGatewayCaptureConfigurationResponseDataAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut configuration_revision: Option<String> = None;
                let mut default_metadata_capture: Option<
                    Vec<crate::datadogV2::model::AiGatewayMetadataCaptureItem>,
                > = None;
                let mut default_mode: Option<crate::datadogV2::model::AiGatewayCaptureConfigurationResponseDataAttributesDefaultMode> = None;
                let mut overrides: Option<Vec<crate::datadogV2::model::AiGatewayCaptureOverride>> =
                    None;
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
                        "default_metadata_capture" => {
                            if v.is_null() {
                                continue;
                            }
                            default_metadata_capture =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "default_mode" => {
                            default_mode =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _default_mode) = default_mode {
                                match _default_mode {
                                    crate::datadogV2::model::AiGatewayCaptureConfigurationResponseDataAttributesDefaultMode::UnparsedObject(_default_mode) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "overrides" => {
                            overrides = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let default_mode =
                    default_mode.ok_or_else(|| M::Error::missing_field("default_mode"))?;
                let overrides = overrides.ok_or_else(|| M::Error::missing_field("overrides"))?;

                let content = AiGatewayCaptureConfigurationResponseDataAttributes {
                    configuration_revision,
                    default_metadata_capture,
                    default_mode,
                    overrides,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayCaptureConfigurationResponseDataAttributesVisitor)
    }
}
