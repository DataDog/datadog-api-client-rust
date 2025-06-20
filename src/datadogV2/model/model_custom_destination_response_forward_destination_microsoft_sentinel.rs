// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The Microsoft Sentinel destination.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomDestinationResponseForwardDestinationMicrosoftSentinel {
    /// Client ID from the Datadog Azure integration.
    #[serde(rename = "client_id")]
    pub client_id: String,
    /// Azure data collection endpoint.
    #[serde(rename = "data_collection_endpoint")]
    pub data_collection_endpoint: String,
    /// Azure data collection rule ID.
    #[serde(rename = "data_collection_rule_id")]
    pub data_collection_rule_id: String,
    /// Azure stream name.
    #[serde(rename = "stream_name")]
    pub stream_name: String,
    /// Tenant ID from the Datadog Azure integration.
    #[serde(rename = "tenant_id")]
    pub tenant_id: String,
    /// Type of the Microsoft Sentinel destination.
    #[serde(rename = "type")]
    pub type_:
        crate::datadogV2::model::CustomDestinationResponseForwardDestinationMicrosoftSentinelType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomDestinationResponseForwardDestinationMicrosoftSentinel {
    pub fn new(
        client_id: String,
        data_collection_endpoint: String,
        data_collection_rule_id: String,
        stream_name: String,
        tenant_id: String,
        type_: crate::datadogV2::model::CustomDestinationResponseForwardDestinationMicrosoftSentinelType,
    ) -> CustomDestinationResponseForwardDestinationMicrosoftSentinel {
        CustomDestinationResponseForwardDestinationMicrosoftSentinel {
            client_id,
            data_collection_endpoint,
            data_collection_rule_id,
            stream_name,
            tenant_id,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for CustomDestinationResponseForwardDestinationMicrosoftSentinel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomDestinationResponseForwardDestinationMicrosoftSentinelVisitor;
        impl<'a> Visitor<'a> for CustomDestinationResponseForwardDestinationMicrosoftSentinelVisitor {
            type Value = CustomDestinationResponseForwardDestinationMicrosoftSentinel;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut client_id: Option<String> = None;
                let mut data_collection_endpoint: Option<String> = None;
                let mut data_collection_rule_id: Option<String> = None;
                let mut stream_name: Option<String> = None;
                let mut tenant_id: Option<String> = None;
                let mut type_: Option<crate::datadogV2::model::CustomDestinationResponseForwardDestinationMicrosoftSentinelType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "client_id" => {
                            client_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_collection_endpoint" => {
                            data_collection_endpoint =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_collection_rule_id" => {
                            data_collection_rule_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "stream_name" => {
                            stream_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tenant_id" => {
                            tenant_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::CustomDestinationResponseForwardDestinationMicrosoftSentinelType::UnparsedObject(_type_) => {
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
                let data_collection_endpoint = data_collection_endpoint
                    .ok_or_else(|| M::Error::missing_field("data_collection_endpoint"))?;
                let data_collection_rule_id = data_collection_rule_id
                    .ok_or_else(|| M::Error::missing_field("data_collection_rule_id"))?;
                let stream_name =
                    stream_name.ok_or_else(|| M::Error::missing_field("stream_name"))?;
                let tenant_id = tenant_id.ok_or_else(|| M::Error::missing_field("tenant_id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = CustomDestinationResponseForwardDestinationMicrosoftSentinel {
                    client_id,
                    data_collection_endpoint,
                    data_collection_rule_id,
                    stream_name,
                    tenant_id,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer
            .deserialize_any(CustomDestinationResponseForwardDestinationMicrosoftSentinelVisitor)
    }
}
