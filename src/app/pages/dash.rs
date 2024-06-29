use leptos_use::{use_cookie, utils::FromToStringCodec};

use super::*;
use crate::server_fns::user::DisplayUserModel;

#[component]
pub fn DashboardPage() -> impl IntoView
{
	use crate::server_fns::user::current::get_current_user;

	// Determine if there is a logged in user cookie
	let (access_token, _) = use_cookie::<String, FromToStringCodec>("leptos_access_token");

	// Needs to be a local resource so that it only gets created once
	let usr = create_blocking_resource(move || access_token.get(), get_current_user);

	view! {
		<Suspense fallback=move || {
			view! { <div></div> }
		}>
			<div>
				<div>
					{move || {
						usr.get()
							.map(|data| {
								let rr = data.unwrap();
								let tt = rr.clone();
								view! {
									<Show
										when=move || rr.is_some()
										fallback=move || {
											view! { <div></div> }
										}
									>

										<DashOptions user=tt.clone()/>
									</Show>
								}
							})
					}}

				</div>

			</div>
		</Suspense>
	}
}

#[island]
pub fn DashOptions(user: Option<DisplayUserModel>) -> impl IntoView
{
	// counter example to check functions still work
	let (counter, set_counter) = use_cookie::<u32, FromToStringCodec>("counter");

	let reset = move || set_counter.set(Some(0));

	if counter.get_untracked().is_none()
	{
		reset();
	}

	let increase = move || {
		set_counter.set(counter.get().map(|c| c + 1));
	};

	let decrease = move || {
		if counter.get() == Some(0)
		{
			return;
		}
		set_counter.set(counter.get().map(|c| c - 1));
	};

	let Some(user) = user
	else
	{
		return view! { <div></div> }.into_view();
	};

	let username = user.username;
	let mut no_of_calendar_events = "0";
	if no_of_calendar_events == "0"
	{
		no_of_calendar_events = "no"
	}
	let event = if no_of_calendar_events != "1" { "events" } else { "event" };
	let mut no_of_reports = "0";
	if no_of_reports == "0"
	{
		no_of_reports = "no"
	}
	let report = if no_of_reports != "1" { "reports" } else { "report" };

	view! {
		<div class="container mx-auto columns-1 text-center mt-10">

			<p class="h0 text-center">"Greetings " {username}</p>

			<h3 class="mt-5 text-center">
				"You have " {no_of_calendar_events} " calendar " {event} " coming up."
			</h3>
			<button class="sm-btn mt-3" on:click=move |_| reset()>
				View Calendar
			</button>

			<h3 class="mt-5 text-center">
				"You have " {no_of_reports} " " {report} "  to review."
			</h3>
			<button class="sm-btn mt-3" on:click=move |_| reset()>
				View Reports
			</button>
		</div>

		<div class="mt-96">
			<p>
				Counter =
				{move || counter.get().map(|c| c.to_string()).unwrap_or("—".to_string())}
			</p>
			<div>
				<button class="std-btn mt-3 w-28" on:click=move |_| increase()>
					"Increase counter"
				</button>
			</div>
			<div>
				<button class="std-btn mt-3 w-28" on:click=move |_| decrease()>
					"Decrease counter"
				</button>
			</div>
			<div>
				<button class="std-btn mt-3 w-28" on:click=move |_| reset()>
					"Reset"
				</button>
			</div>
		</div>
	}.into_view()
}
