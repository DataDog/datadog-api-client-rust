// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The updated configuration of the report schedule. These values replace the existing
/// ones; the targeted resource (`resource_id` and `resource_type`) cannot be changed.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ReportSchedulePatchRequestAttributes {
    /// How a PDF-export report is delivered. `pdf` attaches a PDF file, `png` embeds
    /// an inline PNG image, and `pdf_and_png` delivers both.
    #[serde(rename = "delivery_format")]
    pub delivery_format: Option<crate::datadogV2::model::ReportScheduleDeliveryFormat>,
    /// A description of the report, up to 4096 characters.
    #[serde(rename = "description")]
    pub description: String,
    /// The recipients of the report. Each entry is an email address, a Slack channel
    /// reference in the form `slack:{team_id}.{channel_id}.{channel_name}`, or a Microsoft
    /// Teams channel reference in the form `teams:{tenant_id}|{team_id}|{channel_id}`.
    #[serde(rename = "recipients")]
    pub recipients: Vec<String>,
    /// The recurrence rule for the schedule, expressed as an iCalendar `RRULE` string.
    #[serde(rename = "rrule")]
    pub rrule: String,
    /// The identifier of the dashboard tab to render, when the dashboard has tabs.
    #[serde(rename = "tab_id")]
    pub tab_id: Option<uuid::Uuid>,
    /// The dashboard template variables applied when rendering the report.
    #[serde(rename = "template_variables")]
    pub template_variables: Vec<crate::datadogV2::model::ReportScheduleTemplateVariable>,
    /// The relative timeframe of data to include in the report.
    #[serde(rename = "timeframe")]
    pub timeframe: String,
    /// The IANA time zone identifier the recurrence rule is evaluated in.
    #[serde(rename = "timezone")]
    pub timezone: String,
    /// The title of the report, between 1 and 78 characters.
    #[serde(rename = "title")]
    pub title: String,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ReportSchedulePatchRequestAttributes {
    pub fn new(
        description: String,
        recipients: Vec<String>,
        rrule: String,
        template_variables: Vec<crate::datadogV2::model::ReportScheduleTemplateVariable>,
        timeframe: String,
        timezone: String,
        title: String,
    ) -> ReportSchedulePatchRequestAttributes {
        ReportSchedulePatchRequestAttributes {
            delivery_format: None,
            description,
            recipients,
            rrule,
            tab_id: None,
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
        value: crate::datadogV2::model::ReportScheduleDeliveryFormat,
    ) -> Self {
        self.delivery_format = Some(value);
        self
    }

    pub fn tab_id(mut self, value: uuid::Uuid) -> Self {
        self.tab_id = Some(value);
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

impl<'de> Deserialize<'de> for ReportSchedulePatchRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReportSchedulePatchRequestAttributesVisitor;
        impl<'a> Visitor<'a> for ReportSchedulePatchRequestAttributesVisitor {
            type Value = ReportSchedulePatchRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut delivery_format: Option<
                    crate::datadogV2::model::ReportScheduleDeliveryFormat,
                > = None;
                let mut description: Option<String> = None;
                let mut recipients: Option<Vec<String>> = None;
                let mut rrule: Option<String> = None;
                let mut tab_id: Option<uuid::Uuid> = None;
                let mut template_variables: Option<
                    Vec<crate::datadogV2::model::ReportScheduleTemplateVariable>,
                > = None;
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
                        "delivery_format" => {
                            if v.is_null() {
                                continue;
                            }
                            delivery_format =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _delivery_format) = delivery_format {
                                match _delivery_format {
                                    crate::datadogV2::model::ReportScheduleDeliveryFormat::UnparsedObject(_delivery_format) => {
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
                        "recipients" => {
                            recipients = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "rrule" => {
                            rrule = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tab_id" => {
                            if v.is_null() {
                                continue;
                            }
                            tab_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                let recipients = recipients.ok_or_else(|| M::Error::missing_field("recipients"))?;
                let rrule = rrule.ok_or_else(|| M::Error::missing_field("rrule"))?;
                let template_variables = template_variables
                    .ok_or_else(|| M::Error::missing_field("template_variables"))?;
                let timeframe = timeframe.ok_or_else(|| M::Error::missing_field("timeframe"))?;
                let timezone = timezone.ok_or_else(|| M::Error::missing_field("timezone"))?;
                let title = title.ok_or_else(|| M::Error::missing_field("title"))?;

                let content = ReportSchedulePatchRequestAttributes {
                    delivery_format,
                    description,
                    recipients,
                    rrule,
                    tab_id,
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

        deserializer.deserialize_any(ReportSchedulePatchRequestAttributesVisitor)
    }
}
