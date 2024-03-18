// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The JSON:API attributes of the finding.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct FindingAttributes {
    /// The evaluation of the finding.
    #[serde(rename = "evaluation")]
    pub evaluation: Option<crate::datadogV2::model::FindingEvaluation>,
    /// The date on which the evaluation for this finding changed (Unix ms).
    #[serde(rename = "evaluation_changed_at")]
    pub evaluation_changed_at: Option<i64>,
    /// Information about the mute status of this finding.
    #[serde(rename = "mute")]
    pub mute: Option<crate::datadogV2::model::FindingMute>,
    /// The resource name of this finding.
    #[serde(rename = "resource")]
    pub resource: Option<String>,
    /// The date on which the resource was discovered (Unix ms).
    #[serde(rename = "resource_discovery_date")]
    pub resource_discovery_date: Option<i64>,
    /// The resource type of this finding.
    #[serde(rename = "resource_type")]
    pub resource_type: Option<String>,
    /// The rule that triggered this finding.
    #[serde(rename = "rule")]
    pub rule: Option<crate::datadogV2::model::FindingRule>,
    /// The status of the finding.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::FindingStatus>,
    /// The tags associated with this finding.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl FindingAttributes {
    pub fn new() -> FindingAttributes {
        FindingAttributes {
            evaluation: None,
            evaluation_changed_at: None,
            mute: None,
            resource: None,
            resource_discovery_date: None,
            resource_type: None,
            rule: None,
            status: None,
            tags: None,
            _unparsed: false,
        }
    }

    pub fn evaluation(mut self, value: crate::datadogV2::model::FindingEvaluation) -> Self {
        self.evaluation = Some(value);
        self
    }

    pub fn evaluation_changed_at(mut self, value: i64) -> Self {
        self.evaluation_changed_at = Some(value);
        self
    }

    pub fn mute(mut self, value: crate::datadogV2::model::FindingMute) -> Self {
        self.mute = Some(value);
        self
    }

    pub fn resource(mut self, value: String) -> Self {
        self.resource = Some(value);
        self
    }

    pub fn resource_discovery_date(mut self, value: i64) -> Self {
        self.resource_discovery_date = Some(value);
        self
    }

    pub fn resource_type(mut self, value: String) -> Self {
        self.resource_type = Some(value);
        self
    }

    pub fn rule(mut self, value: crate::datadogV2::model::FindingRule) -> Self {
        self.rule = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV2::model::FindingStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }
}

impl Default for FindingAttributes {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for FindingAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FindingAttributesVisitor;
        impl<'a> Visitor<'a> for FindingAttributesVisitor {
            type Value = FindingAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut evaluation: Option<crate::datadogV2::model::FindingEvaluation> = None;
                let mut evaluation_changed_at: Option<i64> = None;
                let mut mute: Option<crate::datadogV2::model::FindingMute> = None;
                let mut resource: Option<String> = None;
                let mut resource_discovery_date: Option<i64> = None;
                let mut resource_type: Option<String> = None;
                let mut rule: Option<crate::datadogV2::model::FindingRule> = None;
                let mut status: Option<crate::datadogV2::model::FindingStatus> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "evaluation" => {
                            if v.is_null() {
                                continue;
                            }
                            evaluation = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _evaluation) = evaluation {
                                match _evaluation {
                                    crate::datadogV2::model::FindingEvaluation::UnparsedObject(
                                        _evaluation,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "evaluation_changed_at" => {
                            if v.is_null() {
                                continue;
                            }
                            evaluation_changed_at =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "mute" => {
                            if v.is_null() {
                                continue;
                            }
                            mute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource" => {
                            if v.is_null() {
                                continue;
                            }
                            resource = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_discovery_date" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_discovery_date =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_type" => {
                            if v.is_null() {
                                continue;
                            }
                            resource_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rule" => {
                            if v.is_null() {
                                continue;
                            }
                            rule = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::FindingStatus::UnparsedObject(
                                        _status,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = FindingAttributes {
                    evaluation,
                    evaluation_changed_at,
                    mute,
                    resource,
                    resource_discovery_date,
                    resource_type,
                    rule,
                    status,
                    tags,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(FindingAttributesVisitor)
    }
}
