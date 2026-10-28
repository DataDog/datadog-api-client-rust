// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A monitor group that is not ok after a run.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DataObservabilityMonitorRunGroup {
    /// The account of the entity, when the platform has one.
    #[serde(rename = "account")]
    pub account: Option<String>,
    /// The column of the entity, when it is a column.
    #[serde(rename = "column")]
    pub column: Option<String>,
    /// The database of the entity.
    #[serde(rename = "database")]
    pub database: String,
    /// The ID of the table or column the group checks.
    #[serde(rename = "entity_id")]
    pub entity_id: String,
    /// The type of the entity, such as table or column.
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    /// The group's name in Datadog monitors, as the monitor API and UI show it. Pass it as `filter[group]` to list the group's evaluations.
    #[serde(rename = "group")]
    pub group: String,
    /// The values of the monitor's group-by columns for this group. `N/A` means the column had no value.
    #[serde(rename = "group_by_values")]
    pub group_by_values: Option<std::collections::BTreeMap<String, String>>,
    /// The data platform of the entity.
    #[serde(rename = "platform")]
    pub platform: String,
    /// The schema of the entity, when it has one.
    #[serde(rename = "schema")]
    pub schema: Option<String>,
    /// The state of a monitor group, from the alerting engine.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::DataObservabilityMonitorGroupStatus,
    /// The table of the entity, when it is a table or column.
    #[serde(rename = "table")]
    pub table: Option<String>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DataObservabilityMonitorRunGroup {
    pub fn new(
        database: String,
        entity_id: String,
        entity_type: String,
        group: String,
        platform: String,
        status: crate::datadogV2::model::DataObservabilityMonitorGroupStatus,
    ) -> DataObservabilityMonitorRunGroup {
        DataObservabilityMonitorRunGroup {
            account: None,
            column: None,
            database,
            entity_id,
            entity_type,
            group,
            group_by_values: None,
            platform,
            schema: None,
            status,
            table: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn account(mut self, value: String) -> Self {
        self.account = Some(value);
        self
    }

    pub fn column(mut self, value: String) -> Self {
        self.column = Some(value);
        self
    }

    pub fn group_by_values(mut self, value: std::collections::BTreeMap<String, String>) -> Self {
        self.group_by_values = Some(value);
        self
    }

    pub fn schema(mut self, value: String) -> Self {
        self.schema = Some(value);
        self
    }

    pub fn table(mut self, value: String) -> Self {
        self.table = Some(value);
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

impl<'de> Deserialize<'de> for DataObservabilityMonitorRunGroup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DataObservabilityMonitorRunGroupVisitor;
        impl<'a> Visitor<'a> for DataObservabilityMonitorRunGroupVisitor {
            type Value = DataObservabilityMonitorRunGroup;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut account: Option<String> = None;
                let mut column: Option<String> = None;
                let mut database: Option<String> = None;
                let mut entity_id: Option<String> = None;
                let mut entity_type: Option<String> = None;
                let mut group: Option<String> = None;
                let mut group_by_values: Option<std::collections::BTreeMap<String, String>> = None;
                let mut platform: Option<String> = None;
                let mut schema: Option<String> = None;
                let mut status: Option<
                    crate::datadogV2::model::DataObservabilityMonitorGroupStatus,
                > = None;
                let mut table: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "account" => {
                            if v.is_null() {
                                continue;
                            }
                            account = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "column" => {
                            if v.is_null() {
                                continue;
                            }
                            column = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "database" => {
                            database = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "entity_id" => {
                            entity_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "entity_type" => {
                            entity_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group" => {
                            group = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by_values" => {
                            if v.is_null() {
                                continue;
                            }
                            group_by_values =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "platform" => {
                            platform = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "schema" => {
                            if v.is_null() {
                                continue;
                            }
                            schema = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::DataObservabilityMonitorGroupStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "table" => {
                            if v.is_null() {
                                continue;
                            }
                            table = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let database = database.ok_or_else(|| M::Error::missing_field("database"))?;
                let entity_id = entity_id.ok_or_else(|| M::Error::missing_field("entity_id"))?;
                let entity_type =
                    entity_type.ok_or_else(|| M::Error::missing_field("entity_type"))?;
                let group = group.ok_or_else(|| M::Error::missing_field("group"))?;
                let platform = platform.ok_or_else(|| M::Error::missing_field("platform"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = DataObservabilityMonitorRunGroup {
                    account,
                    column,
                    database,
                    entity_id,
                    entity_type,
                    group,
                    group_by_values,
                    platform,
                    schema,
                    status,
                    table,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DataObservabilityMonitorRunGroupVisitor)
    }
}
