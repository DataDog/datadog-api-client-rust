// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The configuration for a print-only report. Specify exactly one of `timeframe` (for a
/// relative time window) or both `from_ts` and `to_ts` (for an absolute time range).
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PrintReportRequestAttributes {
    /// The start of an absolute time range, as a Unix timestamp in milliseconds.
    /// Required when `timeframe` is omitted.
    #[serde(rename = "from_ts")]
    pub from_ts: Option<i64>,
    /// The identifier of the dashboard or integration dashboard to render.
    #[serde(rename = "resource_id")]
    pub resource_id: String,
    /// The type of dashboard resource the report schedule targets.
    #[serde(rename = "resource_type")]
    pub resource_type: crate::datadogV2::model::ReportScheduleResourceType,
    /// The dashboard template variables applied when rendering the report.
    #[serde(rename = "template_variables")]
    pub template_variables: Vec<crate::datadogV2::model::ReportScheduleTemplateVariable>,
    /// A relative time window (for example `1w` or `calendar_month`). Mutually
    /// exclusive with `from_ts` and `to_ts`.
    #[serde(rename = "timeframe")]
    pub timeframe: Option<String>,
    /// The IANA time zone identifier used to evaluate the time window.
    #[serde(rename = "timezone")]
    pub timezone: String,
    /// The end of an absolute time range, as a Unix timestamp in milliseconds.
    /// Required when `timeframe` is omitted.
    #[serde(rename = "to_ts")]
    pub to_ts: Option<i64>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl PrintReportRequestAttributes {
    pub fn new(
        resource_id: String,
        resource_type: crate::datadogV2::model::ReportScheduleResourceType,
        template_variables: Vec<crate::datadogV2::model::ReportScheduleTemplateVariable>,
        timezone: String,
    ) -> PrintReportRequestAttributes {
        PrintReportRequestAttributes {
            from_ts: None,
            resource_id,
            resource_type,
            template_variables,
            timeframe: None,
            timezone,
            to_ts: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn from_ts(mut self, value: i64) -> Self {
        self.from_ts = Some(value);
        self
    }

    pub fn timeframe(mut self, value: String) -> Self {
        self.timeframe = Some(value);
        self
    }

    pub fn to_ts(mut self, value: i64) -> Self {
        self.to_ts = Some(value);
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

impl<'de> Deserialize<'de> for PrintReportRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PrintReportRequestAttributesVisitor;
        impl<'a> Visitor<'a> for PrintReportRequestAttributesVisitor {
            type Value = PrintReportRequestAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut from_ts: Option<i64> = None;
                let mut resource_id: Option<String> = None;
                let mut resource_type: Option<crate::datadogV2::model::ReportScheduleResourceType> =
                    None;
                let mut template_variables: Option<
                    Vec<crate::datadogV2::model::ReportScheduleTemplateVariable>,
                > = None;
                let mut timeframe: Option<String> = None;
                let mut timezone: Option<String> = None;
                let mut to_ts: Option<i64> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "from_ts" => {
                            if v.is_null() {
                                continue;
                            }
                            from_ts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                        "template_variables" => {
                            template_variables =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timeframe" => {
                            if v.is_null() {
                                continue;
                            }
                            timeframe = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "timezone" => {
                            timezone = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "to_ts" => {
                            if v.is_null() {
                                continue;
                            }
                            to_ts = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let resource_id =
                    resource_id.ok_or_else(|| M::Error::missing_field("resource_id"))?;
                let resource_type =
                    resource_type.ok_or_else(|| M::Error::missing_field("resource_type"))?;
                let template_variables = template_variables
                    .ok_or_else(|| M::Error::missing_field("template_variables"))?;
                let timezone = timezone.ok_or_else(|| M::Error::missing_field("timezone"))?;

                let content = PrintReportRequestAttributes {
                    from_ts,
                    resource_id,
                    resource_type,
                    template_variables,
                    timeframe,
                    timezone,
                    to_ts,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(PrintReportRequestAttributesVisitor)
    }
}
