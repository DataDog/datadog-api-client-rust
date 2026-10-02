// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Response containing the saved analysis plan and result refresh information.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ExperimentsAnalysisPlanV2MutationResponse {
    /// Analysis plan resource with its identifier and settings.
    #[serde(rename = "data")]
    pub data: crate::datadogV2::model::ExperimentsAnalysisPlanV2DTOData,
    /// Refresh requirements and warnings returned by an experiment update.
    #[serde(rename = "meta")]
    pub meta: Option<crate::datadogV2::model::ExperimentsPatchExperimentV2MetaDTO>,
    #[serde(flatten)]
    pub additional_properties: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl ExperimentsAnalysisPlanV2MutationResponse {
    pub fn new(
        data: crate::datadogV2::model::ExperimentsAnalysisPlanV2DTOData,
    ) -> ExperimentsAnalysisPlanV2MutationResponse {
        ExperimentsAnalysisPlanV2MutationResponse {
            data,
            meta: None,
            additional_properties: std::collections::BTreeMap::new(),
            _unparsed: false,
        }
    }

    pub fn meta(
        mut self,
        value: crate::datadogV2::model::ExperimentsPatchExperimentV2MetaDTO,
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

impl<'de> Deserialize<'de> for ExperimentsAnalysisPlanV2MutationResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExperimentsAnalysisPlanV2MutationResponseVisitor;
        impl<'a> Visitor<'a> for ExperimentsAnalysisPlanV2MutationResponseVisitor {
            type Value = ExperimentsAnalysisPlanV2MutationResponse;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data: Option<crate::datadogV2::model::ExperimentsAnalysisPlanV2DTOData> =
                    None;
                let mut meta: Option<crate::datadogV2::model::ExperimentsPatchExperimentV2MetaDTO> =
                    None;
                let mut additional_properties: std::collections::BTreeMap<
                    String,
                    serde_json::Value,
                > = std::collections::BTreeMap::new();
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data" => {
                            data = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "meta" => {
                            if v.is_null() {
                                continue;
                            }
                            meta = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            if let Ok(value) = serde_json::from_value(v.clone()) {
                                additional_properties.insert(k, value);
                            }
                        }
                    }
                }
                let data = data.ok_or_else(|| M::Error::missing_field("data"))?;

                let content = ExperimentsAnalysisPlanV2MutationResponse {
                    data,
                    meta,
                    additional_properties,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(ExperimentsAnalysisPlanV2MutationResponseVisitor)
    }
}
