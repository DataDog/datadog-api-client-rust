// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// An Elastic Cloud dataflow toggle. The set of dataflow ids is fixed by the interface schema.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ElasticCloudDataflow {
    /// Whether the dataflow is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Identifier of an Elastic Cloud dataflow.
    #[serde(rename = "id")]
    pub id: crate::datadogV2::model::ElasticCloudDataflowId,
    /// Read-only, server-computed collection status of a dataflow.
    #[serde(rename = "status")]
    pub status: Option<crate::datadogV2::model::IntegrationAccountDataflowStatus>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ElasticCloudDataflow {
    pub fn new(id: crate::datadogV2::model::ElasticCloudDataflowId) -> ElasticCloudDataflow {
        ElasticCloudDataflow {
            enabled: None,
            id,
            status: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn status(
        mut self,
        value: crate::datadogV2::model::IntegrationAccountDataflowStatus,
    ) -> Self {
        self.status = Some(value);
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

impl<'de> Deserialize<'de> for ElasticCloudDataflow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ElasticCloudDataflowVisitor;
        impl<'a> Visitor<'a> for ElasticCloudDataflowVisitor {
            type Value = ElasticCloudDataflow;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut enabled: Option<bool> = None;
                let mut id: Option<crate::datadogV2::model::ElasticCloudDataflowId> = None;
                let mut status: Option<crate::datadogV2::model::IntegrationAccountDataflowStatus> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "enabled" => {
                            if v.is_null() {
                                continue;
                            }
                            enabled = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _id) = id {
                                match _id {
                                    crate::datadogV2::model::ElasticCloudDataflowId::UnparsedObject(_id) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "status" => {
                            if v.is_null() {
                                continue;
                            }
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;

                let content = ElasticCloudDataflow {
                    enabled,
                    id,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ElasticCloudDataflowVisitor)
    }
}
