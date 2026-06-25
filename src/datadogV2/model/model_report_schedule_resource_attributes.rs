// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of an included report target resource.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ReportScheduleResourceAttributes {
    /// The type of dashboard resource the report schedule targets.
    #[serde(rename = "resource_type")]
    pub resource_type: crate::datadogV2::model::ReportScheduleResourceType,
    /// Template variable metadata from the dashboard resource, when available.
    #[serde(
        rename = "template_variables",
        default,
        with = "::serde_with::rust::double_option"
    )]
    pub template_variables:
        Option<Option<Vec<crate::datadogV2::model::ReportScheduleIndexTemplateVariable>>>,
    /// The title of the dashboard or integration dashboard resource, when available.
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option")]
    pub title: Option<Option<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ReportScheduleResourceAttributes {
    pub fn new(
        resource_type: crate::datadogV2::model::ReportScheduleResourceType,
    ) -> ReportScheduleResourceAttributes {
        ReportScheduleResourceAttributes {
            resource_type,
            template_variables: None,
            title: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn template_variables(
        mut self,
        value: Option<Vec<crate::datadogV2::model::ReportScheduleIndexTemplateVariable>>,
    ) -> Self {
        self.template_variables = Some(value);
        self
    }

    pub fn title(mut self, value: Option<String>) -> Self {
        self.title = Some(value);
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

impl<'de> Deserialize<'de> for ReportScheduleResourceAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReportScheduleResourceAttributesVisitor;
        impl<'a> Visitor<'a> for ReportScheduleResourceAttributesVisitor {
            type Value = ReportScheduleResourceAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut resource_type: Option<crate::datadogV2::model::ReportScheduleResourceType> =
                    None;
                let mut template_variables: Option<
                    Option<Vec<crate::datadogV2::model::ReportScheduleIndexTemplateVariable>>,
                > = None;
                let mut title: Option<Option<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
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
                let resource_type =
                    resource_type.ok_or_else(|| M::Error::missing_field("resource_type"))?;

                let content = ReportScheduleResourceAttributes {
                    resource_type,
                    template_variables,
                    title,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ReportScheduleResourceAttributesVisitor)
    }
}
