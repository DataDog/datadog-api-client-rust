// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// List of Container Images.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ContainerImagesResponse {
    /// Array of Container Image objects.
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::datadogV2::model::ContainerImageItem>>,
    /// Pagination links.
    #[serde(rename = "links")]
    pub links: Option<crate::datadogV2::model::ContainerImagesResponseLinks>,
    /// Response metadata object.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::ContainerImageMeta>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ContainerImagesResponse {
    pub fn new() -> ContainerImagesResponse {
        ContainerImagesResponse {
            data: None,
            links: None,
            meta: None,
            _unparsed: false,
        }
    }

    pub fn data(&mut self, value: Vec<crate::datadogV2::model::ContainerImageItem>) -> &mut Self {
        self.data = Some(value);
        self
    }

    pub fn links(
        &mut self,
        value: crate::datadogV2::model::ContainerImagesResponseLinks,
    ) -> &mut Self {
        self.links = Some(value);
        self
    }

    pub fn meta(&mut self, value: crate::datadogV2::model::ContainerImageMeta) -> &mut Self {
        self.meta = Some(value);
        self
    }
}

impl Default for ContainerImagesResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ContainerImagesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContainerImagesResponseVisitor;
        impl<'a> Visitor<'a> for ContainerImagesResponseVisitor {
            type Value = ContainerImagesResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<Vec<crate::datadogV2::model::ContainerImageItem>> = None;
                let mut links: Option<crate::datadogV2::model::ContainerImagesResponseLinks> = None;
                let mut meta: Option<crate::datadogV2::model::ContainerImageMeta> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            if v.is_null() {
                                continue;
                            }
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "links" => {
                            if v.is_null() {
                                continue;
                            }
                            links = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = ContainerImagesResponse {
                    data,
                    links,
                    meta,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ContainerImagesResponseVisitor)
    }
}
