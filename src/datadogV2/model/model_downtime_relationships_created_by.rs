// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// The user who created the downtime.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DowntimeRelationshipsCreatedBy {
    /// Data for the user who created the downtime.
    #[serde(rename = "data", default, with = "::serde_with::rust::double_option")]
    pub data: Option<Option<crate::datadogV2::model::DowntimeRelationshipsCreatedByData>>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DowntimeRelationshipsCreatedBy {
    pub fn new() -> DowntimeRelationshipsCreatedBy {
        DowntimeRelationshipsCreatedBy {
            data: None,
            _unparsed: false,
        }
    }

    pub fn data(
        mut self,
        value: Option<crate::datadogV2::model::DowntimeRelationshipsCreatedByData>,
    ) -> Self {
        self.data = Some(value);
        self
    }
}

impl Default for DowntimeRelationshipsCreatedBy {
    fn default() -> Self {
        Self::new()
    }
}

impl<'de> Deserialize<'de> for DowntimeRelationshipsCreatedBy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DowntimeRelationshipsCreatedByVisitor;
        impl<'a> Visitor<'a> for DowntimeRelationshipsCreatedByVisitor {
            type Value = DowntimeRelationshipsCreatedBy;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<
                    Option<crate::datadogV2::model::DowntimeRelationshipsCreatedByData>,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }

                let content = DowntimeRelationshipsCreatedBy { data, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DowntimeRelationshipsCreatedByVisitor)
    }
}
