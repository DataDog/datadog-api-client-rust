// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Case update status attributes
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CaseUpdateStatusAttributes {
    /// Deprecated way of representing the case status, which only supports OPEN, IN_PROGRESS, and CLOSED statuses. Use `status_name` instead.
    #[deprecated]
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::CaseStatus>,
    /// Status of the case. Must be one of the existing statuses for the case's type.
    #[serde(rename = "status_name")]
    pub status_name: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CaseUpdateStatusAttributes {
    pub fn new() -> CaseUpdateStatusAttributes {
        #[allow(deprecated)]
        CaseUpdateStatusAttributes {
            status: None,
            status_name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn status(mut self, value: crate::datadogV2::model::CaseStatus) -> Self {
        self.status = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn status_name(mut self, value: String) -> Self {
        self.status_name = Some(value);
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

impl Default for CaseUpdateStatusAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for CaseUpdateStatusAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CaseUpdateStatusAttributesVisitor;
        impl<'a> Visitor<'a> for CaseUpdateStatusAttributesVisitor {
            type Value = CaseUpdateStatusAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut status: Option<crate::datadogV2::model::CaseStatus> = None;
                let mut status_name: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::CaseStatus::UnparsedObject(
                                        _status,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "status_name" => {
                            if v.is_null() {
                                continue;
                            }
                            status_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                #[allow(deprecated)]
                let content = CaseUpdateStatusAttributes {
                    status,
                    status_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CaseUpdateStatusAttributesVisitor)
    }
}
