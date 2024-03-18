// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response containing date and type for specified custom reports.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageSpecifiedCustomReportsData {
    /// The response containing attributes for specified custom reports.
    #[serde(rename = "attributes")]
    pub attributes: Option<crate::datadogV1::model::UsageSpecifiedCustomReportsAttributes>,
    /// The date for specified custom reports.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The type of reports.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::UsageReportsType>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageSpecifiedCustomReportsData {
    pub fn new() -> UsageSpecifiedCustomReportsData {
        UsageSpecifiedCustomReportsData {
            attributes: None,
            id: None,
            type_: None,
            _unparsed: false,
        }
    }

    pub fn attributes(
        mut self,
        value: crate::datadogV1::model::UsageSpecifiedCustomReportsAttributes,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV1::model::UsageReportsType) -> Self {
        self.type_ = Some(value);
        self
    }
}

impl Default for UsageSpecifiedCustomReportsData {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageSpecifiedCustomReportsData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageSpecifiedCustomReportsDataVisitor;
        impl<'a> Visitor<'a> for UsageSpecifiedCustomReportsDataVisitor {
            type Value = UsageSpecifiedCustomReportsData;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut attributes: Option<
                    crate::datadogV1::model::UsageSpecifiedCustomReportsAttributes,
                > = None;
                let mut id: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::UsageReportsType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "attributes" => {
                            if v.is_null() {
                                continue;
                            }
                            attributes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::UsageReportsType::UnparsedObject(
                                        _type_,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {}
                    }
                }

                let content = UsageSpecifiedCustomReportsData {
                    attributes,
                    id,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageSpecifiedCustomReportsDataVisitor)
    }
}
