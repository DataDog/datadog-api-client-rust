// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// JSON:API protocol request resource.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsReplaceExperimentProtocolRequestData {
    /// Replace a published protocol template. Omitted optional template fields are cleared. Omitted enforcement and migration_metadata are preserved. Enabled limits require their values; allocation settings require environment_id.
    #[serde(rename = "attributes")]
    pub attributes:
        crate::datadogV2::model::ExperimentsReplaceExperimentProtocolRequestDataAttributes,
    /// Protocol ID. If supplied, must match the path protocol_id.
    #[serde(rename = "id")]
    pub id: Option<uuid::Uuid>,
    /// Protocols resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::ExperimentsPublicProtocolResponseDataType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsReplaceExperimentProtocolRequestData {
    pub fn new(
        attributes: crate::datadogV2::model::ExperimentsReplaceExperimentProtocolRequestDataAttributes,
        type_: crate::datadogV2::model::ExperimentsPublicProtocolResponseDataType,
    ) -> ExperimentsReplaceExperimentProtocolRequestData {
        ExperimentsReplaceExperimentProtocolRequestData {
            attributes,
            id: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn id(mut self, value: uuid::Uuid) -> Self {
        self.id = Some(value);
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

impl<'de> Deserialize<'de> for ExperimentsReplaceExperimentProtocolRequestData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsReplaceExperimentProtocolRequestDataVisitor;
        impl<'a> Visitor<'a> for ExperimentsReplaceExperimentProtocolRequestDataVisitor {
            type Value = ExperimentsReplaceExperimentProtocolRequestData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<crate::datadogV2::model::ExperimentsReplaceExperimentProtocolRequestDataAttributes> = None;
                let mut id: Option<uuid::Uuid> = None;
                let mut type_: Option<
                    crate::datadogV2::model::ExperimentsPublicProtocolResponseDataType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::ExperimentsPublicProtocolResponseDataType::UnparsedObject(_type_) => {
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
                let attributes = attributes.ok_or_else(|| M::Error::missing_field("attributes"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = ExperimentsReplaceExperimentProtocolRequestData {
                    attributes,
                    id,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsReplaceExperimentProtocolRequestDataVisitor)
    }
}
