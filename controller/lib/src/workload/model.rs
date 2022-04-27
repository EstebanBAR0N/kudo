use serde::Deserialize;

pub struct Workload {
    pub id: String,
    pub name: String,
    pub workload_type: String,
    pub uri: String,
    pub ports: Vec<String>,
    pub env: Vec<String>,
    pub resources: Resources,
}

#[derive(Deserialize)]
pub struct WorkloadInfo {
    pub name: String,
    pub workload_type: String,
    pub uri: String,
    pub ports: Vec<String>,
    pub env: Vec<String>,
    pub resources: Resources,
}

#[derive(Deserialize)]
pub struct Resources {
    pub cpu: i32,
    pub memory: i32,
    pub disk: i32,
}
