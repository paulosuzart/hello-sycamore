use crate::components::trace_input::TraceInput;
use serde_with::chrono::{DateTime, Utc};

use crate::components::trace::Trace;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sycamore::prelude::*;

mod components;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde_as]
struct TaskInfo {
    id: String,
    task_name: String,
    execution_time: DateTime<Utc>,
    consecutive_failures: u32,
    execution_version: u32,
    last_failure: Option<DateTime<Utc>>,
    last_success: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[serde_as]
struct StepTrace {
    durable_step_id: String,
    result: Option<String>,
    scheduled_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
    in_task_info: TaskInfo,
    out_task_info: Option<TaskInfo>,
}

impl StepTrace {
    fn in_task_text(&self) -> String {
        serde_json::to_string_pretty(&self.in_task_info).unwrap_or_default()
    }

    fn out_task_text(&self) -> String {
        self.out_task_info.as_ref()
            .map(|info| serde_json::to_string_pretty(&info).unwrap_or_default())
            .unwrap_or_default()
    }

    fn completed_at_text(&self) -> String {
        self
        .completed_at.as_ref()
        .map(|completed_at| completed_at.to_rfc3339())
        .unwrap_or("-".to_string())
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde_as]
struct DurableTrace {
    name: String,
    durable_execution_id: String,
    scheduled_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
    payload: Option<String>,
    result: Option<String>,
    is_error: Option<bool>,
    status: String,
    failure_reason: Option<String>,
    failure_source: Option<String>,
    version: u32,
    steps: Vec<StepTrace>,
}

#[derive(Debug, Clone, Copy)]
struct State(Signal<Option<DurableTrace>>);

#[component]
fn App() -> View {
    let state = use_context::<State>();
    let local_store = window().local_storage().unwrap().expect("No local storage");

    let saved_trace: Option<DurableTrace> = if let Ok(Some(trace)) = local_store.get_item("trace") {
        match serde_json::from_str::<DurableTrace>(&trace) {
            Ok(trace) => Some(trace),
            _ => None,
        }
    } else {
        Default::default()
    };

    state.0.set(saved_trace);
    create_effect(move || {
        state.0.with(|trace| {
            if let Some(trace) = trace {
                local_store
                    .set_item("trace", &serde_json::to_string(trace).unwrap())
                    .unwrap();
            }
        })
    });

    view! {
        (match state.0.get_clone()  {
            Some(trace) => view! {
                // Payload is there. Let's render the trace.
                Trace(trace=trace)
            },
            None => {
                view! {
                    // We don't have a payload. Ask the user.
                    TraceInput()
                }
            }})
    }
}

fn main() {
    sycamore::render(|| {
        let state = State(create_signal(None));
        provide_context(state);
        App()
    })
}
