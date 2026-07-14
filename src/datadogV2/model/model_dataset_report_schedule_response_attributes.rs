// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The configuration and derived state of a report schedule for a published dataset.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DatasetReportScheduleResponseAttributes {
    /// The identifier of the notebook cell that published the dataset, or `null` if not set.
    #[serialize_always]
    #[serde(rename = "cell_id")]
    pub cell_id: Option<String>,
    /// The identifier of the dataset, or `null` if not set.
    #[serialize_always]
    #[serde(rename = "dataset_id")]
    pub dataset_id: Option<String>,
    /// The description of the report.
    #[serde(rename = "description")]
    pub description: String,
    /// The maximum number of rows included in the attached CSV file, or `null` if not set.
    #[serialize_always]
    #[serde(rename = "file_row_limit")]
    pub file_row_limit: Option<i64>,
    /// The maximum number of rows included inline in the email body, or `null` if not set.
    #[serialize_always]
    #[serde(rename = "inline_row_limit")]
    pub inline_row_limit: Option<i64>,
    /// The Unix timestamp, in milliseconds, of the next scheduled delivery, or
    /// `null` if none is scheduled.
    #[serialize_always]
    #[serde(rename = "next_recurrence")]
    pub next_recurrence: Option<i64>,
    /// The identifier of the notebook containing the dataset cell, or `null` if not set.
    #[serialize_always]
    #[serde(rename = "notebook_id")]
    pub notebook_id: Option<i64>,
    /// The recipients of the report (email addresses, Slack channel references, or
    /// Microsoft Teams channel references).
    #[serde(rename = "recipients")]
    pub recipients: Vec<String>,
    /// The identifier of the widget containing the dataset.
    #[serde(rename = "resource_id")]
    pub resource_id: String,
    /// The type of resource targeted by a dataset report schedule.
    #[serde(rename = "resource_type")]
    pub resource_type: crate::datadogV2::model::DatasetReportScheduleResourceType,
    /// The recurrence rule for the schedule, expressed as an iCalendar `RRULE` string.
    #[serde(rename = "rrule")]
    pub rrule: String,
    /// Whether the schedule is currently delivering reports (`active`) or paused (`inactive`).
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::ReportScheduleStatus,
    /// The relative timeframe of data included in the report.
    #[serde(rename = "timeframe")]
    pub timeframe: String,
    /// The IANA time zone identifier the recurrence rule is evaluated in.
    #[serde(rename = "timezone")]
    pub timezone: String,
    /// The title of the report.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DatasetReportScheduleResponseAttributes {
    pub fn new(
        cell_id: Option<String>,
        dataset_id: Option<String>,
        description: String,
        file_row_limit: Option<i64>,
        inline_row_limit: Option<i64>,
        next_recurrence: Option<i64>,
        notebook_id: Option<i64>,
        recipients: Vec<String>,
        resource_id: String,
        resource_type: crate::datadogV2::model::DatasetReportScheduleResourceType,
        rrule: String,
        status: crate::datadogV2::model::ReportScheduleStatus,
        timeframe: String,
        timezone: String,
        title: String,
    ) -> DatasetReportScheduleResponseAttributes {
        DatasetReportScheduleResponseAttributes {
            cell_id,
            dataset_id,
            description,
            file_row_limit,
            inline_row_limit,
            next_recurrence,
            notebook_id,
            recipients,
            resource_id,
            resource_type,
            rrule,
            status,
            timeframe,
            timezone,
            title,
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

impl<'de> Deserialize<'de> for DatasetReportScheduleResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatasetReportScheduleResponseAttributesVisitor;
        impl<'a> Visitor<'a> for DatasetReportScheduleResponseAttributesVisitor {
            type Value = DatasetReportScheduleResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut cell_id: Option<Option<String>> = None;
                let mut dataset_id: Option<Option<String>> = None;
                let mut description: Option<String> = None;
                let mut file_row_limit: Option<Option<i64>> = None;
                let mut inline_row_limit: Option<Option<i64>> = None;
                let mut next_recurrence: Option<Option<i64>> = None;
                let mut notebook_id: Option<Option<i64>> = None;
                let mut recipients: Option<Vec<String>> = None;
                let mut resource_id: Option<String> = None;
                let mut resource_type: Option<
                    crate::datadogV2::model::DatasetReportScheduleResourceType,
                > = None;
                let mut rrule: Option<String> = None;
                let mut status: Option<crate::datadogV2::model::ReportScheduleStatus> = None;
                let mut timeframe: Option<String> = None;
                let mut timezone: Option<String> = None;
                let mut title: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "cell_id" => {
                            cell_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "dataset_id" => {
                            dataset_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "file_row_limit" => {
                            file_row_limit =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "inline_row_limit" => {
                            inline_row_limit =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "next_recurrence" => {
                            next_recurrence =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "notebook_id" => {
                            notebook_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "recipients" => {
                            recipients = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_id" => {
                            resource_id =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "resource_type" => {
                            resource_type =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _resource_type) = resource_type {
                                match _resource_type {
                                    crate::datadogV2::model::DatasetReportScheduleResourceType::UnparsedObject(_resource_type) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "rrule" => {
                            rrule = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::ReportScheduleStatus::UnparsedObject(_status) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "timeframe" => {
                            timeframe = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timezone" => {
                            timezone = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let cell_id = cell_id.ok_or_else(|| M::Error::missing_field("cell_id"))?;
                let dataset_id = dataset_id.ok_or_else(|| M::Error::missing_field("dataset_id"))?;
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let file_row_limit =
                    file_row_limit.ok_or_else(|| M::Error::missing_field("file_row_limit"))?;
                let inline_row_limit =
                    inline_row_limit.ok_or_else(|| M::Error::missing_field("inline_row_limit"))?;
                let next_recurrence =
                    next_recurrence.ok_or_else(|| M::Error::missing_field("next_recurrence"))?;
                let notebook_id =
                    notebook_id.ok_or_else(|| M::Error::missing_field("notebook_id"))?;
                let recipients = recipients.ok_or_else(|| M::Error::missing_field("recipients"))?;
                let resource_id =
                    resource_id.ok_or_else(|| M::Error::missing_field("resource_id"))?;
                let resource_type =
                    resource_type.ok_or_else(|| M::Error::missing_field("resource_type"))?;
                let rrule = rrule.ok_or_else(|| M::Error::missing_field("rrule"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let timeframe = timeframe.ok_or_else(|| M::Error::missing_field("timeframe"))?;
                let timezone = timezone.ok_or_else(|| M::Error::missing_field("timezone"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;

                let content = DatasetReportScheduleResponseAttributes {
                    cell_id,
                    dataset_id,
                    description,
                    file_row_limit,
                    inline_row_limit,
                    next_recurrence,
                    notebook_id,
                    recipients,
                    resource_id,
                    resource_type,
                    rrule,
                    status,
                    timeframe,
                    timezone,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DatasetReportScheduleResponseAttributesVisitor)
    }
}
