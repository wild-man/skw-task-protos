use skw_lib_shared::prelude::{rabbitmq::SkwMqTaskTrait, serde::*, uuid::*};

/// Generate the answer for a Pending assistant message (published by `Ask`).
#[derive(Debug, Serialize, Deserialize)]
pub struct DialogAskTask {
    pub dialog_id: Uuid,
    pub message_id: Uuid,
    /// Signature of the `Ask` request: the final reply (`AskAnswer`) goes to the gates' back topic
    /// under it. `None` for tasks queued before it existed (their answer is only stored).
    #[serde(default)]
    pub signature: Option<String>,
}

impl SkwMqTaskTrait for DialogAskTask {
    fn task_name() -> &'static str {
        "DialogAskTask"
    }
}
