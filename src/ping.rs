use serde_derive::{Deserialize, Serialize};
use skw_lib_shared::prelude::{rabbitmq::SkwMqTaskTrait, uuid::*};

#[derive(Debug, Serialize, Deserialize)]
pub struct PingTask {
    pub api_key: Uuid,
    pub signature: String,
    pub message: String,
}

impl SkwMqTaskTrait for PingTask {
    fn task_name() -> &'static str {
        "PingTask"
    }
}
