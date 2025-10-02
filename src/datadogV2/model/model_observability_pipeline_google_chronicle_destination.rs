// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `google_chronicle` destination sends logs to Google Chronicle.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineGoogleChronicleDestination {
    /// GCP credentials used to authenticate with Google Cloud Storage.
    #[serde(rename = "auth")]
    pub auth: crate::datadogV2::model::ObservabilityPipelineGcpAuth,
    /// The Google Chronicle customer ID.
    #[serde(rename = "customer_id")]
    pub customer_id: String,
    /// The encoding format for the logs sent to Chronicle.
    #[serde(rename = "encoding")]
    pub encoding:
        Option<crate::datadogV2::model::ObservabilityPipelineGoogleChronicleDestinationEncoding>,
    /// The unique identifier for this component.
    #[serde(rename = "id")]
    pub id: String,
    /// A list of component IDs whose output is used as the `input` for this component.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// The log type metadata associated with the Chronicle destination.
    #[serde(rename = "log_type")]
    pub log_type: Option<String>,
    /// The destination type. The value should always be `google_chronicle`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineGoogleChronicleDestinationType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ObservabilityPipelineGoogleChronicleDestination {
    pub fn new(
        auth: crate::datadogV2::model::ObservabilityPipelineGcpAuth,
        customer_id: String,
        id: String,
        inputs: Vec<String>,
        type_: crate::datadogV2::model::ObservabilityPipelineGoogleChronicleDestinationType,
    ) -> ObservabilityPipelineGoogleChronicleDestination {
        ObservabilityPipelineGoogleChronicleDestination {
            auth,
            customer_id,
            encoding: None,
            id,
            inputs,
            log_type: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn encoding(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineGoogleChronicleDestinationEncoding,
    ) -> Self {
        self.encoding = Some(value);
        self
    }

    pub fn log_type(mut self, value: String) -> Self {
        self.log_type = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineGoogleChronicleDestination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineGoogleChronicleDestinationVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineGoogleChronicleDestinationVisitor {
            type Value = ObservabilityPipelineGoogleChronicleDestination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut auth: Option<crate::datadogV2::model::ObservabilityPipelineGcpAuth> = None;
                let mut customer_id: Option<String> = None;
                let mut encoding: Option<crate::datadogV2::model::ObservabilityPipelineGoogleChronicleDestinationEncoding> = None;
                let mut id: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut log_type: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineGoogleChronicleDestinationType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "auth" => {
                            auth = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "customer_id" => {
                            customer_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "encoding" => {
                            if v.is_null() {
                                continue;
                            }
                            encoding = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _encoding) = encoding {
                                match _encoding {
                                    crate::datadogV2::model::ObservabilityPipelineGoogleChronicleDestinationEncoding::UnparsedObject(_encoding) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputs" => {
                            inputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "log_type" => {
                            if v.is_null() {
                                continue;
                            }
                            log_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineGoogleChronicleDestinationType::UnparsedObject(_type_) => {
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
                let auth = auth.ok_or_else(|| M::Error::missing_field("auth"))?;
                let customer_id =
                    customer_id.ok_or_else(|| M::Error::missing_field("customer_id"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineGoogleChronicleDestination {
                    auth,
                    customer_id,
                    encoding,
                    id,
                    inputs,
                    log_type,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineGoogleChronicleDestinationVisitor)
    }
}
