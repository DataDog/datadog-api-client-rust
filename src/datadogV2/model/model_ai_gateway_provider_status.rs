// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Read-only. Whether the provider is working, based on the model sync and live calls. Omitted for
/// providers that are never checked, which include providers without an integration account, and
/// Azure OpenAI and Gemini providers until live traffic records a status.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AiGatewayProviderStatus {
    /// Time of the last observation.
    #[serde(rename = "checked_at")]
    pub checked_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Address of the models endpoint that the gateway called, for custom providers.
    #[serde(rename = "endpoint")]
    pub endpoint: Option<String>,
    /// Display text saying how to fix the provider. Clients render it as-is.
    #[serde(rename = "fix")]
    pub fix: Option<String>,
    /// HTTP status code that the provider returned.
    #[serde(rename = "http_status")]
    pub http_status: Option<i64>,
    /// Display text for the reason. Clients render it as-is.
    #[serde(rename = "label")]
    pub label: Option<String>,
    /// Time of the last observation in which the provider worked.
    #[serde(rename = "last_ok_at")]
    pub last_ok_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The provider's own error message, with credentials removed. Never set for custom providers.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// Why the provider is invalid, as a stable code such as `credentials_rejected`. Set only when `state` is `invalid`.
    #[serde(rename = "reason")]
    pub reason: Option<String>,
    /// Whether the provider is working. `checking` means no observation has been recorded yet, not that a
/// check is running.
    #[serde(rename = "state")]
    pub state: crate::datadogV2::model::AiGatewayProviderStatusState,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl AiGatewayProviderStatus {
    pub fn new(state: crate::datadogV2::model::AiGatewayProviderStatusState) -> AiGatewayProviderStatus {
        AiGatewayProviderStatus {
            checked_at: None,
            endpoint: None,
            fix: None,
            http_status: None,
            label: None,
            last_ok_at: None,
            message: None,
            reason: None,
            state,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }
    
    pub fn checked_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.checked_at = Some(value);
        self
    }
    
    pub fn endpoint(mut self, value: String) -> Self {
        self.endpoint = Some(value);
        self
    }
    
    pub fn fix(mut self, value: String) -> Self {
        self.fix = Some(value);
        self
    }
    
    pub fn http_status(mut self, value: i64) -> Self {
        self.http_status = Some(value);
        self
    }
    
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }
    
    pub fn last_ok_at(mut self, value: chrono::DateTime<chrono::Utc>) -> Self {
        self.last_ok_at = Some(value);
        self
    }
    
    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }
    
    pub fn reason(mut self, value: String) -> Self {
        self.reason = Some(value);
        self
    }
    
    pub fn additional_properties(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.additional_properties = value;
        self
    }
}



impl<'de> Deserialize<'de> for AiGatewayProviderStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AiGatewayProviderStatusVisitor;
        impl<'a> Visitor<'a> for AiGatewayProviderStatusVisitor {
            type Value = AiGatewayProviderStatus;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut checked_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut endpoint: Option<String> = None;
                let mut fix: Option<String> = None;
                let mut http_status: Option<i64> = None;
                let mut label: Option<String> = None;
                let mut last_ok_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut message: Option<String> = None;
                let mut reason: Option<String> = None;
                let mut state: Option<crate::datadogV2::model::AiGatewayProviderStatusState> = None;
                    let mut additional_properties: std::collections::BTreeMap<String, serde_json::Value> = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "checked_at" => {
                            if v.is_null() {
                                continue;
                            }
                            checked_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "endpoint" => {
                            if v.is_null() {
                                continue;
                            }
                            endpoint = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "fix" => {
                            if v.is_null() {
                                continue;
                            }
                            fix = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "http_status" => {
                            if v.is_null() {
                                continue;
                            }
                            http_status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "label" => {
                            if v.is_null() {
                                continue;
                            }
                            label = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "last_ok_at" => {
                            if v.is_null() {
                                continue;
                            }
                            last_ok_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "message" => {
                            if v.is_null() {
                                continue;
                            }
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "reason" => {
                            if v.is_null() {
                                continue;
                            }
                            reason = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "state" => {
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _state) = state {
                                match _state {
                                    crate::datadogV2::model::AiGatewayProviderStatusState::UnparsedObject(_state) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        },
                    }
                }
                let state = state.ok_or_else(|| M::Error::missing_field("state"))?;

                let content = AiGatewayProviderStatus {
                    checked_at,
                    endpoint,
                    fix,
                    http_status,
                    label,
                    last_ok_at,
                    message,
                    reason,
                    state,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(AiGatewayProviderStatusVisitor)
    }
}
