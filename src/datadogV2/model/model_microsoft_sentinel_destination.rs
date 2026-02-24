// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `microsoft_sentinel` destination forwards logs to Microsoft Sentinel.
///
/// **Supported pipeline types:** logs
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MicrosoftSentinelDestination {
    /// Configuration for buffer settings on destination components.
    #[serde(rename = "buffer")]
    pub buffer: Option<crate::datadogV2::model::ObservabilityPipelineBufferOptions>,
    /// Azure AD client ID used for authentication.
    #[serde(rename = "client_id")]
    pub client_id: String,
    /// Name of the environment variable or secret that holds the Azure AD client secret.
    #[serde(rename = "client_secret_key")]
    pub client_secret_key: Option<String>,
    /// Name of the environment variable or secret that holds the Data Collection Endpoint (DCE) URI.
    #[serde(rename = "dce_uri_key")]
    pub dce_uri_key: Option<String>,
    /// The immutable ID of the Data Collection Rule (DCR).
    #[serde(rename = "dcr_immutable_id")]
    pub dcr_immutable_id: String,
    /// The unique identifier for this component.
    #[serde(rename = "id")]
    pub id: String,
    /// A list of component IDs whose output is used as the `input` for this component.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// The name of the Log Analytics table where logs are sent.
    #[serde(rename = "table")]
    pub table: String,
    /// Azure AD tenant ID.
    #[serde(rename = "tenant_id")]
    pub tenant_id: String,
    /// The destination type. The value should always be `microsoft_sentinel`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::MicrosoftSentinelDestinationType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MicrosoftSentinelDestination {
    pub fn new(
        client_id: String,
        dcr_immutable_id: String,
        id: String,
        inputs: Vec<String>,
        table: String,
        tenant_id: String,
        type_: crate::datadogV2::model::MicrosoftSentinelDestinationType,
    ) -> MicrosoftSentinelDestination {
        MicrosoftSentinelDestination {
            buffer: None,
            client_id,
            client_secret_key: None,
            dce_uri_key: None,
            dcr_immutable_id,
            id,
            inputs,
            table,
            tenant_id,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn buffer(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineBufferOptions,
    ) -> Self {
        self.buffer = Some(value);
        self
    }

    pub fn client_secret_key(mut self, value: String) -> Self {
        self.client_secret_key = Some(value);
        self
    }

    pub fn dce_uri_key(mut self, value: String) -> Self {
        self.dce_uri_key = Some(value);
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

impl<'de> Deserialize<'de> for MicrosoftSentinelDestination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MicrosoftSentinelDestinationVisitor;
        impl<'a> Visitor<'a> for MicrosoftSentinelDestinationVisitor {
            type Value = MicrosoftSentinelDestination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut buffer: Option<
                    crate::datadogV2::model::ObservabilityPipelineBufferOptions,
                > = None;
                let mut client_id: Option<String> = None;
                let mut client_secret_key: Option<String> = None;
                let mut dce_uri_key: Option<String> = None;
                let mut dcr_immutable_id: Option<String> = None;
                let mut id: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut table: Option<String> = None;
                let mut tenant_id: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::MicrosoftSentinelDestinationType> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "buffer" => {
                            if v.is_null() {
                                continue;
                            }
                            buffer = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _buffer) = buffer {
                                match _buffer {
                                    crate::datadogV2::model::ObservabilityPipelineBufferOptions::UnparsedObject(_buffer) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "client_id" => {
                            client_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "client_secret_key" => {
                            if v.is_null() {
                                continue;
                            }
                            client_secret_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dce_uri_key" => {
                            if v.is_null() {
                                continue;
                            }
                            dce_uri_key =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dcr_immutable_id" => {
                            dcr_immutable_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputs" => {
                            inputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "table" => {
                            table = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tenant_id" => {
                            tenant_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::MicrosoftSentinelDestinationType::UnparsedObject(_type_) => {
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
                let client_id = client_id.ok_or_else(|| M::Error::missing_field("client_id"))?;
                let dcr_immutable_id =
                    dcr_immutable_id.ok_or_else(|| M::Error::missing_field("dcr_immutable_id"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;
                let table = table.ok_or_else(|| M::Error::missing_field("table"))?;
                let tenant_id = tenant_id.ok_or_else(|| M::Error::missing_field("tenant_id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = MicrosoftSentinelDestination {
                    buffer,
                    client_id,
                    client_secret_key,
                    dce_uri_key,
                    dcr_immutable_id,
                    id,
                    inputs,
                    table,
                    tenant_id,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MicrosoftSentinelDestinationVisitor)
    }
}
