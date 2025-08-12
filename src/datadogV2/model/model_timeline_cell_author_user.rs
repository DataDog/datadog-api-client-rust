// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// timeline cell user author
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TimelineCellAuthorUser {
    /// user author content.
    #[serde(rename = "content")]
    pub content: Option<crate::datadogV2::model::TimelineCellAuthorUserContent>,
    /// user author type.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV2::model::TimelineCellAuthorUserType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TimelineCellAuthorUser {
    pub fn new() -> TimelineCellAuthorUser {
        TimelineCellAuthorUser {
            content: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn content(
        mut self,
        value: crate::datadogV2::model::TimelineCellAuthorUserContent,
    ) -> Self {
        self.content = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV2::model::TimelineCellAuthorUserType) -> Self {
        self.type_ = Some(value);
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

impl Default for TimelineCellAuthorUser {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for TimelineCellAuthorUser {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimelineCellAuthorUserVisitor;
        impl<'a> Visitor<'a> for TimelineCellAuthorUserVisitor {
            type Value = TimelineCellAuthorUser;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut content: Option<crate::datadogV2::model::TimelineCellAuthorUserContent> =
                    None;
                let mut type_: Option<crate::datadogV2::model::TimelineCellAuthorUserType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "content" => {
                            if v.is_null() {
                                continue;
                            }
                            content = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::TimelineCellAuthorUserType::UnparsedObject(_type_) => {
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

                let content = TimelineCellAuthorUser {
                    content,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimelineCellAuthorUserVisitor)
    }
}
