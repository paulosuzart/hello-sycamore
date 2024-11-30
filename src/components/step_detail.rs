use crate::StepTrace;
use sycamore::prelude::*;

// h parameter is the callback to hide the sidepanel
#[component(inline_props)]
pub(crate) fn StepDetail<F>(step_trace: StepTrace, h: F) -> View
where
    F : Fn() + Copy + 'static,
{
    let completed_at_text = step_trace.completed_at_text();
    let in_task_json_text = step_trace.in_task_text();
    let out_task_json_text = step_trace.out_task_text();
    let result_text = step_trace.result.unwrap_or("-".to_string());
    view! {
        div(class="fixed inset-y-0 right-0 w-full max-w-xl bg-white shadow-xl") {
            div(class="h-full flex flex-col") {
                div(class="flex items-center justify-between px-6 py-4 border-b border-gray-200") {
                  h2(class="text-lg font-semibold text-gray-900") { "Step Details" }
                    button(
                        on:click=move |_| h(),
                        class="rounded-md text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500") {
                        svg(
                        xmlns="http://www.w3.org/2000/svg", width="24", height="24", viewBox="0 0 24 24", fill="none",
                        stroke="currentColor", stroke-width="2", stroke-linecap="round", stroke-linejoin="round",
                        class="lucide lucide-x h-6 w-6") {
                            path(d="M18 6 6 18")
                            path(d="m6 6 12 12")
                       }
                    }
                }

                div(class="flex-1 overflow-y-auto p-6") {
                    div(class="space-y-6") {
                        div() {
                            h3(class="text-sm font-medium text-gray-500") { "Step ID" }
                            p(class="mt-1 text-sm text-gray-900") { (step_trace.durable_step_id) }
                        }
                        div(class="grid grid-cols-2 gap-4") {
                            div() {
                                h3(class="text-sm font-medium text-gray-500") { "Scheduled At" }
                                p(class="mt-1 text-sm text-gray-900") { (step_trace.scheduled_at.to_rfc3339()) }
                            }
                            div() {
                                h3(class="text-sm font-medium text-gray-500") { "Completed At" }
                                p(class="mt-1 text-sm text-gray-900") {(completed_at_text)}
                            }
                        }
                        div() {
                            h3(class="text-sm font-medium text-gray-500") {"Result"}
                            div(class="mt-2 bg-gray-50 rounded-lg p-4") {
                               pre(class="text-xs text-gray-900 whitespace-pre-wrap") { (result_text) }
                            }
                        }
                        div() {
                            h3(class="text-sm font-medium text-gray-500") {"Input Task Info"}
                            div(class="mt-2 bg-gray-50 rounded-lg p-4") {
                               pre(class="text-xs text-gray-900 whitespace-pre-wrap") { (in_task_json_text) }
                            }
                            div() {
                                h3(class="text-sm font-medium text-gray-500") {"Output Task Info"}
                                div(class="mt-2 bg-gray-50 rounded-lg p-4") {
                                    pre(class="text-xs text-gray-900 whitespace-pre-wrap") {
                                        (out_task_json_text)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
