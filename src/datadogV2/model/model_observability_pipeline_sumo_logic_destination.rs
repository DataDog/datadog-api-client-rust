// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The `sumo_logic` destination forwards logs to Sumo Logic.
///
/// **Supported pipeline types:** logs
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ObservabilityPipelineSumoLogicDestination {
    /// The output encoding format.
    #[serde(rename = "encoding")]
    pub encoding: Option<crate::datadogV2::model::ObservabilityPipelineSumoLogicDestinationEncoding>,
    /// A list of custom headers to include in the request to Sumo Logic.
    #[serde(rename = "header_custom_fields")]
    pub header_custom_fields: Option<Vec<crate::datadogV2::model::ObservabilityPipelineSumoLogicDestinationHeaderCustomFieldsItem>>,
    /// Optional override for the host name header.
    #[serde(rename = "header_host_name")]
    pub header_host_name: Option<String>,
    /// Optional override for the source category header.
    #[serde(rename = "header_source_category")]
    pub header_source_category: Option<String>,
    /// Optional override for the source name header.
    #[serde(rename = "header_source_name")]
    pub header_source_name: Option<String>,
    /// The unique identifier for this component.
    #[serde(rename = "id")]
    pub id: String,
    /// A list of component IDs whose output is used as the `input` for this component.
    #[serde(rename = "inputs")]
    pub inputs: Vec<String>,
    /// The destination type. The value should always be `sumo_logic`.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ObservabilityPipelineSumoLogicDestinationType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ObservabilityPipelineSumoLogicDestination {
    pub fn new(
        id: String,
        inputs: Vec<String>,
        type_: crate::datadogV2::model::ObservabilityPipelineSumoLogicDestinationType,
    ) -> ObservabilityPipelineSumoLogicDestination {
        ObservabilityPipelineSumoLogicDestination {
            encoding: None,
            header_custom_fields: None,
            header_host_name: None,
            header_source_category: None,
            header_source_name: None,
            id,
            inputs,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn encoding(
        mut self,
        value: crate::datadogV2::model::ObservabilityPipelineSumoLogicDestinationEncoding,
    ) -> Self {
        self.encoding = Some(value);
        self
    }

    pub fn header_custom_fields(
        mut self,
        value: Vec<crate::datadogV2::model::ObservabilityPipelineSumoLogicDestinationHeaderCustomFieldsItem>,
    ) -> Self {
        self.header_custom_fields = Some(value);
        self
    }

    pub fn header_host_name(mut self, value: String) -> Self {
        self.header_host_name = Some(value);
        self
    }

    pub fn header_source_category(mut self, value: String) -> Self {
        self.header_source_category = Some(value);
        self
    }

    pub fn header_source_name(mut self, value: String) -> Self {
        self.header_source_name = Some(value);
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

impl<'de> Deserialize<'de> for ObservabilityPipelineSumoLogicDestination {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ObservabilityPipelineSumoLogicDestinationVisitor;
        impl<'a> Visitor<'a> for ObservabilityPipelineSumoLogicDestinationVisitor {
            type Value = ObservabilityPipelineSumoLogicDestination;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut encoding: Option<
                    crate::datadogV2::model::ObservabilityPipelineSumoLogicDestinationEncoding,
                > = None;
                let mut header_custom_fields: Option<Vec<crate::datadogV2::model::ObservabilityPipelineSumoLogicDestinationHeaderCustomFieldsItem>> = None;
                let mut header_host_name: Option<String> = None;
                let mut header_source_category: Option<String> = None;
                let mut header_source_name: Option<String> = None;
                let mut id: Option<String> = None;
                let mut inputs: Option<Vec<String>> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ObservabilityPipelineSumoLogicDestinationType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "encoding" => {
                            if v.is_null() {
                                continue;
                            }
                            encoding = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _encoding) = encoding {
                                match _encoding {
                                    crate::datadogV2::model::ObservabilityPipelineSumoLogicDestinationEncoding::UnparsedObject(_encoding) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "header_custom_fields" => {
                            if v.is_null() {
                                continue;
                            }
                            header_custom_fields =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "header_host_name" => {
                            if v.is_null() {
                                continue;
                            }
                            header_host_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "header_source_category" => {
                            if v.is_null() {
                                continue;
                            }
                            header_source_category =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "header_source_name" => {
                            if v.is_null() {
                                continue;
                            }
                            header_source_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inputs" => {
                            inputs = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ObservabilityPipelineSumoLogicDestinationType::UnparsedObject(_type_) => {
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
                let inputs = inputs.ok_or_else(|| M::Error::missing_field("inputs"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ObservabilityPipelineSumoLogicDestination {
                    encoding,
                    header_custom_fields,
                    header_host_name,
                    header_source_category,
                    header_source_name,
                    id,
                    inputs,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ObservabilityPipelineSumoLogicDestinationVisitor)
    }
}
