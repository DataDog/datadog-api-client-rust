// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Relationships to Container Images inside a Container Image Group.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ContainerImageGroupImagesRelationshipsLink {
    /// Links data.
    #[serde(rename = "data")]
    pub data: Option<Vec<String>>,
    /// Links attributes.
    #[serde(rename = "links")]
    pub links: Option<crate::datadogV2::model::ContainerImageGroupRelationshipsLinks>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ContainerImageGroupImagesRelationshipsLink {
    pub fn new() -> ContainerImageGroupImagesRelationshipsLink {
        ContainerImageGroupImagesRelationshipsLink {
            data: None,
            links: None,
            _unparsed: false,
        }
    }

    pub fn data(mut self, value: Vec<String>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn links(
        mut self,
        value: crate::datadogV2::model::ContainerImageGroupRelationshipsLinks,
    ) -> Self {
        self.links = Some(value);
        self
    }
}

impl Default for ContainerImageGroupImagesRelationshipsLink {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for ContainerImageGroupImagesRelationshipsLink {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContainerImageGroupImagesRelationshipsLinkVisitor;
        impl<'a> Visitor<'a> for ContainerImageGroupImagesRelationshipsLinkVisitor {
            type Value = ContainerImageGroupImagesRelationshipsLink;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<Vec<String>> = None;
                let mut links: Option<
                    crate::datadogV2::model::ContainerImageGroupRelationshipsLinks,
                > = None;
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
                        &_ => {}
                    }
                }

                let content = ContainerImageGroupImagesRelationshipsLink {
                    data,
                    links,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ContainerImageGroupImagesRelationshipsLinkVisitor)
    }
}
