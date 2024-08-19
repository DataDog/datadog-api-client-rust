// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Custom header access authentication.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CustomDestinationHttpDestinationAuthCustomHeader {
    /// The header name of the authentication.
    #[serde(rename = "header_name")]
    pub header_name: String,
    /// The header value of the authentication. This field is not returned by the API.
    #[serde(rename = "header_value")]
    pub header_value: String,
    /// Type of the custom header access authentication.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::CustomDestinationHttpDestinationAuthCustomHeaderType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CustomDestinationHttpDestinationAuthCustomHeader {
    pub fn new(
        header_name: String,
        header_value: String,
        type_: crate::datadogV2::model::CustomDestinationHttpDestinationAuthCustomHeaderType,
    ) -> CustomDestinationHttpDestinationAuthCustomHeader {
        CustomDestinationHttpDestinationAuthCustomHeader {
            header_name,
            header_value,
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

impl<'de> Deserialize<'de> for CustomDestinationHttpDestinationAuthCustomHeader {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CustomDestinationHttpDestinationAuthCustomHeaderVisitor;
        impl<'a> Visitor<'a> for CustomDestinationHttpDestinationAuthCustomHeaderVisitor {
            type Value = CustomDestinationHttpDestinationAuthCustomHeader;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut header_name: Option<String> = None;
                let mut header_value: Option<String> = None;
                let mut type_: Option<
                    crate::datadogV2::model::CustomDestinationHttpDestinationAuthCustomHeaderType,
                > = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "header_name" => {
                            header_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "header_value" => {
                            header_value =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::CustomDestinationHttpDestinationAuthCustomHeaderType::UnparsedObject(_type_) => {
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
                let header_name =
                    header_name.ok_or_else(|| M::Error::missing_field("header_name"))?;
                let header_value =
                    header_value.ok_or_else(|| M::Error::missing_field("header_value"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = CustomDestinationHttpDestinationAuthCustomHeader {
                    header_name,
                    header_value,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CustomDestinationHttpDestinationAuthCustomHeaderVisitor)
    }
}
