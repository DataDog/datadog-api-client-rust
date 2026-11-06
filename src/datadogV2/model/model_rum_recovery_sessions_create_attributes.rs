// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Recover 1 to 100 explicit archived sessions. Session IDs must be unique within the request.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RumRecoverySessionsCreateAttributes {
    /// Targeted recovery. Omitted request_type defaults to sessions.
    #[serde(rename = "request_type")]
    pub request_type: Option<crate::datadogV2::model::RumRecoverySessionsRequestType>,
    /// Sessions to recover.
    #[serde(rename = "sessions")]
    pub sessions: Vec<crate::datadogV2::model::RumRecoverySessionInput>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RumRecoverySessionsCreateAttributes {
    pub fn new(
        sessions: Vec<crate::datadogV2::model::RumRecoverySessionInput>,
    ) -> RumRecoverySessionsCreateAttributes {
        RumRecoverySessionsCreateAttributes {
            request_type: None,
            sessions,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn request_type(
        mut self,
        value: crate::datadogV2::model::RumRecoverySessionsRequestType,
    ) -> Self {
        self.request_type = Some(value);
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

impl<'de> Deserialize<'de> for RumRecoverySessionsCreateAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RumRecoverySessionsCreateAttributesVisitor;
        impl<'a> Visitor<'a> for RumRecoverySessionsCreateAttributesVisitor {
            type Value = RumRecoverySessionsCreateAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut request_type: Option<
                    crate::datadogV2::model::RumRecoverySessionsRequestType,
                > = None;
                let mut sessions: Option<Vec<crate::datadogV2::model::RumRecoverySessionInput>> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "request_type" => {
                            if v.is_null() {
                                continue;
                            }
                            request_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _request_type) = request_type {
                                match _request_type {
                                    crate::datadogV2::model::RumRecoverySessionsRequestType::UnparsedObject(_request_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "sessions" => {
                            sessions = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let sessions = sessions.ok_or_else(|| M::Error::missing_field("sessions"))?;

                let content = RumRecoverySessionsCreateAttributes {
                    request_type,
                    sessions,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RumRecoverySessionsCreateAttributesVisitor)
    }
}
