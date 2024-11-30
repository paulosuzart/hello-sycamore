use crate::components::step_detail::StepDetail;
use crate::components::util;
use crate::{DurableTrace, State, StepTrace};
use serde_with::chrono::{DateTime, Utc};
use std::clone::Clone;
use sycamore::prelude::*;

#[derive(Props)]
pub struct TraceProps {
    trace: DurableTrace,
}

#[component(inline_props)]
fn Status(status: String, version: u32) -> View {
    let (border, label) = match status.as_str() {
        "running" => ("blue", "running"),
        "failed" => ("red", "failed"),
        "completed" => ("green", "completed"),
        _ => ("grey", "unknown"),
    };
    let span_class = format!(
        "border-{}-300 border rounded-full px-4 text-sm text-{}-700 py-0.5",
        border, border
    );
    view! {
        div(class="space-x-2 text-sm") {
            span() {("Status:")}
            span(class=span_class) { (label) }
            span() {(">")}
            span() { "Version: " (version)}
        }
    }
}

#[component(inline_props)]
fn Header(name: String, payload: String, status: String, duration: String, version: u32) -> View {
    let state = use_context::<State>();
    let clear_state = move |_| state.0.set(None);
    let duration_text_size = if duration.len() > 50 {
        "text-s"
    } else {
        "text-m"
    };
    view! {
        div(class="mb-6") {
            div(class="flex items-center justify-between") {
              div(class="text-2xl font-semibold text-gray-900") {
                    h1(class="font-semibold text-gray-900") {
                        "Durable Trace (" (name) ")"
                    }
                }
                div(class="flex items-center gap-4") {
                    button(
                        on:click=clear_state,
                        class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50") {
                      svg(xmlns="http://www.w3.org/2000/svg", width="24", height="24", viewBox="0 0 24 24", fill="none", stroke="currentColor", stroke-width="2",
                            stroke-linecap="round", stroke-linejoin="round", class="lucide lucide-code w-4 h-4") {
                         polyline(points="16 18 22 12 16 6") {}
                            polyline(points="8 6 2 12 8 18") {}
                        } "Load JSON"
                    }

                  div() {
                    span(class=format!("font-semibold py-1 px-2.5 border-none rounded bg-blue-100 {} text-blue-800 font-medium",
                            duration_text_size) ) {
                        "Total Duration: " (duration)
                    }
                  }
                }

            }
        }

        Status(status=status, version=version)

        div(class="mt-6 bg-gray-50 rounded-lg p-4") {
            h3(class="text-sm font-medium text-gray-700 mb-2") {"Input Payload" }
            pre(class="text-xs text-gray-600 whitespace-pre-wrap overflow-auto max-h-32") {
                (payload)
            }
        }
        hr(class="mb-2")
    }
}

#[component(inline_props)]
fn Summary(id: String) -> View {
    view! {
        table(class="w-full mb-8") {
            thead() {
                tr() {
                    th(class="text-left font-bold text-gray-700") { "Id" }
                }
            }
            tbody() {
                tr() {
                    td(class="text-left text-gray-700") {(id)}
                }
            }
        }
    }
}

#[component(inline_props)]
fn StepItem(
    max_completion: DateTime<Utc>,
    delta_window: f64,
    second_rate: f64,
    step: StepTrace,
    step_detail: Signal<Option<StepTrace>>,
) -> View {
    // difference between the latest completed date to the scheduler at of the task
    let min_delta = max_completion - step.scheduled_at;
    // the position in % where the step should tart to be rendered
    let min_position = (delta_window - min_delta.num_seconds() as f64) * 100.0 / delta_window;
    // if the step is completed, we get the difference from the scheduled_at
    let max_delta = step
        .completed_at
        .map(|completed_at| completed_at - step.scheduled_at);

    // the total width of the step is the duration times how much each second occupies on the screen
    let task_width = max_delta
        .map(|max_d| max_d.num_seconds() as f64)
        .unwrap_or_else(|| delta_window * 0.02) // defaults to 20%
        * second_rate;

    let duration_text = match step.completed_at {
        Some(completed_at) => format!(
            "{} seconds",
            (completed_at - step.scheduled_at).num_seconds()
        ),
        None => "Not completed".to_string(),
    };
    let start_at_text = step.scheduled_at.to_rfc3339();
    let end_at_text = step
        .completed_at
        .map(|completed_at| completed_at.to_rfc3339())
        .unwrap_or_else(|| "-".to_string());

    let step_id = step.durable_step_id.clone();
    let on_show = move |_| {
        step_detail.set(Some(step.clone()));
    };
    view! {
        div(class="relative") {
            div(class="flex items-center mb-2") {
             span(class="text-sm font-medium text-gray-900") { (step_id) }
             span(class="ml-2 text-xs text-gray-500"){ (duration_text) }
            }
        }
        div(class="h-8 relative bg-gray-100 rounded-lg overflow-hidden") {
         button(
                on:click=on_show,
                class="absolute h-full transition-all bg-blue-600 hover:bg-blue-600",
                style=format!("left: {}%; width: {}%;",  min_position, task_width)) {

                span(class="sr-only") { "View details for assign-card-trx"}
            }
        }
        div(class="flex justify-between mt-1 text-xs text-gray-500"){
            span() { (start_at_text) }
            span() { (end_at_text) }
        }
    }
}

#[component(inline_props)]
pub fn Steps(
    steps: Vec<StepTrace>,
    durable_scheduled_at: DateTime<Utc>,
    durable_completed_at: Option<DateTime<Utc>>,
) -> View {
    let max_completion =
        util::find_max_completion(&steps, durable_scheduled_at, durable_completed_at);
    let min_completion = durable_scheduled_at;
    let delta_window = (max_completion - min_completion).num_seconds() as f64;
    let second_rate = 1.0 * 100.0 / delta_window;
    let step_detail = create_signal(Option::<StepTrace>::None);
    let hide_detail = move || step_detail.set(None);

    view! {
        div(class="space-y-6") {
            Keyed(list=steps,
            view=move |step| view! {
                    StepItem(max_completion=max_completion,
                        delta_window=delta_window,
                        second_rate=second_rate, step=step,
                        step_detail=step_detail)
            },
            key=|step| step.durable_step_id.clone())
        }
        (match step_detail.get_clone() {
            Some(t) => view! { StepDetail(step_trace=t, h=hide_detail) },
            _ => view! {},
        })
    }
}

#[component]
pub fn Trace(props: TraceProps) -> View {
    let delta_opt = props
        .trace
        .completed_at
        .map(|completed_at| completed_at.signed_duration_since(props.trace.scheduled_at));

    let duration_string = match delta_opt {
        Some(delta) => util::get_duration_string(delta),
        None => "-".to_string(),
    };

    let step_view = match props.trace.completed_at {
        Some(completed_at) => view! {
            Steps(steps=props.trace.steps, durable_scheduled_at=props.trace.scheduled_at, durable_completed_at=completed_at)
        },
        None => view! {
            Steps(steps=props.trace.steps, durable_scheduled_at=props.trace.scheduled_at)
        },
    };

    view! {
        div(class="bg-white border rounded-lg shadow-lg mx-auto mt-8 mx-auto max-w-7xl py-4 sm:px-6 sm:py-12 lg:px-8") {
            Header(name=props.trace.name,
                payload=props.trace.payload.unwrap_or_else(|| "No Payload".to_string()),
                status=props.trace.status,
                duration=duration_string,
                version=props.trace.version)
            Summary(id=props.trace.durable_execution_id.clone())
            (step_view)
        }
    }
}
