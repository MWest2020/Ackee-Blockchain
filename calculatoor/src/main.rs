use yew::prelude::*;



// So I guess we need to extract this component later :D

#[function_component(App)]
pub fn app() -> Html {

    // write logic here 

    let mut input1 = use_state(||0.0);
    let mut input2 = use_state(||0.0);
    let mut operator = use_state(|| String::from("+"));



    // render template here

    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Calculatoorrr!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
            <div>
                <button>{1}</button>
                <button>{2}</button>
                <button>{3}</button>
                <span>{" "}</span>
                <button>{"+"}</button>
            </div>
            <div>
                <button>{4}</button>
                <button>{5}</button>
                <button>{6}</button>
            </div>
            <div>
                <button>{7}</button>
                <button>{8}</button>
                <button>{9}</button>
            </div>
            <button>{0}</button>
            <button>{"-"}</button>
            <button>{"*"}</button>
            <button>{"/"}</button>
            <button>{"="}</button>
            <button>{"C"}</button>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}


fn calculate(input1: f64, operator: &str, input2: f64) -> f64 {
    match operator {
        "+" => input1 + input2,
        "-" => input1 - input2,
        "*" => input1 * input2,
        "/" => input1 / input2,
        _ => panic!("Invalid operator"),
    }
}