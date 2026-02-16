// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Metadata for the CycloneDX BOM.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CycloneDXMetadata {
    /// The asset component represents the system or host being scanned.
    #[serde(rename = "component")]
    pub component: crate::datadogV2::model::CycloneDXAssetComponent,
    /// Tools used to generate the BOM.
    #[serde(rename = "tools")]
    pub tools: crate::datadogV2::model::CycloneDXTools,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CycloneDXMetadata {
    pub fn new(
        component: crate::datadogV2::model::CycloneDXAssetComponent,
        tools: crate::datadogV2::model::CycloneDXTools,
    ) -> CycloneDXMetadata {
        CycloneDXMetadata {
            component,
            tools,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn additional_properties(
        mut self,
        value: std::collections::BTreeMap<String, serde_json::Value>,
    ) -> Self {
        self.additional_properties = value;
        self
    }
}

impl<'de> Deserialize<'de> for CycloneDXMetadata {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CycloneDXMetadataVisitor;
        impl<'a> Visitor<'a> for CycloneDXMetadataVisitor {
            type Value = CycloneDXMetadata;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut component: Option<crate::datadogV2::model::CycloneDXAssetComponent> = None;
                let mut tools: Option<crate::datadogV2::model::CycloneDXTools> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "component" => {
                            component = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "tools" => {
                            tools = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let component = component.ok_or_else(|| M::Error::missing_field("component"))?;
                let tools = tools.ok_or_else(|| M::Error::missing_field("tools"))?;

                let content = CycloneDXMetadata {
                    component,
                    tools,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CycloneDXMetadataVisitor)
    }
}
