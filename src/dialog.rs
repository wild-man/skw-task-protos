use skw_lib_shared::prelude::{rabbitmq::SkwMqTaskTrait, serde::*, uuid::*};

/// Generate the answer for a Pending assistant message (published by `Ask`).
#[derive(Debug, Serialize, Deserialize)]
pub struct DialogAskTask {
    pub dialog_id: Uuid,
    pub message_id: Uuid,
}

impl SkwMqTaskTrait for DialogAskTask {
    fn task_name() -> &'static str {
        "DialogAskTask"
    }
}
