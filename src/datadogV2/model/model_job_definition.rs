// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Definition of a historical job.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct JobDefinition {
    /// Calculated fields.
    #[serde(rename = "calculatedFields")]
    pub calculated_fields: Option<Vec<crate::datadogV2::model::CalculatedField>>,
    /// Cases used for generating job results.
    #[serde(rename = "cases")]
    pub cases: Vec<crate::datadogV2::model::SecurityMonitoringRuleCaseCreate>,
    /// Starting time of data analyzed by the job.
    #[serde(rename = "from")]
    pub from: i64,
    /// Additional grouping to perform on top of the existing groups in the query section. Must be a subset of the existing groups.
    #[serde(rename = "groupSignalsBy")]
    pub group_signals_by: Option<Vec<String>>,
    /// Index used to load the data.
    #[serde(rename = "index")]
    pub index: String,
    /// Message for generated results.
    #[serde(rename = "message")]
    pub message: String,
    /// Job name.
    #[serde(rename = "name")]
    pub name: String,
    /// Job options.
    #[serde(rename = "options")]
    pub options: Option<crate::datadogV2::model::HistoricalJobOptions>,
    /// Queries for selecting logs analyzed by the job.
    #[serde(rename = "queries")]
    pub queries: Vec<crate::datadogV2::model::HistoricalJobQuery>,
    /// Reference tables used in the queries.
    #[serde(rename = "referenceTables")]
    pub reference_tables: Option<Vec<crate::datadogV2::model::SecurityMonitoringReferenceTable>>,
    /// Tags for generated signals.
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
    /// Cases for generating results from third-party detection method. Only available for third-party detection method.
    #[serde(rename = "thirdPartyCases")]
    pub third_party_cases:
        Option<Vec<crate::datadogV2::model::SecurityMonitoringThirdPartyRuleCaseCreate>>,
    /// Ending time of data analyzed by the job.
    #[serde(rename = "to")]
    pub to: i64,
    /// Job type.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl JobDefinition {
    pub fn new(
        cases: Vec<crate::datadogV2::model::SecurityMonitoringRuleCaseCreate>,
        from: i64,
        index: String,
        message: String,
        name: String,
        queries: Vec<crate::datadogV2::model::HistoricalJobQuery>,
        to: i64,
    ) -> JobDefinition {
        JobDefinition {
            calculated_fields: None,
            cases,
            from,
            group_signals_by: None,
            index,
            message,
            name,
            options: None,
            queries,
            reference_tables: None,
            tags: None,
            third_party_cases: None,
            to,
            type_: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn calculated_fields(
        mut self,
        value: Vec<crate::datadogV2::model::CalculatedField>,
    ) -> Self {
        self.calculated_fields = Some(value);
        self
    }

    pub fn group_signals_by(mut self, value: Vec<String>) -> Self {
        self.group_signals_by = Some(value);
        self
    }

    pub fn options(mut self, value: crate::datadogV2::model::HistoricalJobOptions) -> Self {
        self.options = Some(value);
        self
    }

    pub fn reference_tables(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringReferenceTable>,
    ) -> Self {
        self.reference_tables = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn third_party_cases(
        mut self,
        value: Vec<crate::datadogV2::model::SecurityMonitoringThirdPartyRuleCaseCreate>,
    ) -> Self {
        self.third_party_cases = Some(value);
        self
    }

    pub fn type_(mut self, value: String) -> Self {
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

impl<'de> Deserialize<'de> for JobDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct JobDefinitionVisitor;
        impl<'a> Visitor<'a> for JobDefinitionVisitor {
            type Value = JobDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut calculated_fields: Option<Vec<crate::datadogV2::model::CalculatedField>> =
                    None;
                let mut cases: Option<
                    Vec<crate::datadogV2::model::SecurityMonitoringRuleCaseCreate>,
                > = None;
                let mut from: Option<i64> = None;
                let mut group_signals_by: Option<Vec<String>> = None;
                let mut index: Option<String> = None;
                let mut message: Option<String> = None;
                let mut name: Option<String> = None;
                let mut options: Option<crate::datadogV2::model::HistoricalJobOptions> = None;
                let mut queries: Option<Vec<crate::datadogV2::model::HistoricalJobQuery>> = None;
                let mut reference_tables: Option<
                    Vec<crate::datadogV2::model::SecurityMonitoringReferenceTable>,
                > = None;
                let mut tags: Option<Vec<String>> = None;
                let mut third_party_cases: Option<
                    Vec<crate::datadogV2::model::SecurityMonitoringThirdPartyRuleCaseCreate>,
                > = None;
                let mut to: Option<i64> = None;
                let mut type_: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "calculatedFields" => {
                            if v.is_null() {
                                continue;
                            }
                            calculated_fields =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "cases" => {
                            cases = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "from" => {
                            from = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "groupSignalsBy" => {
                            if v.is_null() {
                                continue;
                            }
                            group_signals_by =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "index" => {
                            index = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "message" => {
                            message = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "name" => {
                            name = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "options" => {
                            if v.is_null() {
                                continue;
                            }
                            options = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "queries" => {
                            queries = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "referenceTables" => {
                            if v.is_null() {
                                continue;
                            }
                            reference_tables =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tags" => {
                            if v.is_null() {
                                continue;
                            }
                            tags = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "thirdPartyCases" => {
                            if v.is_null() {
                                continue;
                            }
                            third_party_cases =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to" => {
                            to = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            if v.is_null() {
                                continue;
                            }
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let cases = cases.ok_or_else(|| M::Error::missing_field("cases"))?;
                let from = from.ok_or_else(|| M::Error::missing_field("from"))?;
                let index = index.ok_or_else(|| M::Error::missing_field("index"))?;
                let message = message.ok_or_else(|| M::Error::missing_field("message"))?;
                let name = name.ok_or_else(|| M::Error::missing_field("name"))?;
                let queries = queries.ok_or_else(|| M::Error::missing_field("queries"))?;
                let to = to.ok_or_else(|| M::Error::missing_field("to"))?;

                let content = JobDefinition {
                    calculated_fields,
                    cases,
                    from,
                    group_signals_by,
                    index,
                    message,
                    name,
                    options,
                    queries,
                    reference_tables,
                    tags,
                    third_party_cases,
                    to,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(JobDefinitionVisitor)
    }
}
