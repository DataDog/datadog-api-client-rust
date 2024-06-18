// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Data envelope for `ListAPIsResponse`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ListAPIsResponseData {
    /// Attributes for `ListAPIsResponseData`.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV2::model::ListAPIsResponseDataAttributes>,
    /// API identifier.
    #[serde(rename = "id")]
    pub id: Option<uuid::Uuid>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ListAPIsResponseData {
    pub fn new() -> ListAPIsResponseData {
        ListAPIsResponseData {
            attributes: None,
            id: None,
            _unparsed: false,
        }
    }
    
    pub fn attributes(mut self, value: crate::datadogV2::model::ListAPIsResponseDataAttributes) -> Self {
        self.attributes = Some(value);
        self
    }
    
    pub fn id(mut self, value: uuid::Uuid) -> Self {
        self.id = Some(value);
        self
    }
    
}


impl Default for ListAPIsResponseData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ListAPIsResponseData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ListAPIsResponseDataVisitor;
        impl<'a> Visitor<'a> for ListAPIsResponseDataVisitor {
            type Value = ListAPIsResponseData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<crate::datadogV2::model::ListAPIsResponseDataAttributes> = None;
                let mut id: Option<uuid::Uuid> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        &_ => {
                        },
                    }
                }

                
                let content = ListAPIsResponseData {
                    attributes,
                    id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ListAPIsResponseDataVisitor)
    }
}
