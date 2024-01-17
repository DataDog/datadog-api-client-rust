// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Updated list stream widget.
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListStreamQuery {
    /// Compute configuration for the List Stream Widget. Compute can be used only with the logs_transaction_stream (from 1 to 5 items) list stream source.
    #[serde(rename = "compute")]
    pub compute: Option<Vec<crate::datadogV1::model::ListStreamComputeItems>>,
    /// Source from which to query items to display in the stream.
    #[serde(rename = "data_source")]
    pub data_source: crate::datadogV1::model::ListStreamSource,
    /// Size to use to display an event.
    #[serde(rename = "event_size")]
    pub event_size: Option<crate::datadogV1::model::WidgetEventSize>,
    /// Group by configuration for the List Stream Widget. Group by can be used only with logs_pattern_stream (up to 3 items) or logs_transaction_stream (one group by item is required) list stream source.
    #[serde(rename = "group_by")]
    pub group_by: Option<Vec<crate::datadogV1::model::ListStreamGroupByItems>>,
    /// List of indexes.
    #[serde(rename = "indexes")]
    pub indexes: Option<Vec<String>>,
    /// Widget query.
    #[serde(rename = "query_string")]
    pub query_string: String,
    /// Which column and order to sort by
    #[serde(rename = "sort")]
    pub sort: Option<Box<crate::datadogV1::model::WidgetFieldSort>>,
    /// Option for storage location. Feature in Private Beta.
    #[serde(rename = "storage")]
    pub storage: Option<String>,
}

impl ListStreamQuery {
    pub fn new(
        data_source: crate::datadogV1::model::ListStreamSource,
        query_string: String,
    ) -> ListStreamQuery {
        ListStreamQuery {
            compute: None,
            data_source,
            event_size: None,
            group_by: None,
            indexes: None,
            query_string,
            sort: None,
            storage: None,
        }
    }
}
