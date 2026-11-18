// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An issue contributing to drop-off at the target step.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ProductAnalyticsJourneyDropOffIssue {
    /// Number of errors attributed to this issue.
    #[serde(rename = "error_count")]
    pub error_count: i64,
    /// Identifier of the issue.
    #[serde(rename = "issue_id")]
    pub issue_id: String,
    /// Number of sessions affected by this issue.
    #[serde(rename = "session_count")]
    pub session_count: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ProductAnalyticsJourneyDropOffIssue {
    pub fn new(
        error_count: i64,
        issue_id: String,
        session_count: i64,
    ) -> ProductAnalyticsJourneyDropOffIssue {
        ProductAnalyticsJourneyDropOffIssue {
            error_count,
            issue_id,
            session_count,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for ProductAnalyticsJourneyDropOffIssue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ProductAnalyticsJourneyDropOffIssueVisitor;
        impl<'a> Visitor<'a> for ProductAnalyticsJourneyDropOffIssueVisitor {
            type Value = ProductAnalyticsJourneyDropOffIssue;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut error_count: Option<i64> = None;
                let mut issue_id: Option<String> = None;
                let mut session_count: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "error_count" => {
                            error_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "issue_id" => {
                            issue_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "session_count" => {
                            session_count =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let error_count =
                    error_count.ok_or_else(|| M::Error::missing_field("error_count"))?;
                let issue_id = issue_id.ok_or_else(|| M::Error::missing_field("issue_id"))?;
                let session_count =
                    session_count.ok_or_else(|| M::Error::missing_field("session_count"))?;

                let content = ProductAnalyticsJourneyDropOffIssue {
                    error_count,
                    issue_id,
                    session_count,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ProductAnalyticsJourneyDropOffIssueVisitor)
    }
}
