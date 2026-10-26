// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Object containing the information of a sample.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct IssueSampleAttributes {
    /// RUM application identifier of the error.
    #[serde(rename = "application_id")]
    pub application_id: Option<String>,
    /// Biases the sample satisfies. A single error can satisfy several biases.
    #[serde(rename = "biases")]
    pub biases: Option<Vec<crate::datadogV2::model::IssueSampleBias>>,
    /// Build identifier of the mobile application.
    #[serde(rename = "build_id")]
    pub build_id: Option<String>,
    /// Bundle version of the mobile application.
    #[serde(rename = "bundle_version")]
    pub bundle_version: Option<String>,
    /// Product that reported the error. The value is `NONE` when the error has no known origin.
    #[serde(rename = "data_source")]
    pub data_source: Option<crate::datadogV2::model::IssueSampleDataSource>,
    /// Debug identifier used to resolve the stack trace of the error to source code.
    #[serde(rename = "debug_id")]
    pub debug_id: Option<String>,
    /// Whether an Exception Replay snapshot was captured for the error.
    #[serde(rename = "debug_info_captured")]
    pub debug_info_captured: Option<bool>,
    /// Source type set by the SDK or tracer that reported the error, such as `browser` for RUM browser errors
    /// or `Crashtracking` for backend crash reports. Usually not set for APM and logs errors.
    #[serde(rename = "error_source_type")]
    pub error_source_type: Option<String>,
    /// Parsed stack trace of the error. Only the sample picked by the `git_metadata` bias has frames.
    #[serde(rename = "error_stack")]
    pub error_stack: crate::datadogV2::model::IssueSampleErrorStack,
    /// Raw error event, as stored by its data source. Its fields differ between APM, logs, and RUM.
    #[serde(rename = "event")]
    pub event: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    /// Git commit SHA of the service version.
    #[serde(rename = "git_commit_sha")]
    pub git_commit_sha: Option<String>,
    /// Git repository identifier of the service.
    #[serde(rename = "git_repository_id")]
    pub git_repository_id: Option<String>,
    /// Git repository URL of the service.
    #[serde(rename = "git_repository_url")]
    pub git_repository_url: Option<String>,
    /// Whether a RUM session replay is available for the error.
    #[serde(rename = "has_session_replay")]
    pub has_session_replay: Option<bool>,
    /// Programming language of the service.
    #[serde(rename = "language")]
    pub language: Option<String>,
    /// Source code locations of the stack frames that resolve to a repository file.
    #[serde(rename = "links")]
    pub links: Option<Vec<crate::datadogV2::model::IssueSampleFrameLink>>,
    /// Operating system of the device that reported the error.
    #[serde(rename = "os_name")]
    pub os_name: Option<String>,
    /// Service name.
    #[serde(rename = "service")]
    pub service: Option<String>,
    /// RUM session identifier of the error.
    #[serde(rename = "session_id")]
    pub session_id: Option<String>,
    /// URL of the RUM session replay of the error.
    #[serde(rename = "session_replay_url")]
    pub session_replay_url: Option<String>,
    /// Identifier of the Exception Replay snapshot of the error.
    #[serde(rename = "snapshot_id")]
    pub snapshot_id: Option<String>,
    /// APM span identifier of the error.
    #[serde(rename = "span_id")]
    pub span_id: Option<String>,
    /// Timestamp of the error.
    #[serde(rename = "timestamp")]
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// APM trace identifier of the error.
    #[serde(rename = "trace_id")]
    pub trace_id: Option<String>,
    /// Build variant of the mobile application.
    #[serde(rename = "variant")]
    pub variant: Option<String>,
    /// Version of the service.
    #[serde(rename = "version")]
    pub version: Option<String>,
    /// RUM view identifier of the error.
    #[serde(rename = "view_id")]
    pub view_id: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl IssueSampleAttributes {
    pub fn new(
        error_stack: crate::datadogV2::model::IssueSampleErrorStack,
        timestamp: chrono::DateTime<chrono::Utc>,
    ) -> IssueSampleAttributes {
        IssueSampleAttributes {
            application_id: None,
            biases: None,
            build_id: None,
            bundle_version: None,
            data_source: None,
            debug_id: None,
            debug_info_captured: None,
            error_source_type: None,
            error_stack,
            event: None,
            git_commit_sha: None,
            git_repository_id: None,
            git_repository_url: None,
            has_session_replay: None,
            language: None,
            links: None,
            os_name: None,
            service: None,
            session_id: None,
            session_replay_url: None,
            snapshot_id: None,
            span_id: None,
            timestamp,
            trace_id: None,
            variant: None,
            version: None,
            view_id: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn application_id(mut self, value: String) -> Self {
        self.application_id = Some(value);
        self
    }

    pub fn biases(mut self, value: Vec<crate::datadogV2::model::IssueSampleBias>) -> Self {
        self.biases = Some(value);
        self
    }

    pub fn build_id(mut self, value: String) -> Self {
        self.build_id = Some(value);
        self
    }

    pub fn bundle_version(mut self, value: String) -> Self {
        self.bundle_version = Some(value);
        self
    }

    pub fn data_source(mut self, value: crate::datadogV2::model::IssueSampleDataSource) -> Self {
        self.data_source = Some(value);
        self
    }

    pub fn debug_id(mut self, value: String) -> Self {
        self.debug_id = Some(value);
        self
    }

    pub fn debug_info_captured(mut self, value: bool) -> Self {
        self.debug_info_captured = Some(value);
        self
    }

    pub fn error_source_type(mut self, value: String) -> Self {
        self.error_source_type = Some(value);
        self
    }

    pub fn event(mut self, value: std::collections::BTreeMap<String, serde_json::Value>) -> Self {
        self.event = Some(value);
        self
    }

    pub fn git_commit_sha(mut self, value: String) -> Self {
        self.git_commit_sha = Some(value);
        self
    }

    pub fn git_repository_id(mut self, value: String) -> Self {
        self.git_repository_id = Some(value);
        self
    }

    pub fn git_repository_url(mut self, value: String) -> Self {
        self.git_repository_url = Some(value);
        self
    }

    pub fn has_session_replay(mut self, value: bool) -> Self {
        self.has_session_replay = Some(value);
        self
    }

    pub fn language(mut self, value: String) -> Self {
        self.language = Some(value);
        self
    }

    pub fn links(mut self, value: Vec<crate::datadogV2::model::IssueSampleFrameLink>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn os_name(mut self, value: String) -> Self {
        self.os_name = Some(value);
        self
    }

    pub fn service(mut self, value: String) -> Self {
        self.service = Some(value);
        self
    }

    pub fn session_id(mut self, value: String) -> Self {
        self.session_id = Some(value);
        self
    }

    pub fn session_replay_url(mut self, value: String) -> Self {
        self.session_replay_url = Some(value);
        self
    }

    pub fn snapshot_id(mut self, value: String) -> Self {
        self.snapshot_id = Some(value);
        self
    }

    pub fn span_id(mut self, value: String) -> Self {
        self.span_id = Some(value);
        self
    }

    pub fn trace_id(mut self, value: String) -> Self {
        self.trace_id = Some(value);
        self
    }

    pub fn variant(mut self, value: String) -> Self {
        self.variant = Some(value);
        self
    }

    pub fn version(mut self, value: String) -> Self {
        self.version = Some(value);
        self
    }

    pub fn view_id(mut self, value: String) -> Self {
        self.view_id = Some(value);
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

impl<'de> Deserialize<'de> for IssueSampleAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IssueSampleAttributesVisitor;
        impl<'a> Visitor<'a> for IssueSampleAttributesVisitor {
            type Value = IssueSampleAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut application_id: Option<String> = None;
                let mut biases: Option<Vec<crate::datadogV2::model::IssueSampleBias>> = None;
                let mut build_id: Option<String> = None;
                let mut bundle_version: Option<String> = None;
                let mut data_source: Option<crate::datadogV2::model::IssueSampleDataSource> = None;
                let mut debug_id: Option<String> = None;
                let mut debug_info_captured: Option<bool> = None;
                let mut error_source_type: Option<String> = None;
                let mut error_stack: Option<crate::datadogV2::model::IssueSampleErrorStack> = None;
                let mut event: Option<std::collections::BTreeMap<String, serde_json::Value>> = None;
                let mut git_commit_sha: Option<String> = None;
                let mut git_repository_id: Option<String> = None;
                let mut git_repository_url: Option<String> = None;
                let mut has_session_replay: Option<bool> = None;
                let mut language: Option<String> = None;
                let mut links: Option<Vec<crate::datadogV2::model::IssueSampleFrameLink>> = None;
                let mut os_name: Option<String> = None;
                let mut service: Option<String> = None;
                let mut session_id: Option<String> = None;
                let mut session_replay_url: Option<String> = None;
                let mut snapshot_id: Option<String> = None;
                let mut span_id: Option<String> = None;
                let mut timestamp: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut trace_id: Option<String> = None;
                let mut variant: Option<String> = None;
                let mut version: Option<String> = None;
                let mut view_id: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "application_id" => {
                            if v.is_null() {
                                continue;
                            }
                            application_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "biases" => {
                            if v.is_null() {
                                continue;
                            }
                            biases = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "build_id" => {
                            if v.is_null() {
                                continue;
                            }
                            build_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "bundle_version" => {
                            if v.is_null() {
                                continue;
                            }
                            bundle_version =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "data_source" => {
                            if v.is_null() {
                                continue;
                            }
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _data_source) = data_source {
                                match _data_source {
                                    crate::datadogV2::model::IssueSampleDataSource::UnparsedObject(_data_source) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "debug_id" => {
                            if v.is_null() {
                                continue;
                            }
                            debug_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "debug_info_captured" => {
                            if v.is_null() {
                                continue;
                            }
                            debug_info_captured =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_source_type" => {
                            if v.is_null() {
                                continue;
                            }
                            error_source_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_stack" => {
                            error_stack =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "event" => {
                            if v.is_null() {
                                continue;
                            }
                            event = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "git_commit_sha" => {
                            if v.is_null() {
                                continue;
                            }
                            git_commit_sha =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "git_repository_id" => {
                            if v.is_null() {
                                continue;
                            }
                            git_repository_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "git_repository_url" => {
                            if v.is_null() {
                                continue;
                            }
                            git_repository_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "has_session_replay" => {
                            if v.is_null() {
                                continue;
                            }
                            has_session_replay =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "language" => {
                            if v.is_null() {
                                continue;
                            }
                            language = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "links" => {
                            if v.is_null() {
                                continue;
                            }
                            links = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "os_name" => {
                            if v.is_null() {
                                continue;
                            }
                            os_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "service" => {
                            if v.is_null() {
                                continue;
                            }
                            service = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_id" => {
                            if v.is_null() {
                                continue;
                            }
                            session_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_replay_url" => {
                            if v.is_null() {
                                continue;
                            }
                            session_replay_url =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "snapshot_id" => {
                            if v.is_null() {
                                continue;
                            }
                            snapshot_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "span_id" => {
                            if v.is_null() {
                                continue;
                            }
                            span_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timestamp" => {
                            timestamp = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "trace_id" => {
                            if v.is_null() {
                                continue;
                            }
                            trace_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "variant" => {
                            if v.is_null() {
                                continue;
                            }
                            variant = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "version" => {
                            if v.is_null() {
                                continue;
                            }
                            version = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "view_id" => {
                            if v.is_null() {
                                continue;
                            }
                            view_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let error_stack =
                    error_stack.ok_or_else(|| M::Error::missing_field("error_stack"))?;
                let timestamp = timestamp.ok_or_else(|| M::Error::missing_field("timestamp"))?;

                let content = IssueSampleAttributes {
                    application_id,
                    biases,
                    build_id,
                    bundle_version,
                    data_source,
                    debug_id,
                    debug_info_captured,
                    error_source_type,
                    error_stack,
                    event,
                    git_commit_sha,
                    git_repository_id,
                    git_repository_url,
                    has_session_replay,
                    language,
                    links,
                    os_name,
                    service,
                    session_id,
                    session_replay_url,
                    snapshot_id,
                    span_id,
                    timestamp,
                    trace_id,
                    variant,
                    version,
                    view_id,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(IssueSampleAttributesVisitor)
    }
}
