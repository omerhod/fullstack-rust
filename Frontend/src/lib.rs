#![recursion_limit="256"]

use crate::Msg::SetMarkdownFetchState;
use std::fmt::{Error, Formatter};
use yew::events::KeyboardEvent;
use std::future::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, Window};
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::prelude::*;
use serde::Deserialize;


pub fn send_future<COMP: Component, F>(link: ComponentLink<COMP>, future: F)
where
    F: Future<Output = COMP::Message> + 'static,
{
    spawn_local(async move {
        link.send_message(future.await);
    });
}
struct Model {
    link: ComponentLink<Self>,
    input:String,
    todos: Vec<String>,
    todoItems: FetchState<String>,
}

enum Msg {
    Add,
    Update(String),
    Remove(usize),
    RemoveAll,
    Nothing,
    Fetch,
    SetMarkdownFetchState(FetchState<String>),
}
#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}
impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        std::fmt::Debug::fmt(&self.err, f)
    }
}
impl std::error::Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        FetchError { err: value }
    }
}
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

#[derive(Deserialize, Debug)]
struct Item {
    id: i8, 
    list_id: i8, 
    title: String,
    checked: bool,
}
async fn fetch_markdown() -> Result<String, FetchError> {
        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(
            "http://localhost:8080/todos/1/items",
            &opts,
        )?;

        let window: Window = web_sys::window().unwrap();
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        assert!(resp_value.is_instance_of::<Response>());

        let resp: Response = resp_value.dyn_into().unwrap();

        let text = JsFuture::from(resp.text()?).await?;
        Ok(text.as_string().unwrap())
}


fn getJson(data: &str) -> Vec<Item> {
    serde_json::from_str(data).unwrap()
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            todos: vec![],
            input:"".to_string(),
            todoItems: FetchState::NotFetching
        }
    }

   
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetMarkdownFetchState(fetch_state) => {
                self.todoItems = fetch_state;
            }
            Msg::Fetch => {
                let future = async {
                    match fetch_markdown().await {
                        Ok(md) => Msg::SetMarkdownFetchState(FetchState::Success(md)),
                        Err(err) => Msg::SetMarkdownFetchState(FetchState::Failed(err)),
                    }
                };
                send_future(self.link.clone(), future);
                self.link
                    .send_message(SetMarkdownFetchState(FetchState::Fetching));
            }
            Msg::Add => {
                let t = 
                self.input.clone();
                self.todos.push(t);
                self.input = "".to_string();
            }
            Msg::Update(s) => {
                self.input = s
            }
            Msg::Remove(index) => {
                self.todos.remove(index);
            }
            Msg::RemoveAll => {
                self.todos = vec![];
            }
            Msg::Nothing => {}
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {  
        let view_todo = |(index, todo): (usize, &Item)| {
                html! {
                    <li>
                        {todo.title.clone()}
                        // <button onclick= self.link.callback(move |_| Msg::Remove(index))>{"X"}</button>
                    </li>
                }
        };
        let view_todo_str = |(index, todo): (usize, &String)| {
                html! {
                    <li>
                        {todo}
                        <button onclick= self.link.callback(move |_| Msg::Remove(index))>{"X"}</button>
                    </li>
                }
        };
        match &self.todoItems {
         FetchState::NotFetching => html! { 
                <div>
                    <h1>{"Todo App"}</h1>
                // <input
                //         placeholder="what do you want to do?",
                //         value=&self.input,
                //         oninput=self.link.callback(|e: InputData| Msg::Update(e.value)),
                //         onkeypress=self.link.callback(|e: KeyboardEvent| {
                //         if e.key() == "Enter" { Msg::Add } else { Msg::Nothing }
                //         })/> 
                        <button onclick=self.link.callback(|_| Msg::Fetch)>
                            {"Get Todo Items!"}
                        </button>

                </div>
        },
        FetchState::Fetching => html! {"Fetching"},
            FetchState::Success(data) => html! {
                             <div>
                    <h1>{"Todo App"}</h1>
                <input
                        placeholder="what do you want to do?",
                        value=&self.input,
                        oninput=self.link.callback(|e: InputData| Msg::Update(e.value)),
                        onkeypress=self.link.callback(|e: KeyboardEvent| {
                        if e.key() == "Enter" { Msg::Add } else { Msg::Nothing }
                        })/> 
                        // <button onclick=self.link.callback(|_| Msg::Fetch)>
                        //     {"Get Todo Items!"}
                        // </button>

                    <button onclick=self.link.callback(|_| Msg::RemoveAll)>{ "Delete all Todos!" }</button>
                    <ul>
                        {for getJson(&data).iter().enumerate().map(|t| view_todo(t))}
                    </ul>
                    <ul>
                        {for self.todos.iter().enumerate().map(|t| view_todo_str(t))}
                    </ul>
                    
                </div>
            },
            FetchState::Failed(err) => html! {&err},
    }
}
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<Model>();
}
