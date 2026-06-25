// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The configuration and derived state of a report schedule in a list response.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ReportScheduleListResponseAttributes {
    /// The delivery format for dashboard report schedules, or `null` if not set.
    #[serde(
        rename = "delivery_format",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub delivery_format:
        Option<Option<crate::datadogV2::model::ReportScheduleResponseAttributesDeliveryFormat>>,
    /// The description of the report.
    #[serde(rename = "description")]
    pub description: String,
    /// The Unix timestamp, in milliseconds, of the next scheduled delivery, or `null` if none is scheduled.
    #[serialize_always]
    #[serde(rename = "next_recurrence")]
    pub next_recurrence: Option<i64>,
    /// The recipients of the report (email addresses, Slack channel references, or Microsoft Teams channel references).
    #[serde(rename = "recipients")]
    pub recipients: Vec<String>,
    /// The identifier of the resource rendered in the report.
    #[serde(rename = "resource_id")]
    pub resource_id: String,
    /// The type of dashboard resource the report schedule targets.
    #[serde(rename = "resource_type")]
    pub resource_type: crate::datadogV2::model::ReportScheduleResourceType,
    /// The recurrence rule for the schedule, expressed as an iCalendar `RRULE` string.
    #[serde(rename = "rrule")]
    pub rrule: String,
    /// Whether the schedule is currently delivering reports (`active`) or paused (`inactive`).
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::ReportScheduleStatus,
    /// The dashboard template variables applied when rendering the report.
    #[serde(rename = "template_variables")]
    pub template_variables: Vec<crate::datadogV2::model::ReportScheduleTemplateVariable>,
    /// The relative timeframe of data included in the report, or `null` if not set.
    #[serialize_always]
    #[serde(rename = "timeframe")]
    pub timeframe: Option<String>,
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

impl ReportScheduleListResponseAttributes {
    pub fn new(
        description: String,
        next_recurrence: Option<i64>,
        recipients: Vec<String>,
        resource_id: String,
        resource_type: crate::datadogV2::model::ReportScheduleResourceType,
        rrule: String,
        status: crate::datadogV2::model::ReportScheduleStatus,
        template_variables: Vec<crate::datadogV2::model::ReportScheduleTemplateVariable>,
        timeframe: Option<String>,
        timezone: String,
        title: String,
    ) -> ReportScheduleListResponseAttributes {
        ReportScheduleListResponseAttributes {
            delivery_format: None,
            description,
            next_recurrence,
            recipients,
            resource_id,
            resource_type,
            rrule,
            status,
            template_variables,
            timeframe,
            timezone,
            title,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn delivery_format(
        mut self,
        value: Option<crate::datadogV2::model::ReportScheduleResponseAttributesDeliveryFormat>,
    ) -> Self {
        self.delivery_format = Some(value);
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

impl<'de> Deserialize<'de> for ReportScheduleListResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReportScheduleListResponseAttributesVisitor;
        impl<'a> Visitor<'a> for ReportScheduleListResponseAttributesVisitor {
            type Value = ReportScheduleListResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut delivery_format: Option<
                    Option<crate::datadogV2::model::ReportScheduleResponseAttributesDeliveryFormat>,
                > = None;
                let mut description: Option<String> = None;
                let mut next_recurrence: Option<Option<i64>> = None;
                let mut recipients: Option<Vec<String>> = None;
                let mut resource_id: Option<String> = None;
                let mut resource_type: Option<crate::datadogV2::model::ReportScheduleResourceType> =
                    None;
                let mut rrule: Option<String> = None;
                let mut status: Option<crate::datadogV2::model::ReportScheduleStatus> = None;
                let mut template_variables: Option<
                    Vec<crate::datadogV2::model::ReportScheduleTemplateVariable>,
                > = None;
                let mut timeframe: Option<Option<String>> = None;
                let mut timezone: Option<String> = None;
                let mut title: Option<String> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "delivery_format" => {
                            delivery_format =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _delivery_format) = delivery_format {
                                match _delivery_format {
                                    Some(crate::datadogV2::model::ReportScheduleResponseAttributesDeliveryFormat::UnparsedObject(_delivery_format)) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "description" => {
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "next_recurrence" => {
                            next_recurrence =
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
                                    crate::datadogV2::model::ReportScheduleResourceType::UnparsedObject(_resource_type) => {
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
                        "template_variables" => {
                            template_variables =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let description =
                    description.ok_or_else(|| M::Error::missing_field("description"))?;
                let next_recurrence =
                    next_recurrence.ok_or_else(|| M::Error::missing_field("next_recurrence"))?;
                let recipients = recipients.ok_or_else(|| M::Error::missing_field("recipients"))?;
                let resource_id =
                    resource_id.ok_or_else(|| M::Error::missing_field("resource_id"))?;
                let resource_type =
                    resource_type.ok_or_else(|| M::Error::missing_field("resource_type"))?;
                let rrule = rrule.ok_or_else(|| M::Error::missing_field("rrule"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;
                let template_variables = template_variables
                    .ok_or_else(|| M::Error::missing_field("template_variables"))?;
                let timeframe = timeframe.ok_or_else(|| M::Error::missing_field("timeframe"))?;
                let timezone = timezone.ok_or_else(|| M::Error::missing_field("timezone"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;

                let content = ReportScheduleListResponseAttributes {
                    delivery_format,
                    description,
                    next_recurrence,
                    recipients,
                    resource_id,
                    resource_type,
                    rrule,
                    status,
                    template_variables,
                    timeframe,
                    timezone,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ReportScheduleListResponseAttributesVisitor)
    }
}
