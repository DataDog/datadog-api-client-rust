// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::fmt::{self, Formatter};

/// Available values query for logs, RUM, or spans data sources.
#[non_exhaustive]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct NotebookTemplateVariableAvailableValuesQueryLogRumSpans {
    /// The data source for the query. Must be one of `logs`, `rum`, or `spans`.
    #[serde(rename = "data_source")]
    pub data_source: String,
    /// Group-by fields for the query.
    #[serde(rename = "group_by")]
    pub group_by: Vec<crate::datadogV1::model::NotebookTemplateVariableAvailableValuesQueryGroupBy>,
    /// Search parameters for an available values query.
    #[serde(rename = "search")]
    pub search: crate::datadogV1::model::NotebookTemplateVariableAvailableValuesQuerySearch,
    #[serde(skip)]
    #[serde(default)]
    pub(crate) _unparsed: bool,
}

impl NotebookTemplateVariableAvailableValuesQueryLogRumSpans {
    pub fn new(
        data_source: String,
        group_by: Vec<crate::datadogV1::model::NotebookTemplateVariableAvailableValuesQueryGroupBy>,
        search: crate::datadogV1::model::NotebookTemplateVariableAvailableValuesQuerySearch,
    ) -> NotebookTemplateVariableAvailableValuesQueryLogRumSpans {
        NotebookTemplateVariableAvailableValuesQueryLogRumSpans {
            data_source,
            group_by,
            search,
            _unparsed: false,
        }
    }
}

impl<'de> Deserialize<'de> for NotebookTemplateVariableAvailableValuesQueryLogRumSpans {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct NotebookTemplateVariableAvailableValuesQueryLogRumSpansVisitor;
        impl<'a> Visitor<'a> for NotebookTemplateVariableAvailableValuesQueryLogRumSpansVisitor {
            type Value = NotebookTemplateVariableAvailableValuesQueryLogRumSpans;

            fn expecting(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("a mapping")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'a>,
            {
                let mut data_source: Option<String> = None;
                let mut group_by: Option<Vec<crate::datadogV1::model::NotebookTemplateVariableAvailableValuesQueryGroupBy>> = None;
                let mut search: Option<
                    crate::datadogV1::model::NotebookTemplateVariableAvailableValuesQuerySearch,
                > = None;
                let mut _unparsed = false;

                while let Some((k, v)) = map.next_entry::<String, serde_json::Value>()? {
                    match k.as_str() {
                        "data_source" => {
                            data_source =
                                Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "group_by" => {
                            group_by = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        "search" => {
                            search = Some(serde_json::from_value(v).map_err(M::Error::custom)?);
                        }
                        &_ => {
                            return Err(serde::de::Error::custom(
                                "Additional properties not allowed",
                            ));
                        }
                    }
                }
                let data_source =
                    data_source.ok_or_else(|| M::Error::missing_field("data_source"))?;
                let group_by = group_by.ok_or_else(|| M::Error::missing_field("group_by"))?;
                let search = search.ok_or_else(|| M::Error::missing_field("search"))?;

                let content = NotebookTemplateVariableAvailableValuesQueryLogRumSpans {
                    data_source,
                    group_by,
                    search,
                    _unparsed,
                };

                Ok(content)
            }
        }

        deserializer.deserialize_any(NotebookTemplateVariableAvailableValuesQueryLogRumSpansVisitor)
    }
}
