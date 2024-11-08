// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Definition of a historical job based on a security monitoring rule.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct JobDefinitionFromRule {
    /// Index of the rule case applied by the job.
    #[serde(rename = "caseIndex")]
    pub case_index: i32,
    /// Starting time of data analyzed by the job.
    #[serde(rename = "from")]
    pub from: i64,
    /// ID of the detection rule used to create the job.
    #[serde(rename = "id")]
    pub id: String,
    /// Index used to load the data.
    #[serde(rename = "index")]
    pub index: String,
    /// Notifications sent when the job is completed.
    #[serde(rename = "notifications")]
    pub notifications: Option<Vec<String>>,
    /// Ending time of data analyzed by the job.
    #[serde(rename = "to")]
    pub to: i64,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl JobDefinitionFromRule {
    pub fn new(
        case_index: i32,
        from: i64,
        id: String,
        index: String,
        to: i64,
    ) -> JobDefinitionFromRule {
        JobDefinitionFromRule {
            case_index,
            from,
            id,
            index,
            notifications: None,
            to,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn notifications(mut self, value: Vec<String>) -> Self {
        self.notifications = Some(value);
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

impl<'de> Deserialize<'de> for JobDefinitionFromRule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JobDefinitionFromRuleVisitor;
        impl<'a> Visitor<'a> for JobDefinitionFromRuleVisitor {
            type Value = JobDefinitionFromRule;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut case_index: Option<i32> = None;
                let mut from: Option<i64> = None;
                let mut id: Option<String> = None;
                let mut index: Option<String> = None;
                let mut notifications: Option<Vec<String>> = None;
                let mut to: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "caseIndex" => {
                            case_index = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "from" => {
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "index" => {
                            index = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notifications" => {
                            if v.is_null() {
                                continue;
                            }
                            notifications =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to" => {
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let case_index = case_index.ok_or_else(|| M::Error::missing_field("case_index"))?;
                let from = from.ok_or_else(|| M::Error::missing_field("from"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let index = index.ok_or_else(|| M::Error::missing_field("index"))?;
                let to = to.ok_or_else(|| M::Error::missing_field("to"))?;

                let content = JobDefinitionFromRule {
                    case_index,
                    from,
                    id,
                    index,
                    notifications,
                    to,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(JobDefinitionFromRuleVisitor)
    }
}
