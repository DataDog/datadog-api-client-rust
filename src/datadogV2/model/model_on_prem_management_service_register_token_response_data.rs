// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Data for the registered token.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct OnPremManagementServiceRegisterTokenResponseData {
    /// Attributes for the registered token.
    #[serde(rename = "attributes")]
    pub attributes: crate::datadogV2::model::OnPremManagementServiceRegisterTokenResponseAttributes,
    /// The token identifier.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// The type of the resource. The value should always be tokenDefinitions.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::OnPremManagementServiceRegisterTokenResponseType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl OnPremManagementServiceRegisterTokenResponseData {
    pub fn new(
        attributes: crate::datadogV2::model::OnPremManagementServiceRegisterTokenResponseAttributes,
        id: uuid::Uuid,
        type_: crate::datadogV2::model::OnPremManagementServiceRegisterTokenResponseType,
    ) -> OnPremManagementServiceRegisterTokenResponseData {
        OnPremManagementServiceRegisterTokenResponseData {
            attributes,
            id,
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

impl<'de> Deserialize<'de> for OnPremManagementServiceRegisterTokenResponseData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OnPremManagementServiceRegisterTokenResponseDataVisitor;
        impl<'a> Visitor<'a> for OnPremManagementServiceRegisterTokenResponseDataVisitor {
            type Value = OnPremManagementServiceRegisterTokenResponseData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV2::model::OnPremManagementServiceRegisterTokenResponseAttributes,
                > = None;
                let mut id: Option<uuid::Uuid> = None;
                let mut type_: Option<
                    crate::datadogV2::model::OnPremManagementServiceRegisterTokenResponseType,
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
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::OnPremManagementServiceRegisterTokenResponseType::UnparsedObject(_type_) => {
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
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = OnPremManagementServiceRegisterTokenResponseData {
                    attributes,
                    id,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(OnPremManagementServiceRegisterTokenResponseDataVisitor)
    }
}
