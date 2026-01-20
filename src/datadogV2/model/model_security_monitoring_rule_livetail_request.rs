// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Request to preview a rule query with applied filters.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SecurityMonitoringRuleLivetailRequest {
    /// Data source for the query.
    #[serde(rename = "dataSource")]
    pub data_source: String,
    /// The detection method.
    #[serde(rename = "detectionMethod")]
    pub detection_method: crate::datadogV2::model::SecurityMonitoringRuleDetectionMethod,
    /// Fields to apply distinct on.
    #[serde(rename = "distinctFields")]
    pub distinct_fields: Option<Vec<String>>,
    /// Additional security filters to apply.
    #[serde(rename = "filters")]
    pub filters: Option<Vec<crate::datadogV2::model::SecurityMonitoringFilter>>,
    /// Fields to group by.
    #[serde(rename = "groupByFields")]
    pub group_by_fields: Option<Vec<String>>,
    /// The query to preview.
    #[serde(rename = "query")]
    pub query: String,
    /// Index of the query in the rule.
    #[serde(rename = "queryIndex")]
    pub query_index: i32,
    /// The rule type.
    #[serde(rename = "type")]
    pub type_: crate::datadogV2::model::SecurityMonitoringRuleTypeRead,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl SecurityMonitoringRuleLivetailRequest {
    pub fn new(
        data_source: String,
        detection_method: crate::datadogV2::model::SecurityMonitoringRuleDetectionMethod,
        query: String,
        query_index: i32,
        type_: crate::datadogV2::model::SecurityMonitoringRuleTypeRead,
    ) -> SecurityMonitoringRuleLivetailRequest {
        SecurityMonitoringRuleLivetailRequest {
            data_source,
            detection_method,
            distinct_fields: None,
            filters: None,
            group_by_fields: None,
            query,
            query_index,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn distinct_fields(mut self, value: Vec<String>) -> Self {
        self.distinct_fields = Some(value);
        self
    }

    pub fn filters(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringFilter>,
    ) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn group_by_fields(mut self, value: Vec<String>) -> Self {
        self.group_by_fields = Some(value);
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

impl<'de> Deserialize<'de> for SecurityMonitoringRuleLivetailRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SecurityMonitoringRuleLivetailRequestVisitor;
        impl<'a> Visitor<'a> for SecurityMonitoringRuleLivetailRequestVisitor {
            type Value = SecurityMonitoringRuleLivetailRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<String> = None;
                let mut detection_method: Option<
                    crate::datadogV2::model::SecurityMonitoringRuleDetectionMethod,
                > = None;
                let mut distinct_fields: Option<Vec<String>> = None;
                let mut filters: Option<Vec<crate::datadogV2::model::SecurityMonitoringFilter>> =
                    None;
                let mut group_by_fields: Option<Vec<String>> = None;
                let mut query: Option<String> = None;
                let mut query_index: Option<i32> = None;
                let mut type_: Option<crate::datadogV2::model::SecurityMonitoringRuleTypeRead> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "dataSource" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "detectionMethod" => {
                            detection_method =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _detection_method) = detection_method {
                                match _detection_method {
                                    crate::datadogV2::model::SecurityMonitoringRuleDetectionMethod::UnparsedObject(_detection_method) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "distinctFields" => {
                            if v.is_null() {
                                continue;
                            }
                            distinct_fields =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "filters" => {
                            if v.is_null() {
                                continue;
                            }
                            filters = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "groupByFields" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by_fields =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queryIndex" => {
                            query_index =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV2::model::SecurityMonitoringRuleTypeRead::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
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
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let detection_method =
                    detection_method.ok_or_else(|| M::Error::missing_field("detection_method"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let query_index =
                    query_index.ok_or_else(|| M::Error::missing_field("query_index"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = SecurityMonitoringRuleLivetailRequest {
                    data_source,
                    detection_method,
                    distinct_fields,
                    filters,
                    group_by_fields,
                    query,
                    query_index,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(SecurityMonitoringRuleLivetailRequestVisitor)
    }
}
