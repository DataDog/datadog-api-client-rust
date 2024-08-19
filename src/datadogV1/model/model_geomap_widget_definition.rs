// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// This visualization displays a series of values by country on a world map.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GeomapWidgetDefinition {
    /// A list of custom links.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>>,
    /// Array of one request object to display in the widget. The request must contain a `group-by` tag whose value is a country ISO code.
    ///
    /// See the [Request JSON schema documentation](<https://docs.datadoghq.com/dashboards/graphing_json/request_json>)
    /// for information about building the `REQUEST_SCHEMA`.
    #[serde(rename = "requests")]
    pub requests: Vec<crate::datadogV1::model::GeomapWidgetRequest>,
    /// The style to apply to the widget.
    #[serde(rename = "style")]
    pub style: crate::datadogV1::model::GeomapWidgetDefinitionStyle,
    /// Time setting for the widget.
    #[serde(rename = "time")]
    pub time: Option<crate::datadogV1::model::WidgetTime>,
    /// The title of your widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// The size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the geomap widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::GeomapWidgetDefinitionType,
    /// The view of the world that the map should render.
    #[serde(rename = "view")]
    pub view: crate::datadogV1::model::GeomapWidgetDefinitionView,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GeomapWidgetDefinition {
    pub fn new(
        requests: Vec<crate::datadogV1::model::GeomapWidgetRequest>,
        style: crate::datadogV1::model::GeomapWidgetDefinitionStyle,
        type_: crate::datadogV1::model::GeomapWidgetDefinitionType,
        view: crate::datadogV1::model::GeomapWidgetDefinitionView,
    ) -> GeomapWidgetDefinition {
        GeomapWidgetDefinition {
            custom_links: None,
            requests,
            style,
            time: None,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            view,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn custom_links(mut self, value: Vec<crate::datadogV1::model::WidgetCustomLink>) -> Self {
        self.custom_links = Some(value);
        self
    }

    pub fn time(mut self, value: crate::datadogV1::model::WidgetTime) -> Self {
        self.time = Some(value);
        self
    }

    pub fn title(mut self, value: String) -> Self {
        self.title = Some(value);
        self
    }

    pub fn title_align(mut self, value: crate::datadogV1::model::WidgetTextAlign) -> Self {
        self.title_align = Some(value);
        self
    }

    pub fn title_size(mut self, value: String) -> Self {
        self.title_size = Some(value);
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

impl<'de> Deserialize<'de> for GeomapWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GeomapWidgetDefinitionVisitor;
        impl<'a> Visitor<'a> for GeomapWidgetDefinitionVisitor {
            type Value = GeomapWidgetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>> = None;
                let mut requests: Option<Vec<crate::datadogV1::model::GeomapWidgetRequest>> = None;
                let mut style: Option<crate::datadogV1::model::GeomapWidgetDefinitionStyle> = None;
                let mut time: Option<crate::datadogV1::model::WidgetTime> = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::GeomapWidgetDefinitionType> = None;
                let mut view: Option<crate::datadogV1::model::GeomapWidgetDefinitionView> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
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
                        "requests" => {
                            requests = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "style" => {
                            style = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV1::model::GeomapWidgetDefinitionType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "view" => {
                            view = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let requests = requests.ok_or_else(|| M::Error::missing_field("requests"))?;
                let style = style.ok_or_else(|| M::Error::missing_field("style"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;
                let view = view.ok_or_else(|| M::Error::missing_field("view"))?;

                let content = GeomapWidgetDefinition {
                    custom_links,
                    requests,
                    style,
                    time,
                    title,
                    title_align,
                    title_size,
                    type_,
                    view,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GeomapWidgetDefinitionVisitor)
    }
}
