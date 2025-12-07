use crate::db::{delete_task, get_all_tasks, insert_task, update_task_status, Task};
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Home {}

    }
}

#[component]
pub fn Home() -> Element {
    let tasks = use_signal::<Vec<Task>>(|| vec![]);

    use_effect({
        let mut tasks = tasks.clone();
        move || {
            if let Ok(all) = get_all_tasks() {
                tasks.set(all);
            }
        }
    });

    rsx! {
        div {
            class: "bg-gray-950 text-white min-h-screen p-6 font-sans",
            div {
                class: "max-w-4xl mx-auto",
                h1 {
                    class: "text-3xl font-bold mb-6 text-center text-red-500",
                    "My Todo List"
                },
                Input { tasks: tasks.clone() },
                TaskList { tasks }
            }

        }
    }
}

#[component]
fn Input(tasks: Signal<Vec<Task>>) -> Element {
    let mut new_title = use_signal(|| String::new());
    let mut new_tag = use_signal(|| "gameplay".to_string());

    let add_task = move |_| {
        if new_title.read().trim().is_empty() {
            return;
        }

        let new_task = Task {
            id: 0,
            title: new_title(),
            tag: new_tag(),
            status: "Pending".to_string(),
        };

        insert_task(&new_task).expect("Insert failed");

        tasks.write().push(new_task);

        new_title.set("".into());
    };

    rsx! {
        div {
            class: "flex flex-col md:flex-row gap-4 mb-8",
            input {
                class: "flex-1 px-4 py-2 rounded-lg bg-gray-800 border border-gray-700 focus:outline-none focus:ring-2 focus:ring-red-500",
                placeholder: "New Task",
                oninput: move |e| new_title.set(e.value().clone()),
            },
            select {
                class: "pl-4 pr-12 py-2 rounded-lg bg-gray-800 border border-gray-700 text-white appearance-none focus:outline-none focus:ring-2 focus:ring-red-500",
                onchange: move |e: Event<FormData>| {
                    new_tag.set(e.value());
                },
                option { value: "gameplay", "🎮 Gameplay" }
                option { value: "guide", "📘 Guide" }
                option { value: "shorts", "🎬 Shorts" }
                option { value: "tutorial", "🎓 Tutorial" }
                option { value: "news", "📰 News" }
                option { value: "event", "🎉 Event" }
            },
            button {
                class: "px-6 py-2 bg-red-600 hover:bg-red-700 rounded-lg text-white font-semibold",
                onclick: add_task,
                "Add Task",
            }
        }
    }
}

#[component]
fn TaskList(tasks: Signal<Vec<Task>>) -> Element {
    let tasks_read = tasks.read();
    rsx! {
            div {
                class: "space-y-4",
                for (idx, task) in tasks_read.iter().enumerate() {
                    TaskItem { task: task.clone(), idx: idx, tasks: tasks.clone() }
            }
        }
    }
}

#[component]
fn TaskItem(task: Task, idx: usize, tasks: Signal<Vec<Task>>) -> Element {
    let mut status = use_signal(|| task.status.clone());
    rsx! {
        div {
            class: "flex items-start justify-between p-4 bg-gray-900 rounded-lg border border-gray-700",
            div {
                h3 { class: "text-lg font-semibold", "{task.title}" }
                p {
                    class: "text-sm text-gray-400",
                    "Tag: ",
                    span { class: "text-yellow-400", "{task.tag}" }
                }
            },
            div {
                class: "flex items-center gap-4",
                select {
                    class: "pl-4 pr-12 py-2 rounded-lg bg-gray-800 border border-gray-700 text-white appearance-none focus:outline-none focus:ring-2 focus:ring-red-500",
                    onchange: move |ev| {
                        let new_status = ev.value().clone();
                        if let Err(e) = update_task_status(task.id, &new_status) {
                            println!("Failed to update task: {:?}", e);
                        } else {
                            status.set(new_status.clone());
                            tasks.write()[idx].status = new_status;
                        }
                    },
                    value: "{status}",
                    option { value: "Pending", "⏳ Pending" }
                    option { value: "In Progress", "🔄 In Progress" }
                    option { value: "Uploaded", "📤 Uploaded" }
                    option { value: "Completed", "✅ Completed" }
                    option { value: "On Hold","⏸️ On Hold" }
                    option { value: "Archived",  "🗃️ Archived" }
                },
                button {
                    class: "text-red-400 hover:text-red-600 text-sm",
                    onclick: move |_| {
                        if let Err(e) = delete_task(task.id) {
                            println!("Failed to delete task: {:?}", e);
                        } else {
                            tasks.write().remove(idx);
                        }
                    },
                    "✕"
                }
            }
        }
    }
}
