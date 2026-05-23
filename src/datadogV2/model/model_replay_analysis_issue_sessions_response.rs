// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A paginated list of sessions related to a RUM replay analysis issue.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ReplayAnalysisIssueSessionsResponse {
    /// Array of session data objects related to the issue.
    #[serde(rename = "data")]
    pub data: Vec<crate::datadogV2::model::ReplayAnalysisIssueSessionData>,
    /// Metadata object for paginated issue list responses.
    #[serde(rename = "meta")]
    pub meta: crate::datadogV2::model::ReplayAnalysisIssueMeta,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ReplayAnalysisIssueSessionsResponse {
    pub fn new(
        data: Vec<crate::datadogV2::model::ReplayAnalysisIssueSessionData>,
        meta: crate::datadogV2::model::ReplayAnalysisIssueMeta,
    ) -> ReplayAnalysisIssueSessionsResponse {
        ReplayAnalysisIssueSessionsResponse {
            data,
            meta,
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

impl<'de> Deserialize<'de> for ReplayAnalysisIssueSessionsResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReplayAnalysisIssueSessionsResponseVisitor;
        impl<'a> Visitor<'a> for ReplayAnalysisIssueSessionsResponseVisitor {
            type Value = ReplayAnalysisIssueSessionsResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<Vec<crate::datadogV2::model::ReplayAnalysisIssueSessionData>> =
                    None;
                let mut meta: Option<crate::datadogV2::model::ReplayAnalysisIssueMeta> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meta" => {
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let data = data.ok_or_else(|| M::Error::missing_field("data"))?;
                let meta = meta.ok_or_else(|| M::Error::missing_field("meta"))?;

                let content = ReplayAnalysisIssueSessionsResponse {
                    data,
                    meta,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ReplayAnalysisIssueSessionsResponseVisitor)
    }
}
