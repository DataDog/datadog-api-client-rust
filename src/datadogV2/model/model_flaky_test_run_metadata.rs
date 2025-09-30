// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata about the latest failed test run of the flaky test.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FlakyTestRunMetadata {
    /// The duration of the test run in milliseconds.
    #[serde(
        rename = "duration_ms",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub duration_ms: Option<Option<i64>>,
    /// The error message from the test failure.
    #[serde(
        rename = "error_message",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub error_message: Option<Option<String>>,
    /// The stack trace from the test failure.
    #[serde(
        rename = "error_stack",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub error_stack: Option<Option<String>>,
    /// The line number where the test ends in the source file.
    #[serde(
        rename = "source_end",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub source_end: Option<Option<i64>>,
    /// The source file where the test is defined.
    #[serde(
        rename = "source_file",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub source_file: Option<Option<String>>,
    /// The line number where the test starts in the source file.
    #[serde(
        rename = "source_start",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub source_start: Option<Option<i64>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FlakyTestRunMetadata {
    pub fn new() -> FlakyTestRunMetadata {
        FlakyTestRunMetadata {
            duration_ms: None,
            error_message: None,
            error_stack: None,
            source_end: None,
            source_file: None,
            source_start: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn duration_ms(mut self, value: Option<i64>) -> Self {
        self.duration_ms = Some(value);
        self
    }

    pub fn error_message(mut self, value: Option<String>) -> Self {
        self.error_message = Some(value);
        self
    }

    pub fn error_stack(mut self, value: Option<String>) -> Self {
        self.error_stack = Some(value);
        self
    }

    pub fn source_end(mut self, value: Option<i64>) -> Self {
        self.source_end = Some(value);
        self
    }

    pub fn source_file(mut self, value: Option<String>) -> Self {
        self.source_file = Some(value);
        self
    }

    pub fn source_start(mut self, value: Option<i64>) -> Self {
        self.source_start = Some(value);
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

impl Default for FlakyTestRunMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FlakyTestRunMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FlakyTestRunMetadataVisitor;
        impl<'a> Visitor<'a> for FlakyTestRunMetadataVisitor {
            type Value = FlakyTestRunMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut duration_ms: Option<Option<i64>> = None;
                let mut error_message: Option<Option<String>> = None;
                let mut error_stack: Option<Option<String>> = None;
                let mut source_end: Option<Option<i64>> = None;
                let mut source_file: Option<Option<String>> = None;
                let mut source_start: Option<Option<i64>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "duration_ms" => {
                            duration_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_message" => {
                            error_message =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "error_stack" => {
                            error_stack =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source_end" => {
                            source_end = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source_file" => {
                            source_file =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source_start" => {
                            source_start =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = FlakyTestRunMetadata {
                    duration_ms,
                    error_message,
                    error_stack,
                    source_end,
                    source_file,
                    source_start,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FlakyTestRunMetadataVisitor)
    }
}
