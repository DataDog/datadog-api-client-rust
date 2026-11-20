// Unless explicitly stated otherwise all files in this repository are licensed under the Apache-2.0 License.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2019-Present Datadog, Inc.
use serde::{Deserializer, Deserialize, Serialize};

/// Test-only generated schema for the dashboard versioning proof of concept.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DashboardBackendWidget {
	AlertGraphBackendWidget(Box<crate::datadogV1_20270101::model::AlertGraphBackendWidget>),
	AlertValueBackendWidget(Box<crate::datadogV1_20270101::model::AlertValueBackendWidget>),
	BarChartBackendWidget(Box<crate::datadogV1_20270101::model::BarChartBackendWidget>),
	ChangeBackendWidget(Box<crate::datadogV1_20270101::model::ChangeBackendWidget>),
	CheckStatusBackendWidget(Box<crate::datadogV1_20270101::model::CheckStatusBackendWidget>),
	CohortBackendWidget(Box<crate::datadogV1_20270101::model::CohortBackendWidget>),
	DistributionBackendWidget(Box<crate::datadogV1_20270101::model::DistributionBackendWidget>),
	EmbeddedAppBackendWidget(Box<crate::datadogV1_20270101::model::EmbeddedAppBackendWidget>),
	EventStream(Box<crate::datadogV1_20270101::model::EventStream>),
	EventTimelineBackendWidget(Box<crate::datadogV1_20270101::model::EventTimelineBackendWidget>),
	FreeTextBackendWidget(Box<crate::datadogV1_20270101::model::FreeTextBackendWidget>),
	FunnelBackendWidget(Box<crate::datadogV1_20270101::model::FunnelBackendWidget>),
	GeomapBackendWidget(Box<crate::datadogV1_20270101::model::GeomapBackendWidget>),
	GroupBackendWidget(Box<crate::datadogV1_20270101::model::GroupBackendWidget>),
	HeatgridBackendWidget(Box<crate::datadogV1_20270101::model::HeatgridBackendWidget>),
	HeatmapBackendWidget(Box<crate::datadogV1_20270101::model::HeatmapBackendWidget>),
	HostmapBackendWidget(Box<crate::datadogV1_20270101::model::HostmapBackendWidget>),
	IframeBackendWidget(Box<crate::datadogV1_20270101::model::IframeBackendWidget>),
	ImageBackendWidget(Box<crate::datadogV1_20270101::model::ImageBackendWidget>),
	JourneyMapBackendWidget(Box<crate::datadogV1_20270101::model::JourneyMapBackendWidget>),
	ListStreamBackendWidget(Box<crate::datadogV1_20270101::model::ListStreamBackendWidget>),
	LogStream(Box<crate::datadogV1_20270101::model::LogStream>),
	ManageStatusBackendWidget(Box<crate::datadogV1_20270101::model::ManageStatusBackendWidget>),
	NoteBackendWidget(Box<crate::datadogV1_20270101::model::NoteBackendWidget>),
	PointPlotBackendWidget(Box<crate::datadogV1_20270101::model::PointPlotBackendWidget>),
	PowerpackBackendWidget(Box<crate::datadogV1_20270101::model::PowerpackBackendWidget>),
	QueryTableBackendWidget(Box<crate::datadogV1_20270101::model::QueryTableBackendWidget>),
	QueryValueBackendWidget(Box<crate::datadogV1_20270101::model::QueryValueBackendWidget>),
	RetentionCurveBackendWidget(Box<crate::datadogV1_20270101::model::RetentionCurveBackendWidget>),
	RunWorkflowBackendWidget(Box<crate::datadogV1_20270101::model::RunWorkflowBackendWidget>),
	SankeyBackendWidget(Box<crate::datadogV1_20270101::model::SankeyBackendWidget>),
	ScatterplotBackendWidget(Box<crate::datadogV1_20270101::model::ScatterplotBackendWidget>),
	ServiceMapBackendWidget(Box<crate::datadogV1_20270101::model::ServiceMapBackendWidget>),
	SloBackendWidget(Box<crate::datadogV1_20270101::model::SloBackendWidget>),
	SloListBackendWidget(Box<crate::datadogV1_20270101::model::SloListBackendWidget>),
	SplitGroupBackendWidget(Box<crate::datadogV1_20270101::model::SplitGroupBackendWidget>),
	SunburstBackendWidget(Box<crate::datadogV1_20270101::model::SunburstBackendWidget>),
	TimeseriesBackendWidget(Box<crate::datadogV1_20270101::model::TimeseriesBackendWidget>),
	ToplistBackendWidget(Box<crate::datadogV1_20270101::model::ToplistBackendWidget>),
	TopologyMapBackendWidget(Box<crate::datadogV1_20270101::model::TopologyMapBackendWidget>),
	ServiceSummaryBackendWidget(Box<crate::datadogV1_20270101::model::ServiceSummaryBackendWidget>),
	TreemapBackendWidget(Box<crate::datadogV1_20270101::model::TreemapBackendWidget>),
	WildcardBackendWidget(Box<crate::datadogV1_20270101::model::WildcardBackendWidget>),
	UnparsedObject(crate::datadog::UnparsedObject),
}

impl<'de> Deserialize<'de> for DashboardBackendWidget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::AlertGraphBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::AlertGraphBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::AlertValueBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::AlertValueBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::BarChartBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::BarChartBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ChangeBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::ChangeBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::CheckStatusBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::CheckStatusBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::CohortBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::CohortBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::DistributionBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::DistributionBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::EmbeddedAppBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::EmbeddedAppBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::EventStream>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::EventStream(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::EventTimelineBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::EventTimelineBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FreeTextBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::FreeTextBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::FunnelBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::FunnelBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GeomapBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::GeomapBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::GroupBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::GroupBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::HeatgridBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::HeatgridBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::HeatmapBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::HeatmapBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::HostmapBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::HostmapBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::IframeBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::IframeBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ImageBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::ImageBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::JourneyMapBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::JourneyMapBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ListStreamBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::ListStreamBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::LogStream>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::LogStream(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ManageStatusBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::ManageStatusBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::NoteBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::NoteBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::PointPlotBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::PointPlotBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::PowerpackBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::PowerpackBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryTableBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::QueryTableBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::QueryValueBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::QueryValueBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::RetentionCurveBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::RetentionCurveBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::RunWorkflowBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::RunWorkflowBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::SankeyBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::SankeyBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ScatterplotBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::ScatterplotBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ServiceMapBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::ServiceMapBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::SloBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::SloBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::SloListBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::SloListBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::SplitGroupBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::SplitGroupBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::SunburstBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::SunburstBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TimeseriesBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::TimeseriesBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ToplistBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::ToplistBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TopologyMapBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::TopologyMapBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::ServiceSummaryBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::ServiceSummaryBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::TreemapBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::TreemapBackendWidget(_v));
            }
        }
		if let Ok(_v) = serde_json::from_value::<Box<crate::datadogV1_20270101::model::WildcardBackendWidget>>(value.clone()) {
			if !_v._unparsed {
                return Ok(DashboardBackendWidget::WildcardBackendWidget(_v));
            }
        }
	
        return Ok(DashboardBackendWidget::UnparsedObject(crate::datadog::UnparsedObject { value }));
    }
}
