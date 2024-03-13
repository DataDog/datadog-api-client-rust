// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// CI visibility usage in a given hour.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct UsageCIVisibilityHour {
    /// The number of spans for pipelines in the queried hour.
    #[serde(
        rename = "ci_pipeline_indexed_spans",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub ci_pipeline_indexed_spans: Option<Option<i64>>,
    /// The number of spans for tests in the queried hour.
    #[serde(
        rename = "ci_test_indexed_spans",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub ci_test_indexed_spans: Option<Option<i64>>,
    /// Shows the total count of all active Git committers for Intelligent Test Runner in the current month. A committer is active if they commit at least 3 times in a given month.
    #[serde(
        rename = "ci_visibility_itr_committers",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub ci_visibility_itr_committers: Option<Option<i64>>,
    /// Shows the total count of all active Git committers for Pipelines in the current month. A committer is active if they commit at least 3 times in a given month.
    #[serde(
        rename = "ci_visibility_pipeline_committers",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub ci_visibility_pipeline_committers: Option<Option<i64>>,
    /// The total count of all active Git committers for tests in the current month. A committer is active if they commit at least 3 times in a given month.
    #[serde(
        rename = "ci_visibility_test_committers",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub ci_visibility_test_committers: Option<Option<i64>>,
    /// The organization name.
    #[serde(rename = "org_name")]
    pub org_name: Option<String>,
    /// The organization public ID.
    #[serde(rename = "public_id")]
    pub public_id: Option<String>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl UsageCIVisibilityHour {
    pub fn new() -> UsageCIVisibilityHour {
        UsageCIVisibilityHour {
            ci_pipeline_indexed_spans: None,
            ci_test_indexed_spans: None,
            ci_visibility_itr_committers: None,
            ci_visibility_pipeline_committers: None,
            ci_visibility_test_committers: None,
            org_name: None,
            public_id: None,
            _unparsed: false,
        }
    }

    pub fn ci_pipeline_indexed_spans(mut self, value: Option<i64>) -> Self {
        self.ci_pipeline_indexed_spans = Some(value);
        self
    }

    pub fn ci_test_indexed_spans(mut self, value: Option<i64>) -> Self {
        self.ci_test_indexed_spans = Some(value);
        self
    }

    pub fn ci_visibility_itr_committers(mut self, value: Option<i64>) -> Self {
        self.ci_visibility_itr_committers = Some(value);
        self
    }

    pub fn ci_visibility_pipeline_committers(mut self, value: Option<i64>) -> Self {
        self.ci_visibility_pipeline_committers = Some(value);
        self
    }

    pub fn ci_visibility_test_committers(mut self, value: Option<i64>) -> Self {
        self.ci_visibility_test_committers = Some(value);
        self
    }

    pub fn org_name(mut self, value: String) -> Self {
        self.org_name = Some(value);
        self
    }

    pub fn public_id(mut self, value: String) -> Self {
        self.public_id = Some(value);
        self
    }
}

impl Default for UsageCIVisibilityHour {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for UsageCIVisibilityHour {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct UsageCIVisibilityHourVisitor;
        impl<'a> Visitor<'a> for UsageCIVisibilityHourVisitor {
            type Value = UsageCIVisibilityHour;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut ci_pipeline_indexed_spans: Option<Option<i64>> = None;
                let mut ci_test_indexed_spans: Option<Option<i64>> = None;
                let mut ci_visibility_itr_committers: Option<Option<i64>> = None;
                let mut ci_visibility_pipeline_committers: Option<Option<i64>> = None;
                let mut ci_visibility_test_committers: Option<Option<i64>> = None;
                let mut org_name: Option<String> = None;
                let mut public_id: Option<String> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "ci_pipeline_indexed_spans" => {
                            ci_pipeline_indexed_spans =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_test_indexed_spans" => {
                            ci_test_indexed_spans =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_visibility_itr_committers" => {
                            ci_visibility_itr_committers =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_visibility_pipeline_committers" => {
                            ci_visibility_pipeline_committers =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ci_visibility_test_committers" => {
                            ci_visibility_test_committers =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_name" => {
                            if v.is_null() {
                                continue;
                            }
                            org_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "public_id" => {
                            if v.is_null() {
                                continue;
                            }
                            public_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = UsageCIVisibilityHour {
                    ci_pipeline_indexed_spans,
                    ci_test_indexed_spans,
                    ci_visibility_itr_committers,
                    ci_visibility_pipeline_committers,
                    ci_visibility_test_committers,
                    org_name,
                    public_id,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(UsageCIVisibilityHourVisitor)
    }
}
