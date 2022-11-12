use stylist::yew::styled_component;
use yew::prelude::*;

enum Msg {
	AddOne,
}

struct CounterComponent {
	count: i64,
}

// todo: figure out how to make this work
// #[styled_component(MyStyledComponent)]
// fn CounterStyledComponent() -> Html {
// 	html! {
// 			<div>
// 					<h1>{ "Counter" }</h1>
// 					<p>{ "Current count: " } <strong>{ 0 }</strong></p>
// 					<button>{ "Increment" }</button>
// 			</div>
// 	}
// }

impl Component for CounterComponent {
	type Message = Msg;
	type Properties = ();

	fn create(_ctx: &Context<Self>) -> Self {
		Self { count: 0 }
	}

	fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
		match msg {
			Msg::AddOne => {
				self.count += 1;
				true // re-render component
			}
		}
	}

	fn view(&self, ctx: &Context<Self>) -> Html {
		let link = ctx.link();
		html! {
			<div class="container">
				<p>{self.count}</p>
				<button onclick={link.callback(|_| Msg::AddOne)}>{"+1"}</button>
			</div>
		}
	}
}

fn main() {
	yew::start_app::<CounterComponent>();
}
