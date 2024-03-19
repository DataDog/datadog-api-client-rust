// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The Distribution visualization is another way of showing metrics
/// aggregated across one or several tags, such as hosts.
/// Unlike the heat map, a distribution graph’s x-axis is quantity rather than time.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DistributionWidgetDefinition {
    /// A list of custom links.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>>,
    /// (Deprecated) The widget legend was replaced by a tooltip and sidebar.
    #[deprecated]
    #[serde(rename = "legend_size")]
    pub legend_size: Option<String>,
    /// List of markers.
    #[serde(rename = "markers")]
    pub markers: Option<Vec<crate::datadogV1::model::WidgetMarker>>,
    /// Array of one request object to display in the widget.
    ///
    /// See the dedicated [Request JSON schema documentation](<https://docs.datadoghq.com/dashboards/graphing_json/request_json>)
    ///  to learn how to build the `REQUEST_SCHEMA`.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::datadogV1::model::DistributionWidgetRequest>,
    /// (Deprecated) The widget legend was replaced by a tooltip and sidebar.
    #[deprecated]
    #[serde(rename = "show_legend")]
    pub show_legend: Option<bool>,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV1::model::WidgetTime>,
    /// Title of the widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the distribution widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::DistributionWidgetDefinitionType,
    /// X Axis controls for the distribution widget.
    #[serde(rename = "xaxis")]
    pub xaxis: Option<crate::datadogV1::model::DistributionWidgetXAxis>,
    /// Y Axis controls for the distribution widget.
    #[serde(rename = "yaxis")]
    pub yaxis: Option<crate::datadogV1::model::DistributionWidgetYAxis>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DistributionWidgetDefinition {
    pub fn new(
        requests: Vec<crate::datadogV1::model::DistributionWidgetRequest>,
        type_: crate::datadogV1::model::DistributionWidgetDefinitionType,
    ) -> DistributionWidgetDefinition {
        #[allow(deprecated)]
        DistributionWidgetDefinition {
            custom_links: None,
            legend_size: None,
            markers: None,
            requests,
            show_legend: None,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            xaxis: None,
            yaxis: None,
            _unparsed: false,
        }
    }

    #[allow(deprecated)]
    pub fn custom_links(mut self, value: Vec<crate::datadogV1::model::WidgetCustomLink>) -> Self {
        self.custom_links = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn legend_size(mut self, value: String) -> Self {
        self.legend_size = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn markers(mut self, value: Vec<crate::datadogV1::model::WidgetMarker>) -> Self {
        self.markers = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn show_legend(mut self, value: bool) -> Self {
        self.show_legend = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn time(mut self, value: crate::datadogV1::model::WidgetTime) -> Self {
        self.time = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn title_align(mut self, value: crate::datadogV1::model::WidgetTextAlign) -> Self {
        self.title_align = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn title_size(mut self, value: String) -> Self {
        self.title_size = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn xaxis(mut self, value: crate::datadogV1::model::DistributionWidgetXAxis) -> Self {
        self.xaxis = Some(value);
        self
    }

    #[allow(deprecated)]
    pub fn yaxis(mut self, value: crate::datadogV1::model::DistributionWidgetYAxis) -> Self {
        self.yaxis = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for DistributionWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DistributionWidgetDefinitionVisitor;
        impl<'a> Visitor<'a> for DistributionWidgetDefinitionVisitor {
            type Value = DistributionWidgetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>> = None;
                let mut legend_size: Option<String> = None;
                let mut markers: Option<Vec<crate::datadogV1::model::WidgetMarker>> = None;
                let mut requests: Option<Vec<crate::datadogV1::model::DistributionWidgetRequest>> =
                    None;
                let mut show_legend: Option<bool> = None;
                let mut time: Option<crate::datadogV1::model::WidgetTime> = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::DistributionWidgetDefinitionType> =
                    None;
                let mut xaxis: Option<crate::datadogV1::model::DistributionWidgetXAxis> = None;
                let mut yaxis: Option<crate::datadogV1::model::DistributionWidgetYAxis> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "custom_links" => {
                            if v.is_null() {
                                continue;
                            }
                            custom_links =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "legend_size" => {
                            if v.is_null() {
                                continue;
                            }
                            legend_size =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "markers" => {
                            if v.is_null() {
                                continue;
                            }
                            markers = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "requests" => {
                            requests = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "show_legend" => {
                            if v.is_null() {
                                continue;
                            }
                            show_legend =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "time" => {
                            if v.is_null() {
                                continue;
                            }
                            time = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title" => {
                            if v.is_null() {
                                continue;
                            }
                            title = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "title_align" => {
                            if v.is_null() {
                                continue;
                            }
                            title_align =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _title_align) = title_align {
                                match _title_align {
                                    crate::datadogV1::model::WidgetTextAlign::UnparsedObject(
                                        _title_align,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        "title_size" => {
                            if v.is_null() {
                                continue;
                            }
                            title_size = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::DistributionWidgetDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "xaxis" => {
                            if v.is_null() {
                                continue;
                            }
                            xaxis = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "yaxis" => {
                            if v.is_null() {
                                continue;
                            }
                            yaxis = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let requests = requests.ok_or_else(|| M::Error::missing_field("requests"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                #[allow(deprecated)]
                let content = DistributionWidgetDefinition {
                    custom_links,
                    legend_size,
                    markers,
                    requests,
                    show_legend,
                    time,
                    title,
                    title_align,
                    title_size,
                    type_,
                    xaxis,
                    yaxis,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DistributionWidgetDefinitionVisitor)
    }
}
