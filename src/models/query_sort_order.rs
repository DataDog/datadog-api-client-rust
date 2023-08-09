/*
 * Datadog API V2 Collection
 *
 * Collection of all Datadog Public endpoints.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: support@datadoghq.com
 * Generated by: https://openapi-generator.tech
 */

/// QuerySortOrder : Direction of sort.

/// Direction of sort.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum QuerySortOrder {
    #[serde(rename = "asc")]
    ASC,
    #[serde(rename = "desc")]
    DESC,

}

impl ToString for QuerySortOrder {
    fn to_string(&self) -> String {
        match self {
            Self::ASC => String::from("asc"),
            Self::DESC => String::from("desc"),
        }
    }
}

impl Default for QuerySortOrder {
    fn default() -> QuerySortOrder {
        Self::ASC
    }
}




