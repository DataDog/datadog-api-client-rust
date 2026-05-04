// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// This widget displays a Cloudcraft topology of cloud resources for the selected provider.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CloudcraftWidgetDefinition {
    /// List of custom links.
    #[serde(rename = "custom_links")]
    pub custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>>,
    /// The description of the widget.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// List of tags or attributes used to group the cloud resources in the widget.
    #[serde(rename = "group_by")]
    pub group_by: Vec<String>,
    /// Search query that visually highlights matching resources in the diagram.
    #[serde(rename = "highlighted")]
    pub highlighted: String,
    /// Overlay applied on top of the Cloudcraft topology.
    #[serde(rename = "overlay")]
    pub overlay: crate::datadogV1::model::CloudcraftWidgetDefinitionOverlay,
    /// Filter applied to the selected overlay.
    #[serde(rename = "overlay_filter")]
    pub overlay_filter: String,
    /// Projection used to render the Cloudcraft topology.
    #[serde(rename = "projection")]
    pub projection: crate::datadogV1::model::CloudcraftWidgetDefinitionProjection,
    /// Cloud provider for the Cloudcraft widget.
    #[serde(rename = "provider")]
    pub provider: crate::datadogV1::model::CloudcraftWidgetDefinitionProvider,
    /// Query string used to filter the cloud resources displayed in the widget.
    #[serde(rename = "query_string")]
    pub query_string: String,
    /// Whether to show empty outline groups in the diagram.
    #[serde(rename = "show_empty_groups")]
    pub show_empty_groups: bool,
    /// Title of your widget.
    #[serde(rename = "title")]
    pub title: Option<String>,
    /// How to align the text on the widget.
    #[serde(rename = "title_align")]
    pub title_align: Option<crate::datadogV1::model::WidgetTextAlign>,
    /// Size of the title.
    #[serde(rename = "title_size")]
    pub title_size: Option<String>,
    /// Type of the Cloudcraft widget.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::CloudcraftWidgetDefinitionType,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CloudcraftWidgetDefinition {
    pub fn new(
        group_by: Vec<String>,
        highlighted: String,
        overlay: crate::datadogV1::model::CloudcraftWidgetDefinitionOverlay,
        overlay_filter: String,
        projection: crate::datadogV1::model::CloudcraftWidgetDefinitionProjection,
        provider: crate::datadogV1::model::CloudcraftWidgetDefinitionProvider,
        query_string: String,
        show_empty_groups: bool,
        type_: crate::datadogV1::model::CloudcraftWidgetDefinitionType,
    ) -> CloudcraftWidgetDefinition {
        CloudcraftWidgetDefinition {
            custom_links: None,
            description: None,
            group_by,
            highlighted,
            overlay,
            overlay_filter,
            projection,
            provider,
            query_string,
            show_empty_groups,
            title: None,
            title_align: None,
            title_size: None,
            type_,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn custom_links(mut self, value: Vec<crate::datadogV1::model::WidgetCustomLink>) -> Self {
        self.custom_links = Some(value);
        self
    }

    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
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

impl<'de> Deserialize<'de> for CloudcraftWidgetDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CloudcraftWidgetDefinitionVisitor;
        impl<'a> Visitor<'a> for CloudcraftWidgetDefinitionVisitor {
            type Value = CloudcraftWidgetDefinition;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut custom_links: Option<Vec<crate::datadogV1::model::WidgetCustomLink>> = None;
                let mut description: Option<String> = None;
                let mut group_by: Option<Vec<String>> = None;
                let mut highlighted: Option<String> = None;
                let mut overlay: Option<
                    crate::datadogV1::model::CloudcraftWidgetDefinitionOverlay,
                > = None;
                let mut overlay_filter: Option<String> = None;
                let mut projection: Option<
                    crate::datadogV1::model::CloudcraftWidgetDefinitionProjection,
                > = None;
                let mut provider: Option<
                    crate::datadogV1::model::CloudcraftWidgetDefinitionProvider,
                > = None;
                let mut query_string: Option<String> = None;
                let mut show_empty_groups: Option<bool> = None;
                let mut title: Option<String> = None;
                let mut title_align: Option<crate::datadogV1::model::WidgetTextAlign> = None;
                let mut title_size: Option<String> = None;
                let mut type_: Option<crate::datadogV1::model::CloudcraftWidgetDefinitionType> =
                    None;
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
                        "description" => {
                            if v.is_null() {
                                continue;
                            }
                            description =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by" => {
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "highlighted" => {
                            highlighted =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "overlay" => {
                            overlay = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _overlay) = overlay {
                                match _overlay {
                                    crate::datadogV1::model::CloudcraftWidgetDefinitionOverlay::UnparsedObject(_overlay) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "overlay_filter" => {
                            overlay_filter =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "projection" => {
                            projection = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _projection) = projection {
                                match _projection {
                                    crate::datadogV1::model::CloudcraftWidgetDefinitionProjection::UnparsedObject(_projection) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "provider" => {
                            provider = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _provider) = provider {
                                match _provider {
                                    crate::datadogV1::model::CloudcraftWidgetDefinitionProvider::UnparsedObject(_provider) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "query_string" => {
                            query_string =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "show_empty_groups" => {
                            show_empty_groups =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
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
                                    crate::datadogV1::model::CloudcraftWidgetDefinitionType::UnparsedObject(_type_) => {
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
                let group_by = group_by.ok_or_else(|| M::Error::missing_field("group_by"))?;
                let highlighted =
                    highlighted.ok_or_else(|| M::Error::missing_field("highlighted"))?;
                let overlay = overlay.ok_or_else(|| M::Error::missing_field("overlay"))?;
                let overlay_filter =
                    overlay_filter.ok_or_else(|| M::Error::missing_field("overlay_filter"))?;
                let projection = projection.ok_or_else(|| M::Error::missing_field("projection"))?;
                let provider = provider.ok_or_else(|| M::Error::missing_field("provider"))?;
                let query_string =
                    query_string.ok_or_else(|| M::Error::missing_field("query_string"))?;
                let show_empty_groups = show_empty_groups
                    .ok_or_else(|| M::Error::missing_field("show_empty_groups"))?;
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = CloudcraftWidgetDefinition {
                    custom_links,
                    description,
                    group_by,
                    highlighted,
                    overlay,
                    overlay_filter,
                    projection,
                    provider,
                    query_string,
                    show_empty_groups,
                    title,
                    title_align,
                    title_size,
                    type_,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CloudcraftWidgetDefinitionVisitor)
    }
}
