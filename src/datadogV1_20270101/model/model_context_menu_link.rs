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
pub struct ContextMenuLink {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "is_hidden")]
    pub is_hidden: Option<bool>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "link")]
    pub link: Option<String>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "override_label")]
    pub override_label: crate::datadogV1_20270101::model::ContextMenuLinkLabel,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl ContextMenuLink {
    pub fn new(override_label: crate::datadogV1_20270101::model::ContextMenuLinkLabel) -> ContextMenuLink {
        ContextMenuLink {
            is_hidden: None,
            link: None,
            override_label,
            _unparsed: false,
        }
    }
    
    pub fn is_hidden(mut self, value: bool) -> Self {
        self.is_hidden = Some(value);
        self
    }
    
    pub fn link(mut self, value: String) -> Self {
        self.link = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for ContextMenuLink {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContextMenuLinkVisitor;
        impl<'a> Visitor<'a> for ContextMenuLinkVisitor {
            type Value = ContextMenuLink;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut is_hidden: Option<bool> = None;
                let mut link: Option<String> = None;
                let mut override_label: Option<crate::datadogV1_20270101::model::ContextMenuLinkLabel> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "is_hidden" => {
                            if v.is_null() {
                                continue;
                            }
                            is_hidden = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "link" => {
                            if v.is_null() {
                                continue;
                            }
                            link = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        },
                        "override_label" => {
                            override_label = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _override_label) = override_label {
                                match _override_label {
                                    crate::datadogV1_20270101::model::ContextMenuLinkLabel::UnparsedObject(_override_label) => {
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
                let override_label = override_label.ok_or_else(|| M::Error::missing_field("override_label"))?;

                let content = ContextMenuLink {
                    is_hidden,
                    link,
                    override_label,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ContextMenuLinkVisitor)
    }
}
