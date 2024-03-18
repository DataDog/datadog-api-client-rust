// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Request to create a role by cloning an existing role.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct RoleCloneRequest {
    /// Data for the clone role request.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::RoleClone,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl RoleCloneRequest {
    pub fn new(data: crate::datadogV2::model::RoleClone) -> RoleCloneRequest {
        RoleCloneRequest {
            data,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for RoleCloneRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct RoleCloneRequestVisitor;
        impl<'a> Visitor<'a> for RoleCloneRequestVisitor {
            type Value = RoleCloneRequest;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV2::model::RoleClone> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {}
                    }
                }
                let data = data.ok_or_else(|| M::Error::missing_field("data"))?;

                let content = RoleCloneRequest { data, _unparsed };

                Ok(content)
            }
        }

        deserializer.deserialize_any(RoleCloneRequestVisitor)
    }
}
