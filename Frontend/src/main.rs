#![recursion_limit="256"]
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    input:String,
    todos: Vec<String>,
}

enum Msg {
    Add,
    Update(String),
    Remove(usize),
    RemoveAll,
    Nothing,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            todos: vec![],
            input:"".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
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
        let view_todo = |(index, todo): (usize, &String)| {
            html! {
                <li>
                    {todo}
                    <button onclick= self.link.callback(move |_| Msg::Remove(index))>{"X"}</button>
                </li>
            }
        };
        html! {
            <div>
                <h1>{"Todo App"}</h1>
               <input
                    placeholder="what do you want to do?",
                    value=&self.input,
                    oninput=self.link.callback(|e: InputData| Msg::Update(e.value)),
                    onkeypress=self.link.callback(|e: KeyPressEvent| {
                       if e.key() == "Enter" { Msg::Add } else { Msg::Nothing }
                    })/>

                <button onclick=self.link.callback(|_| Msg::RemoveAll)>{ "Delete all Todos!" }</button>
                <ul>
                    {for self.todos.iter().enumerate().map(|t| view_todo(t))}
                </ul>
                
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}