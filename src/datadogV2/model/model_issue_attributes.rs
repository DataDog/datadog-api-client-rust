// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the information of an issue.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IssueAttributes {
    /// Error message associated with the issue.
    #[serde(rename = "error_message")]
    pub error_message: Option<String>,
    /// Type of the error that matches the issue.
    #[serde(rename = "error_type")]
    pub error_type: Option<String>,
    /// Path of the file where the issue occurred.
    #[serde(rename = "file_path")]
    pub file_path: Option<String>,
    /// Timestamp of the first seen error in milliseconds since the Unix epoch.
    #[serde(rename = "first_seen")]
    pub first_seen: Option<i64>,
    /// The application version (for example, git commit hash) where the issue was first observed.
    #[serde(rename = "first_seen_version")]
    pub first_seen_version: Option<String>,
    /// Name of the function where the issue occurred.
    #[serde(rename = "function_name")]
    pub function_name: Option<String>,
    /// Error is a crash.
    #[serde(rename = "is_crash")]
    pub is_crash: Option<bool>,
    /// Array of programming languages associated with the issue.
    #[serde(rename = "languages")]
    pub languages: Option<Vec<crate::datadogV2::model::IssueLanguage>>,
    /// Timestamp of the last seen error in milliseconds since the Unix epoch.
    #[serde(rename = "last_seen")]
    pub last_seen: Option<i64>,
    /// The application version (for example, git commit hash) where the issue was last observed.
    #[serde(rename = "last_seen_version")]
    pub last_seen_version: Option<String>,
    /// Platform associated with the issue.
    #[serde(rename = "platform")]
    pub platform: Option<crate::datadogV2::model::IssuePlatform>,
    /// Service name.
    #[serde(rename = "service")]
    pub service: Option<String>,
    /// State of the issue
    #[serde(rename = "state")]
    pub state: Option<crate::datadogV2::model::IssueState>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IssueAttributes {
    pub fn new() -> IssueAttributes {
        IssueAttributes {
            error_message: None,
            error_type: None,
            file_path: None,
            first_seen: None,
            first_seen_version: None,
            function_name: None,
            is_crash: None,
            languages: None,
            last_seen: None,
            last_seen_version: None,
            platform: None,
            service: None,
            state: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn error_message(mut self, value: String) -> Self {
        self.error_message = Some(value);
        self
    }

    pub fn error_type(mut self, value: String) -> Self {
        self.error_type = Some(value);
        self
    }

    pub fn file_path(mut self, value: String) -> Self {
        self.file_path = Some(value);
        self
    }

    pub fn first_seen(mut self, value: i64) -> Self {
        self.first_seen = Some(value);
        self
    }

    pub fn first_seen_version(mut self, value: String) -> Self {
        self.first_seen_version = Some(value);
        self
    }

    pub fn function_name(mut self, value: String) -> Self {
        self.function_name = Some(value);
        self
    }

    pub fn is_crash(mut self, value: bool) -> Self {
        self.is_crash = Some(value);
        self
    }

    pub fn languages(mut self, value: Vec<crate::datadogV2::model::IssueLanguage>) -> Self {
        self.languages = Some(value);
        self
    }

    pub fn last_seen(mut self, value: i64) -> Self {
        self.last_seen = Some(value);
        self
    }

    pub fn last_seen_version(mut self, value: String) -> Self {
        self.last_seen_version = Some(value);
        self
    }

    pub fn platform(mut self, value: crate::datadogV2::model::IssuePlatform) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
        self
    }

    pub fn state(mut self, value: crate::datadogV2::model::IssueState) -> Self {
        self.state = Some(value);
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

impl Default for IssueAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for IssueAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IssueAttributesVisitor;
        impl<'a> Visitor<'a> for IssueAttributesVisitor {
            type Value = IssueAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut error_message: Option<String> = None;
                let mut error_type: Option<String> = None;
                let mut file_path: Option<String> = None;
                let mut first_seen: Option<i64> = None;
                let mut first_seen_version: Option<String> = None;
                let mut function_name: Option<String> = None;
                let mut is_crash: Option<bool> = None;
                let mut languages: Option<Vec<crate::datadogV2::model::IssueLanguage>> = None;
                let mut last_seen: Option<i64> = None;
                let mut last_seen_version: Option<String> = None;
                let mut platform: Option<crate::datadogV2::model::IssuePlatform> = None;
                let mut service: Option<String> = None;
                let mut state: Option<crate::datadogV2::model::IssueState> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "error_message" => {
                            if v.is_null() {
                                continue;
                            }
                            error_message =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_type" => {
                            if v.is_null() {
                                continue;
                            }
                            error_type = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "file_path" => {
                            if v.is_null() {
                                continue;
                            }
                            file_path = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "first_seen" => {
                            if v.is_null() {
                                continue;
                            }
                            first_seen = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "first_seen_version" => {
                            if v.is_null() {
                                continue;
                            }
                            first_seen_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "function_name" => {
                            if v.is_null() {
                                continue;
                            }
                            function_name =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "is_crash" => {
                            if v.is_null() {
                                continue;
                            }
                            is_crash = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "languages" => {
                            if v.is_null() {
                                continue;
                            }
                            languages = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_seen" => {
                            if v.is_null() {
                                continue;
                            }
                            last_seen = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_seen_version" => {
                            if v.is_null() {
                                continue;
                            }
                            last_seen_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "platform" => {
                            if v.is_null() {
                                continue;
                            }
                            platform = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _platform) = platform {
                                match _platform {
                                    crate::datadogV2::model::IssuePlatform::UnparsedObject(
                                        _platform,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            if v.is_null() {
                                continue;
                            }
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _state) = state {
                                match _state {
                                    crate::datadogV2::model::IssueState::UnparsedObject(_state) => {
                                        _unparsed = true;
                                    }
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

                let content = IssueAttributes {
                    error_message,
                    error_type,
                    file_path,
                    first_seen,
                    first_seen_version,
                    function_name,
                    is_crash,
                    languages,
                    last_seen,
                    last_seen_version,
                    platform,
                    service,
                    state,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IssueAttributesVisitor)
    }
}
