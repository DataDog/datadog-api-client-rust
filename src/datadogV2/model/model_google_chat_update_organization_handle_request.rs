// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Update organization handle request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GoogleChatUpdateOrganizationHandleRequest {
    /// Organization handle data for an update request.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::GoogleChatUpdateOrganizationHandleRequestData,
    /// Organization handle resource type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::GoogleChatOrganizationHandleType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GoogleChatUpdateOrganizationHandleRequest {
    pub fn new(
        data: crate::datadogV2::model::GoogleChatUpdateOrganizationHandleRequestData,
        type_: crate::datadogV2::model::GoogleChatOrganizationHandleType,
    ) -> GoogleChatUpdateOrganizationHandleRequest {
        GoogleChatUpdateOrganizationHandleRequest {
            data,
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

impl<'de> Deserialize<'de> for GoogleChatUpdateOrganizationHandleRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GoogleChatUpdateOrganizationHandleRequestVisitor;
        impl<'a> Visitor<'a> for GoogleChatUpdateOrganizationHandleRequestVisitor {
            type Value = GoogleChatUpdateOrganizationHandleRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<
                    crate::datadogV2::model::GoogleChatUpdateOrganizationHandleRequestData,
                > = None;
                let mut type_: Option<crate::datadogV2::model::GoogleChatOrganizationHandleType> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::GoogleChatOrganizationHandleType::UnparsedObject(_type_) => {
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
                let data = data.ok_or_else(|| M::Error::missing_field("data"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = GoogleChatUpdateOrganizationHandleRequest {
                    data,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GoogleChatUpdateOrganizationHandleRequestVisitor)
    }
}
