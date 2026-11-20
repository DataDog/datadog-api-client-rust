// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use std::fmt::{self, Formatter};
use serde::{Deserializer, Deserialize, Serialize};
use serde::de::{Error, MapAccess, Visitor};
use serde_with::skip_serializing_none;

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BarChartBackendDefinitionStyle20270101DisplayOneOf {
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "legend")]
    pub legend: Option<crate::datadogV1::model::BarChartBackendDefinitionStyleDisplayAnyOfAnyOfItemLegend>,
    /// Test-only generated schema for the dashboard versioning proof of concept.
    #[serde(rename = "type")]
    pub type_: crate::datadogV1::model::BarChartBackendDefinitionStyleDisplayAnyOfAnyOfItemType,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool
}

impl BarChartBackendDefinitionStyle20270101DisplayOneOf {
    pub fn new(type_: crate::datadogV1::model::BarChartBackendDefinitionStyleDisplayAnyOfAnyOfItemType) -> BarChartBackendDefinitionStyle20270101DisplayOneOf {
        BarChartBackendDefinitionStyle20270101DisplayOneOf {
            legend: None,
            type_,
            _unparsed: false,
        }
    }
    
    pub fn legend(mut self, value: crate::datadogV1::model::BarChartBackendDefinitionStyleDisplayAnyOfAnyOfItemLegend) -> Self {
        self.legend = Some(value);
        self
    }
    
}



impl<'de> Deserialize<'de> for BarChartBackendDefinitionStyle20270101DisplayOneOf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BarChartBackendDefinitionStyle20270101DisplayOneOfVisitor;
        impl<'a> Visitor<'a> for BarChartBackendDefinitionStyle20270101DisplayOneOfVisitor {
            type Value = BarChartBackendDefinitionStyle20270101DisplayOneOf;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut legend: Option<crate::datadogV1::model::BarChartBackendDefinitionStyleDisplayAnyOfAnyOfItemLegend> = None;
                let mut type_: Option<crate::datadogV1::model::BarChartBackendDefinitionStyleDisplayAnyOfAnyOfItemType> = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "legend" => {
                            if v.is_null() {
                                continue;
                            }
                            legend = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _legend) = legend {
                                match _legend {
                                    crate::datadogV1::model::BarChartBackendDefinitionStyleDisplayAnyOfAnyOfItemLegend::UnparsedObject(_legend) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        "type" => {
                            type_ = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                            if let Some(ref _type_) = type_ {
                                match _type_ {
                                    crate::datadogV1::model::BarChartBackendDefinitionStyleDisplayAnyOfAnyOfItemType::UnparsedObject(_type_) => {
                                        _unparsed = true;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        &_ => {
                            return Err(serde::de::Error::custom("Additional properties not allowed"));
                        },
                    }
                }
                let type_ = type_.ok_or_else(|| M::Error::missing_field("type_"))?;

                let content = BarChartBackendDefinitionStyle20270101DisplayOneOf {
                    legend,
                    type_,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(BarChartBackendDefinitionStyle20270101DisplayOneOfVisitor)
    }
}
