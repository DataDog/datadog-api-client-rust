// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Summary of optional influential-tag enrichment. Count and key fields are present only when analysis completes; enrichment availability does not affect completion of the investigation result.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TimeseriesAnomalyInvestigationTagAnalysis {
    /// Tag keys analyzed. Present only when analysis completes.
    #[serde(rename = "analyzed_tag_keys")]
    pub analyzed_tag_keys: Option<Vec<String>>,
    /// Outcome of optional influential-tag enrichment.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::TimeseriesAnomalyInvestigationTagAnalysisStatus,
    /// Number of tag keys analyzed. Present only when analysis completes.
    #[serde(rename = "tag_keys_analyzed")]
    pub tag_keys_analyzed: Option<i64>,
    /// Number of tag values analyzed. Present only when analysis completes.
    #[serde(rename = "tag_values_analyzed")]
    pub tag_values_analyzed: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl TimeseriesAnomalyInvestigationTagAnalysis {
    pub fn new(
        status: crate::datadogV2::model::TimeseriesAnomalyInvestigationTagAnalysisStatus,
    ) -> TimeseriesAnomalyInvestigationTagAnalysis {
        TimeseriesAnomalyInvestigationTagAnalysis {
            analyzed_tag_keys: None,
            status,
            tag_keys_analyzed: None,
            tag_values_analyzed: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn analyzed_tag_keys(mut self, value: Vec<String>) -> Self {
        self.analyzed_tag_keys = Some(value);
        self
    }

    pub fn tag_keys_analyzed(mut self, value: i64) -> Self {
        self.tag_keys_analyzed = Some(value);
        self
    }

    pub fn tag_values_analyzed(mut self, value: i64) -> Self {
        self.tag_values_analyzed = Some(value);
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

impl<'de> Deserialize<'de> for TimeseriesAnomalyInvestigationTagAnalysis {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TimeseriesAnomalyInvestigationTagAnalysisVisitor;
        impl<'a> Visitor<'a> for TimeseriesAnomalyInvestigationTagAnalysisVisitor {
            type Value = TimeseriesAnomalyInvestigationTagAnalysis;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut analyzed_tag_keys: Option<Vec<String>> = None;
                let mut status: Option<
                    crate::datadogV2::model::TimeseriesAnomalyInvestigationTagAnalysisStatus,
                > = None;
                let mut tag_keys_analyzed: Option<i64> = None;
                let mut tag_values_analyzed: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "analyzed_tag_keys" => {
                            if v.is_null() {
                                continue;
                            }
                            analyzed_tag_keys =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::TimeseriesAnomalyInvestigationTagAnalysisStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "tag_keys_analyzed" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_keys_analyzed =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tag_values_analyzed" => {
                            if v.is_null() {
                                continue;
                            }
                            tag_values_analyzed =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = TimeseriesAnomalyInvestigationTagAnalysis {
                    analyzed_tag_keys,
                    status,
                    tag_keys_analyzed,
                    tag_values_analyzed,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(TimeseriesAnomalyInvestigationTagAnalysisVisitor)
    }
}
