// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The full app definition response object.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GetAppResponse {
    /// The data object containing the app definition.
    #[serde(rename = "data")]
    pub data: Option<crate::datadogV2::model::GetAppResponseData>,
    /// Data on the version of the app that was published.
    #[serde(rename = "included")]
    pub included: Option<Vec<crate::datadogV2::model::Deployment>>,
    /// Metadata of an app.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::AppMeta>,
    /// The app's publication relationship and custom connections.
    #[serde(rename = "relationship")]
    pub relationship: Option<crate::datadogV2::model::AppRelationship>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl GetAppResponse {
    pub fn new() -> GetAppResponse {
        GetAppResponse {
            data: None,
            included: None,
            meta: None,
            relationship: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn data(mut self, value: crate::datadogV2::model::GetAppResponseData) -> Self {
        self.data = Some(value);
        self
    }

    pub fn included(mut self, value: Vec<crate::datadogV2::model::Deployment>) -> Self {
        self.included = Some(value);
        self
    }

    pub fn meta(mut self, value: crate::datadogV2::model::AppMeta) -> Self {
        self.meta = Some(value);
        self
    }

    pub fn relationship(mut self, value: crate::datadogV2::model::AppRelationship) -> Self {
        self.relationship = Some(value);
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

impl Default for GetAppResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for GetAppResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct GetAppResponseVisitor;
        impl<'a> Visitor<'a> for GetAppResponseVisitor {
            type Value = GetAppResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV2::model::GetAppResponseData> = None;
                let mut included: Option<Vec<crate::datadogV2::model::Deployment>> = None;
                let mut meta: Option<crate::datadogV2::model::AppMeta> = None;
                let mut relationship: Option<crate::datadogV2::model::AppRelationship> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            if v.is_null() {
                                continue;
                            }
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "included" => {
                            if v.is_null() {
                                continue;
                            }
                            included = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "relationship" => {
                            if v.is_null() {
                                continue;
                            }
                            relationship =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }

                let content = GetAppResponse {
                    data,
                    included,
                    meta,
                    relationship,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(GetAppResponseVisitor)
    }
}
