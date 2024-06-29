use leptos::*;
use leptos_meta::{Html, Meta};
use leptos_router::ActionForm;

#[server(ToggleDarkMode, "/api")]
pub async fn toggle_dark_mode(prefers_dark: bool) -> Result<bool, ServerFnError>
{
	use actix_web::http::header::{HeaderMap, HeaderValue, SET_COOKIE};
	use leptos_actix::{ResponseOptions, ResponseParts};

	let response = use_context::<ResponseOptions>().expect("Failed to get ResponseOptions");
	let mut response_parts = ResponseParts::default();
	let mut headers = HeaderMap::new();
	headers.insert(
	               SET_COOKIE,
	               HeaderValue::from_str(&format!("darkmode={prefers_dark}; Path=/")).expect(
		"Failed to create header value for cookie"
	)
	);
	response_parts.headers = headers;

	std::thread::sleep(std::time::Duration::from_millis(250));

	response.overwrite(response_parts);
	Ok(prefers_dark)
}

#[cfg(not(feature = "ssr"))]
fn initial_prefers_dark() -> bool
{
	use wasm_bindgen::JsCast;

	let doc = document().unchecked_into::<web_sys::HtmlDocument>();
	let cookie = doc.cookie().unwrap_or_default();
	cookie.contains("darkmode=true")
}

#[cfg(feature = "ssr")]
fn initial_prefers_dark() -> bool
{
	use_context::<actix_web::HttpRequest>().and_then(|req| {
		                                       req.cookies()
		                                          .map(|cookies| {
			                                          cookies.iter()
			                                                 .any(|cookie| {
				                                                 cookie.name() == "darkmode" && cookie.value() == "true"
			                                                 })
		                                          })
		                                          .ok()
	                                       })
	                                       .unwrap_or(false)
}

#[component]
pub fn DarkModeToggle() -> impl IntoView
{
	let initial = initial_prefers_dark();
	let toggle_dark_mode_action = create_server_action::<ToggleDarkMode>();

	// input is `Some(value)` when pending, and `None` if not pending
	let input = toggle_dark_mode_action.input();
	// value contains most recently-returned value
	let value = toggle_dark_mode_action.value();

	let prefers_dark = move || {
		match (input.get(), value.get())
		{
			// if there's some current input, use that optimistically
			(Some(submission), _) => submission.prefers_dark,
			// otherwise, if there was a previous value confirmed by server, use that
			(_, Some(Ok(value))) => value,
			// otherwise, use the initial value
			_ => initial
		}
	};

	let color_scheme = move || {
		if prefers_dark()
		{
			"dark".to_string()
		}
		else
		{
			"light".to_string()
		}
	};

	let night = "☾";
	let day = "☼";

	view! {
		<Html class=color_scheme/>

		<Meta name="color-scheme" content=color_scheme/>

		<div class="darkmode-toggle">
			<ActionForm action=toggle_dark_mode_action>
				<input
					type="hidden"
					name="prefers_dark"
					value=move || (!prefers_dark()).to_string()
				/>
				<input
					type="submit"
					value=move || {
						if prefers_dark() {
							"    ".to_owned() + night + ""
						} else {
							" ".to_owned() + day + "    "
						}
					}

					class="text-yellow-500 dark:text-blue-500 font-bold text-xl -mt-2"
				/>
			</ActionForm>
		</div>
	}
}
