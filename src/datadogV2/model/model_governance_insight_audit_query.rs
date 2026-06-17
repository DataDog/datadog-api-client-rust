// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An audit log query used to compute an insight value.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GovernanceInsightAuditQuery {
    /// The aggregation applied to an audit log query.
    #[serde(rename = "compute")]
    pub compute: crate::datadogV2::model::GovernanceInsightAuditCompute,
    /// The audit log indexes the query runs against.
    #[serde(rename = "indexes")]
    pub indexes: Vec<String>,
    /// The audit log search query string.
    #[serde(rename = "query")]
    pub query: String,
    /// The data source the query runs against.
    #[serde(rename = "source")]
    pub source: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GovernanceInsightAuditQuery {
    pub fn new(
        compute: crate::datadogV2::model::GovernanceInsightAuditCompute,
        indexes: Vec<String>,
        query: String,
        source: String,
    ) -> GovernanceInsightAuditQuery {
        GovernanceInsightAuditQuery {
            compute,
            indexes,
            query,
            source,
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

impl<'de> Deserialize<'de> for GovernanceInsightAuditQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GovernanceInsightAuditQueryVisitor;
        impl<'a> Visitor<'a> for GovernanceInsightAuditQueryVisitor {
            type Value = GovernanceInsightAuditQuery;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut compute: Option<crate::datadogV2::model::GovernanceInsightAuditCompute> =
                    None;
                let mut indexes: Option<Vec<String>> = None;
                let mut query: Option<String> = None;
                let mut source: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "compute" => {
                            compute = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "indexes" => {
                            indexes = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query" => {
                            query = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "source" => {
                            source = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let compute = compute.ok_or_else(|| M::Error::missing_field("compute"))?;
                let indexes = indexes.ok_or_else(|| M::Error::missing_field("indexes"))?;
                let query = query.ok_or_else(|| M::Error::missing_field("query"))?;
                let source = source.ok_or_else(|| M::Error::missing_field("source"))?;

                let content = GovernanceInsightAuditQuery {
                    compute,
                    indexes,
                    query,
                    source,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GovernanceInsightAuditQueryVisitor)
    }
}
