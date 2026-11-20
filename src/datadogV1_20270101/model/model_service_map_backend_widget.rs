// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ServiceMapBackendWidget {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "definition")]
    pub definition: crate::datadogV1_20270101::model::ServiceMapDefinition,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "id")]
    pub id: Option<f64>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "layout")]
    pub layout: Option<crate::datadogV1_20270101::model::WidgetLayout>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ServiceMapBackendWidget {
    pub fn new(definition: crate::datadogV1_20270101::model::ServiceMapDefinition) -> ServiceMapBackendWidget {
        ServiceMapBackendWidget {
            definition,
            id: None,
            layout: None,
            _unparsed: false,
        }
    }
    
    pub fn id(mut self, value: f64) -> Self {
        self.id = Some(value);
        self
    }
    
    pub fn layout(mut self, value: crate::datadogV1_20270101::model::WidgetLayout) -> Self {
        self.layout = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for ServiceMapBackendWidget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ServiceMapBackendWidgetVisitor;
        impl<'a> Visitor<'a> for ServiceMapBackendWidgetVisitor {
            type Value = ServiceMapBackendWidget;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut definition: Option<crate::datadogV1_20270101::model::ServiceMapDefinition> = None;
                let mut id: Option<f64> = None;
                let mut layout: Option<crate::datadogV1_20270101::model::WidgetLayout> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "definition" => {
                            definition = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "id" => {
                            if v.is_null() || v.as_str() == Some("") {
                                continue;
                            }
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "layout" => {
                            if v.is_null() {
                                continue;
                            }
                            layout = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _layout) = layout {
                                match _layout {
                                    crate::datadogV1_20270101::model::WidgetLayout::UnparsedObject(_layout) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let definition = definition.ok_or_else(|| M::Error::missing_field("definition"))?;

                let content = ServiceMapBackendWidget {
                    definition,
                    id,
                    layout,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ServiceMapBackendWidgetVisitor)
    }
}
