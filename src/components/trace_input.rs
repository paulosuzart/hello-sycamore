use crate::{DurableTrace, State};

use serde_json;
use sycamore::prelude::*;
use sycamore::rt::console_error;

#[component(inline_props)]
fn TraceInputErrorModal<F>(on_hider_error: F, error_msg: ReadSignal<String>) -> View
where
    F: Fn() + Copy + 'static,
{
    view! {
        div(class="fixed z-50 inset-0 bg-gray-900 bg-opacity-60 overflow-y-auto h-full w-full px-4") {
            div(class="relative top-40 mx-auto shadow-xl rounded-md bg-white max-w-md") {
                div(class="flex justify-end p-2") {
                    button(on:click=move |_| on_hider_error(),
                        class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm p-1.5 ml-auto inline-flex items-center") {
                            svg(class="w-5 h-5", fill="currentColor", viewBox="0 0 20 20", xmlns="http://www.w3.org/2000/svg") {
                                path(fill-rule="evenodd",
                                    d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z",
                                    clip-rule="evenodd")
                            }
                        }
                }
                div(class="p-6 pt-0 text-center") {
                    svg(class="w-20 h-20 text-red-600 mx-auto", fill="none", stroke="currentColor", viewBox="0 0 24 24",
                        xmlns="http://www.w3.org/2000/svg") {
                            path(stroke-linecap="round", stroke-linejoin="round", stroke-width="2",
                                d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z")
                        }
                    h3(class="text-xl font-normal text-gray-500 mt-5 mb-6") {
                        "Invalid Json. Please paste a valid durable trace json:"
                    }
                    p(class="text-s") { (error_msg.get_clone()) }
                }
            }
        }
    }
}

#[component(inline_props)]
fn TraceInputText<F>(on_error: F, err_message: Signal<String>) -> View
where
    F: Fn() + Copy + 'static,
{
    let state = use_context::<State>();
    let payload = create_signal(String::new());
    let parse_json =
        move |_| match serde_json::from_str::<DurableTrace>(payload.get_clone().as_str()) {
            Ok(p) => {
                state.0.set(Some(p));
            }
            Err(e) => {
                console_error!("{}", e);
                on_error();
                err_message.set(e.to_string());
            }
        };
    view! {
        div(class="max-w-xl mx-auto mt-16 flex w-full flex-col border rounded-lg bg-white p-8") {
            h2(class="title-font mb-1 text-lg font-medium text-gray-900") { "Durable Trace" }
            p(class="mb-5 leading-relaxed text-gray-600") { "Please paste the durable trace json" }
            div(class="mb-4") {
                label(class="text-sm leading-7 text-gray-600") {"Payload" }
                textarea(bind:value=payload,
                    id="payload", name="payload", class="h-32 w-full resize-none rounded border border-gray-300 bg-white py-1 px-3 text-base leading-6 text-gray-700 outline-none transition-colors duration-200 ease-in-out focus:border-indigo-500 focus:ring-2 focus:ring-indigo-200")
            }
            button(on:click=parse_json,
                class="rounded border-0 bg-indigo-500 py-2 px-6 text-lg text-white hover:bg-indigo-600 focus:outline-none") { "Load Trace" }
        }
    }
}

#[component]
pub fn TraceInput() -> View {
    let show_error = create_signal(false);
    let set_show_error = move || show_error.set(true);
    let set_hide_error = move || show_error.set(false);
    let err_msg = create_signal(String::new());
    let err_msg_read = create_memo(move || err_msg.get_clone());
    view! {
        (if show_error.get() {
            view! {TraceInputErrorModal(on_hider_error=set_hide_error,error_msg=err_msg_read)}
        } else {
            view! {TraceInputText(on_error=set_show_error, err_message=err_msg)}
        })
    }
}
