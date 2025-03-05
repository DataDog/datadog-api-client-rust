// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Holds search results.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MonitorSearchResult {
    /// Classification of the monitor.
    #[serde(rename = "classification")]
    pub classification: Option<String>,
    /// Object describing the creator of the shared element.
    #[serde(rename = "creator")]
    pub creator: Option<crate::datadogV1::model::Creator>,
    /// ID of the monitor.
    #[serde(rename = "id")]
    pub id: Option<i64>,
    /// Latest timestamp the monitor triggered.
    #[serde(
        rename = "last_triggered_ts",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub last_triggered_ts: Option<Option<i64>>,
    /// Metrics used by the monitor.
    #[serde(rename = "metrics")]
    pub metrics: Option<Vec<String>>,
    /// The monitor name.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// The notification triggered by the monitor.
    #[serde(rename = "notifications")]
    pub notifications: Option<Vec<crate::datadogV1::model::MonitorSearchResultNotification>>,
    /// The ID of the organization.
    #[serde(rename = "org_id")]
    pub org_id: Option<i64>,
    /// Quality issues detected with the monitor.
    #[serde(rename = "quality_issues")]
    pub quality_issues: Option<Vec<String>>,
    /// The monitor query.
    #[serde(rename = "query")]
    pub query: Option<String>,
    /// The scope(s) to which the downtime applies, for example `host:app2`.
    /// Provide multiple scopes as a comma-separated list, for example `env:dev,env:prod`.
    /// The resulting downtime applies to sources that matches ALL provided scopes
    /// (that is `env:dev AND env:prod`), NOT any of them.
    #[serde(rename = "scopes")]
    pub scopes: Option<Vec<String>>,
    /// The different states your monitor can be in.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV1::model::MonitorOverallStates>,
    /// Tags associated with the monitor.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// The type of the monitor. For more information about `type`, see the [monitor options](<https://docs.datadoghq.com/monitors/guide/monitor_api_options/>) docs.
    #[serde(rename = "type")]
    pub type_: Option<crate::datadogV1::model::MonitorType>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl MonitorSearchResult {
    pub fn new() -> MonitorSearchResult {
        MonitorSearchResult {
            classification: None,
            creator: None,
            id: None,
            last_triggered_ts: None,
            metrics: None,
            name: None,
            notifications: None,
            org_id: None,
            quality_issues: None,
            query: None,
            scopes: None,
            status: None,
            tags: None,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn classification(mut self, value: String) -> Self {
        self.classification = Some(value);
        self
    }

    pub fn creator(mut self, value: crate::datadogV1::model::Creator) -> Self {
        self.creator = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn last_triggered_ts(mut self, value: Option<i64>) -> Self {
        self.last_triggered_ts = Some(value);
        self
    }

    pub fn metrics(mut self, value: Vec<String>) -> Self {
        self.metrics = Some(value);
        self
    }

    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    pub fn notifications(
        mut self,
        value: Vec<crate::datadogV1::model::MonitorSearchResultNotification>,
    ) -> Self {
        self.notifications = Some(value);
        self
    }

    pub fn org_id(mut self, value: i64) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn quality_issues(mut self, value: Vec<String>) -> Self {
        self.quality_issues = Some(value);
        self
    }

    pub fn query(mut self, value: String) -> Self {
        self.query = Some(value);
        self
    }

    pub fn scopes(mut self, value: Vec<String>) -> Self {
        self.scopes = Some(value);
        self
    }

    pub fn status(mut self, value: crate::datadogV1::model::MonitorOverallStates) -> Self {
        self.status = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn type_(mut self, value: crate::datadogV1::model::MonitorType) -> Self {
        self.type_ = Some(value);
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

impl Default for MonitorSearchResult {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for MonitorSearchResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MonitorSearchResultVisitor;
        impl<'a> Visitor<'a> for MonitorSearchResultVisitor {
            type Value = MonitorSearchResult;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut classification: Option<String> = None;
                let mut creator: Option<crate::datadogV1::model::Creator> = None;
                let mut id: Option<i64> = None;
                let mut last_triggered_ts: Option<Option<i64>> = None;
                let mut metrics: Option<Vec<String>> = None;
                let mut name: Option<String> = None;
                let mut notifications: Option<
                    Vec<crate::datadogV1::model::MonitorSearchResultNotification>,
                > = None;
                let mut org_id: Option<i64> = None;
                let mut quality_issues: Option<Vec<String>> = None;
                let mut query: Option<String> = None;
                let mut scopes: Option<Vec<String>> = None;
                let mut status: Option<crate::datadogV1::model::MonitorOverallStates> = None;
                let mut tags: Option<Vec<String>> = None;
                let mut type_: Option<crate::datadogV1::model::MonitorType> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "classification" => {
                            if v.is_null() {
                                continue;
                            }
                            classification =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "creator" => {
                            if v.is_null() {
                                continue;
                            }
                            creator = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            if v.is_null() {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "last_triggered_ts" => {
                            last_triggered_ts =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "metrics" => {
                            if v.is_null() {
                                continue;
                            }
                            metrics = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            if v.is_null() {
                                continue;
                            }
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notifications" => {
                            if v.is_null() {
                                continue;
                            }
                            notifications =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "org_id" => {
                            if v.is_null() {
                                continue;
                            }
                            org_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "quality_issues" => {
                            if v.is_null() {
                                continue;
                            }
                            quality_issues =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            if v.is_null() {
                                continue;
                            }
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "scopes" => {
                            if v.is_null() {
                                continue;
                            }
                            scopes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV1::model::MonitorOverallStates::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
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
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::MonitorType::UnparsedObject(
                                        _type_,
                                    ) => {
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

                let content = MonitorSearchResult {
                    classification,
                    creator,
                    id,
                    last_triggered_ts,
                    metrics,
                    name,
                    notifications,
                    org_id,
                    quality_issues,
                    query,
                    scopes,
                    status,
                    tags,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(MonitorSearchResultVisitor)
    }
}
