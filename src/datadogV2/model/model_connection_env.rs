// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// A list of connections or connection groups used in the workflow.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ConnectionEnv {
    /// The `ConnectionEnv` `connectionGroups`.
    #[serde(rename = "connectionGroups")]
    pub connection_groups: Option<Vec<crate::datadogV2::model::ConnectionGroup>>,
    /// The `ConnectionEnv` `connections`.
    #[serde(rename = "connections")]
    pub connections: Option<Vec<crate::datadogV2::model::Connection>>,
    /// The definition of `ConnectionEnvEnv` object.
    #[serde(rename = "env")]
    pub env: crate::datadogV2::model::ConnectionEnvEnv,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ConnectionEnv {
    pub fn new(env: crate::datadogV2::model::ConnectionEnvEnv) -> ConnectionEnv {
        ConnectionEnv {
            connection_groups: None,
            connections: None,
            env,
            _unparsed: false,
        }
    }

    pub fn connection_groups(
        mut self,
        value: Vec<crate::datadogV2::model::ConnectionGroup>,
    ) -> Self {
        self.connection_groups = Some(value);
        self
    }

    pub fn connections(mut self, value: Vec<crate::datadogV2::model::Connection>) -> Self {
        self.connections = Some(value);
        self
    }
}

impl<'de> Deserialize<'de> for ConnectionEnv {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ConnectionEnvVisitor;
        impl<'a> Visitor<'a> for ConnectionEnvVisitor {
            type Value = ConnectionEnv;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut connection_groups: Option<Vec<crate::datadogV2::model::ConnectionGroup>> =
                    None;
                let mut connections: Option<Vec<crate::datadogV2::model::Connection>> = None;
                let mut env: Option<crate::datadogV2::model::ConnectionEnvEnv> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "connectionGroups" => {
                            if v.is_null() {
                                continue;
                            }
                            connection_groups =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "connections" => {
                            if v.is_null() {
                                continue;
                            }
                            connections =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "env" => {
                            env = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _env) = env {
                                match _env {
                                    crate::datadogV2::model::ConnectionEnvEnv::UnparsedObject(
                                        _env,
                                    ) => {
                                        _unparsed = true;
                                    }
                                    _ => {}
                                }
                            }
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let env = env.ok_or_else(|| M::Error::missing_field("env"))?;

                let content = ConnectionEnv {
                    connection_groups,
                    connections,
                    env,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ConnectionEnvVisitor)
    }
}
