// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PickActionRequest {
    /// The client type for action filtering.
    #[serde(rename = "client")]
    pub client: Option<crate::datadogV2::model::ClientType>,
    /// The number of relevant actions to return.
    #[serde(rename = "number_of_relevant_actions")]
    pub number_of_relevant_actions: i64,
    /// The stability level for action filtering.
    #[serde(rename = "stability")]
    pub stability: Option<crate::datadogV2::model::StabilityLevel>,
    /// The user's prompt to find relevant actions.
    #[serde(rename = "user_prompt")]
    pub user_prompt: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PickActionRequest {
    pub fn new(number_of_relevant_actions: i64, user_prompt: String) -> PickActionRequest {
        PickActionRequest {
            client: None,
            number_of_relevant_actions,
            stability: None,
            user_prompt,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn client(mut self, value: crate::datadogV2::model::ClientType) -> Self {
        self.client = Some(value);
        self
    }

    pub fn stability(mut self, value: crate::datadogV2::model::StabilityLevel) -> Self {
        self.stability = Some(value);
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

impl<'de> Deserialize<'de> for PickActionRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PickActionRequestVisitor;
        impl<'a> Visitor<'a> for PickActionRequestVisitor {
            type Value = PickActionRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut client: Option<crate::datadogV2::model::ClientType> = None;
                let mut number_of_relevant_actions: Option<i64> = None;
                let mut stability: Option<crate::datadogV2::model::StabilityLevel> = None;
                let mut user_prompt: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "client" => {
                            if v.is_null() {
                                continue;
                            }
                            client = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _client) = client {
                                match _client {
                                    crate::datadogV2::model::ClientType::UnparsedObject(
                                        _client,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "number_of_relevant_actions" => {
                            number_of_relevant_actions =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "stability" => {
                            if v.is_null() {
                                continue;
                            }
                            stability = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _stability) = stability {
                                match _stability {
                                    crate::datadogV2::model::StabilityLevel::UnparsedObject(
                                        _stability,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "user_prompt" => {
                            user_prompt =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let number_of_relevant_actions = number_of_relevant_actions
                    .ok_or_else(|| M::Error::missing_field("number_of_relevant_actions"))?;
                let user_prompt =
                    user_prompt.ok_or_else(|| M::Error::missing_field("user_prompt"))?;

                let content = PickActionRequest {
                    client,
                    number_of_relevant_actions,
                    stability,
                    user_prompt,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PickActionRequestVisitor)
    }
}
