// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A representative session illustrating a replay analysis issue.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ReplayAnalysisRepresentativeSession {
    /// Category of the issue observed in this session.
    #[serde(rename = "issue_category")]
    pub issue_category: String,
    /// AI-generated analysis details for a replay issue.
    #[serde(rename = "llm_analysis_details")]
    pub llm_analysis_details: crate::datadogV2::model::ReplayAnalysisLLMDetails,
    /// A screenshot captured during a replay session.
    #[serde(rename = "screenshot")]
    pub screenshot: Option<crate::datadogV2::model::ReplayAnalysisScreenshot>,
    /// Unique identifier of the representative session.
    #[serde(rename = "session_id")]
    pub session_id: String,
    /// Session start timestamp in milliseconds.
    #[serde(rename = "session_start_timestamp_ms")]
    pub session_start_timestamp_ms: i64,
    /// List of signals observed in the representative session.
    #[serde(rename = "signals")]
    pub signals: Vec<crate::datadogV2::model::ReplayAnalysisSignal>,
    /// Name of the view where the issue was observed.
    #[serde(
        rename = "view_name",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub view_name: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ReplayAnalysisRepresentativeSession {
    pub fn new(
        issue_category: String,
        llm_analysis_details: crate::datadogV2::model::ReplayAnalysisLLMDetails,
        session_id: String,
        session_start_timestamp_ms: i64,
        signals: Vec<crate::datadogV2::model::ReplayAnalysisSignal>,
    ) -> ReplayAnalysisRepresentativeSession {
        ReplayAnalysisRepresentativeSession {
            issue_category,
            llm_analysis_details,
            screenshot: None,
            session_id,
            session_start_timestamp_ms,
            signals,
            view_name: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn screenshot(mut self, value: crate::datadogV2::model::ReplayAnalysisScreenshot) -> Self {
        self.screenshot = Some(value);
        self
    }

    pub fn view_name(mut self, value: Option<String>) -> Self {
        self.view_name = Some(value);
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

impl<'de> Deserialize<'de> for ReplayAnalysisRepresentativeSession {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReplayAnalysisRepresentativeSessionVisitor;
        impl<'a> Visitor<'a> for ReplayAnalysisRepresentativeSessionVisitor {
            type Value = ReplayAnalysisRepresentativeSession;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut issue_category: Option<String> = None;
                let mut llm_analysis_details: Option<
                    crate::datadogV2::model::ReplayAnalysisLLMDetails,
                > = None;
                let mut screenshot: Option<crate::datadogV2::model::ReplayAnalysisScreenshot> =
                    None;
                let mut session_id: Option<String> = None;
                let mut session_start_timestamp_ms: Option<i64> = None;
                let mut signals: Option<Vec<crate::datadogV2::model::ReplayAnalysisSignal>> = None;
                let mut view_name: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "issue_category" => {
                            issue_category =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "llm_analysis_details" => {
                            llm_analysis_details =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "screenshot" => {
                            if v.is_null() {
                                continue;
                            }
                            screenshot = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_id" => {
                            session_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_start_timestamp_ms" => {
                            session_start_timestamp_ms =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "signals" => {
                            signals = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "view_name" => {
                            view_name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let issue_category =
                    issue_category.ok_or_else(|| M::Error::missing_field("issue_category"))?;
                let llm_analysis_details = llm_analysis_details
                    .ok_or_else(|| M::Error::missing_field("llm_analysis_details"))?;
                let session_id = session_id.ok_or_else(|| M::Error::missing_field("session_id"))?;
                let session_start_timestamp_ms = session_start_timestamp_ms
                    .ok_or_else(|| M::Error::missing_field("session_start_timestamp_ms"))?;
                let signals = signals.ok_or_else(|| M::Error::missing_field("signals"))?;

                let content = ReplayAnalysisRepresentativeSession {
                    issue_category,
                    llm_analysis_details,
                    screenshot,
                    session_id,
                    session_start_timestamp_ms,
                    signals,
                    view_name,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ReplayAnalysisRepresentativeSessionVisitor)
    }
}
