// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The most recently started refresh. Null until one starts.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsBanditRefreshV2 {
    /// When the refresh was requested.
    #[serde(rename = "created_at")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// When the refresh finished. Null while it runs.
    #[serialize_always]
    #[serde(rename = "ended_at")]
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The refresh ID.
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    /// When the refresh started running. Null while it is pending.
    #[serialize_always]
    #[serde(rename = "started_at")]
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    /// The status of the refresh.
    #[serde(rename = "status")]
    pub status: crate::datadogV2::model::ExperimentsBanditRefreshStatus,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsBanditRefreshV2 {
    pub fn new(
        created_at: chrono::DateTime<chrono::Utc>,
        ended_at: Option<chrono::DateTime<chrono::Utc>>,
        id: uuid::Uuid,
        started_at: Option<chrono::DateTime<chrono::Utc>>,
        status: crate::datadogV2::model::ExperimentsBanditRefreshStatus,
    ) -> ExperimentsBanditRefreshV2 {
        ExperimentsBanditRefreshV2 {
            created_at,
            ended_at,
            id,
            started_at,
            status,
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

impl<'de> Deserialize<'de> for ExperimentsBanditRefreshV2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsBanditRefreshV2Visitor;
        impl<'a> Visitor<'a> for ExperimentsBanditRefreshV2Visitor {
            type Value = ExperimentsBanditRefreshV2;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut created_at: Option<chrono::DateTime<chrono::Utc>> = None;
                let mut ended_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut id: Option<uuid::Uuid> = None;
                let mut started_at: Option<Option<chrono::DateTime<chrono::Utc>>> = None;
                let mut status: Option<crate::datadogV2::model::ExperimentsBanditRefreshStatus> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "created_at" => {
                            created_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "ended_at" => {
                            ended_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "id" => {
                            id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "started_at" => {
                            started_at = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "status" => {
                            status = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _status) = status {
                                match _status {
                                    crate::datadogV2::model::ExperimentsBanditRefreshStatus::UnparsedObject(_status) => {
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
                let created_at = created_at.ok_or_else(|| M::Error::missing_field("created_at"))?;
                let ended_at = ended_at.ok_or_else(|| M::Error::missing_field("ended_at"))?;
                let id = id.ok_or_else(|| M::Error::missing_field("id"))?;
                let started_at = started_at.ok_or_else(|| M::Error::missing_field("started_at"))?;
                let status = status.ok_or_else(|| M::Error::missing_field("status"))?;

                let content = ExperimentsBanditRefreshV2 {
                    created_at,
                    ended_at,
                    id,
                    started_at,
                    status,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsBanditRefreshV2Visitor)
    }
}
