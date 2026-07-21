// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Attributes of a DDSQL tabular query response. `query_id` is set when
/// `state` is `running`; `columns` is set when `state` is `completed`.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct DdsqlTabularQueryResponseAttributes {
    /// Column-major result set. Each element carries one column's name, type, and values,
    /// with one value per row of the result. Set when `state` is `completed`.
    #[serde(rename = "columns")]
    pub columns: Option<Vec<crate::datadogV2::model::DdsqlTabularQueryColumn>>,
    /// Opaque token to pass to the fetch endpoint to poll for results.
    /// Set when `state` is `running` and absent when `state` is `completed`.
    #[serde(rename = "query_id")]
    pub query_id: Option<String>,
    /// Lifecycle state of a DDSQL tabular query response.
    /// `running` means the query is still executing and the client should poll
    /// the fetch endpoint with the returned `query_id`. `completed` means the
    /// result set is inlined in `columns` and no further polling is required.
    #[serde(rename = "state")]
    pub state: crate::datadogV2::model::DdsqlTabularQueryState,
    /// Non-fatal messages emitted by the query engine while serving this response.
    #[serde(rename = "warnings")]
    pub warnings: Option<Vec<String>>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl DdsqlTabularQueryResponseAttributes {
    pub fn new(
        state: crate::datadogV2::model::DdsqlTabularQueryState,
    ) -> DdsqlTabularQueryResponseAttributes {
        DdsqlTabularQueryResponseAttributes {
            columns: None,
            query_id: None,
            state,
            warnings: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn columns(mut self, value: Vec<crate::datadogV2::model::DdsqlTabularQueryColumn>) -> Self {
        self.columns = Some(value);
        self
    }

    pub fn query_id(mut self, value: String) -> Self {
        self.query_id = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
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

impl<'de> Deserialize<'de> for DdsqlTabularQueryResponseAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DdsqlTabularQueryResponseAttributesVisitor;
        impl<'a> Visitor<'a> for DdsqlTabularQueryResponseAttributesVisitor {
            type Value = DdsqlTabularQueryResponseAttributes;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut columns: Option<Vec<crate::datadogV2::model::DdsqlTabularQueryColumn>> =
                    None;
                let mut query_id: Option<String> = None;
                let mut state: Option<crate::datadogV2::model::DdsqlTabularQueryState> = None;
                let mut warnings: Option<Vec<String>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "columns" => {
                            if v.is_null() {
                                continue;
                            }
                            columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "query_id" => {
                            if v.is_null() {
                                continue;
                            }
                            query_id = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "state" => {
                            state = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _state) = state {
                                match _state {
                                    crate::datadogV2::model::DdsqlTabularQueryState::UnparsedObject(_state) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        }
                        "warnings" => {
                            if v.is_null() {
                                continue;
                            }
                            warnings = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let state = state.ok_or_else(|| M::Error::missing_field("state"))?;

                let content = DdsqlTabularQueryResponseAttributes {
                    columns,
                    query_id,
                    state,
                    warnings,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(DdsqlTabularQueryResponseAttributesVisitor)
    }
}
