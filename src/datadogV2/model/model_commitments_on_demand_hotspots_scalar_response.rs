// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response containing scalar on-demand hot-spots data for cloud commitment programs.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CommitmentsOnDemandHotspotsScalarResponse {
    /// Array of scalar columns in the response.
    #[serde(rename = "columns")]
    pub columns: Vec<crate::datadogV2::model::CommitmentsScalarColumn>,
    /// Metadata for the on-demand hot-spots scalar response.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::CommitmentsOnDemandHotspotsScalarMeta>,
    /// Array of scalar columns in the response.
    #[serde(rename = "total")]
    pub total: Vec<crate::datadogV2::model::CommitmentsScalarColumn>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl CommitmentsOnDemandHotspotsScalarResponse {
    pub fn new(
        columns: Vec<crate::datadogV2::model::CommitmentsScalarColumn>,
        total: Vec<crate::datadogV2::model::CommitmentsScalarColumn>,
    ) -> CommitmentsOnDemandHotspotsScalarResponse {
        CommitmentsOnDemandHotspotsScalarResponse {
            columns,
            meta: None,
            total,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn meta(
        mut self,
        value: crate::datadogV2::model::CommitmentsOnDemandHotspotsScalarMeta,
    ) -> Self {
        self.meta = Some(value);
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

impl<'de> Deserialize<'de> for CommitmentsOnDemandHotspotsScalarResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CommitmentsOnDemandHotspotsScalarResponseVisitor;
        impl<'a> Visitor<'a> for CommitmentsOnDemandHotspotsScalarResponseVisitor {
            type Value = CommitmentsOnDemandHotspotsScalarResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut columns: Option<Vec<crate::datadogV2::model::CommitmentsScalarColumn>> =
                    None;
                let mut meta: Option<
                    crate::datadogV2::model::CommitmentsOnDemandHotspotsScalarMeta,
                > = None;
                let mut total: Option<Vec<crate::datadogV2::model::CommitmentsScalarColumn>> = None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "columns" => {
                            columns = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "total" => {
                            total = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let columns = columns.ok_or_else(|| M::Error::missing_field("columns"))?;
                let total = total.ok_or_else(|| M::Error::missing_field("total"))?;

                let content = CommitmentsOnDemandHotspotsScalarResponse {
                    columns,
                    meta,
                    total,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(CommitmentsOnDemandHotspotsScalarResponseVisitor)
    }
}
