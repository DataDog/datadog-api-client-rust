// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Deserializer, Serialize};

/// The attributes of a notebook cell in create cell request. Valid cell types are `markdown`, `timeseries`, `toplist`, `heatmap`, `distribution`,
/// `log_stream`. [More information on each graph visualization type.](<https://docs.datadoghq.com/dashboards/widgets/>)
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum NotebookCellCreateRequestAttributes {
    NotebookMarkdownCellAttributes(Box<crate::datadogV1::model::NotebookMarkdownCellAttributes>),
    NotebookTimeseriesCellAttributes(
        Box<crate::datadogV1::model::NotebookTimeseriesCellAttributes>,
    ),
    NotebookToplistCellAttributes(Box<crate::datadogV1::model::NotebookToplistCellAttributes>),
    NotebookHeatMapCellAttributes(Box<crate::datadogV1::model::NotebookHeatMapCellAttributes>),
    NotebookDistributionCellAttributes(
        Box<crate::datadogV1::model::NotebookDistributionCellAttributes>,
    ),
    NotebookLogStreamCellAttributes(Box<crate::datadogV1::model::NotebookLogStreamCellAttributes>),
    UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for NotebookCellCreateRequestAttributes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::NotebookMarkdownCellAttributes>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(NotebookCellCreateRequestAttributes::NotebookMarkdownCellAttributes(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::NotebookTimeseriesCellAttributes>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    NotebookCellCreateRequestAttributes::NotebookTimeseriesCellAttributes(_v),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::NotebookToplistCellAttributes>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(NotebookCellCreateRequestAttributes::NotebookToplistCellAttributes(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::NotebookHeatMapCellAttributes>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(NotebookCellCreateRequestAttributes::NotebookHeatMapCellAttributes(_v));
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::NotebookDistributionCellAttributes>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    NotebookCellCreateRequestAttributes::NotebookDistributionCellAttributes(_v),
                );
            }
        }
        if let Ok(_v) = serde_json::from_value::<
            Box<crate::datadogV1::model::NotebookLogStreamCellAttributes>,
        >(value.clone())
        {
            if !_v._unparsed {
                return Ok(
                    NotebookCellCreateRequestAttributes::NotebookLogStreamCellAttributes(_v),
                );
            }
        }

        return Ok(NotebookCellCreateRequestAttributes::UnparsedObject(
            crate::datadog::UnparsedObject { value },
        ));
    }
}
