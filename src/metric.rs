#[derive(Clone, Debug, Serialize)]
pub struct HostMetricValue {
    #[serde(rename(serialize = "hostId"))]
    pub host_id: String,
    pub name: String,
    pub time: u64,
    pub value: f64,
}
