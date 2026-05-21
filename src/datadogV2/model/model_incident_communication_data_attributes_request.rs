// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes for creating or updating a communication.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IncidentCommunicationDataAttributesRequest {
    /// The kind of communication.
    #[serde(rename = "communication_type")]
    pub communication_type: crate::datadogV2::model::IncidentCommunicationKind,
    /// The content of a communication.
    #[serde(rename = "content")]
    pub content: crate::datadogV2::model::IncidentCommunicationContent,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IncidentCommunicationDataAttributesRequest {
    pub fn new(
        communication_type: crate::datadogV2::model::IncidentCommunicationKind,
        content: crate::datadogV2::model::IncidentCommunicationContent,
    ) -> IncidentCommunicationDataAttributesRequest {
        IncidentCommunicationDataAttributesRequest {
            communication_type,
            content,
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

impl<'de> Deserialize<'de> for IncidentCommunicationDataAttributesRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IncidentCommunicationDataAttributesRequestVisitor;
        impl<'a> Visitor<'a> for IncidentCommunicationDataAttributesRequestVisitor {
            type Value = IncidentCommunicationDataAttributesRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut communication_type: Option<
                    crate::datadogV2::model::IncidentCommunicationKind,
                > = None;
                let mut content: Option<crate::datadogV2::model::IncidentCommunicationContent> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "communication_type" => {
                            communication_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _communication_type) = communication_type {
                                match _communication_type {
                                    crate::datadogV2::model::IncidentCommunicationKind::UnparsedObject(_communication_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "content" => {
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let communication_type = communication_type
                    .ok_or_else(|| M::Error::missing_field("communication_type"))?;
                let content = content.ok_or_else(|| M::Error::missing_field("content"))?;

                let content = IncidentCommunicationDataAttributesRequest {
                    communication_type,
                    content,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IncidentCommunicationDataAttributesRequestVisitor)
    }
}
