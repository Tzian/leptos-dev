pub mod dark_mode;
pub mod nav;
pub mod pages;
#[cfg(feature = "ssr")]
pub mod state;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView
{
	use leptos_use::{use_cookie, utils::FromToStringCodec};

	use crate::{app::{dark_mode::DarkModeToggle,
	                  nav::navbar::NavBar,
	                  pages::{dash::DashboardPage,
	                          home::HomePage,
	                          user::{login::LoginPage, register::RegisterPage},
	                          NotFound}},
	            server_fns::user::current::get_current_user};

	// Provides context that manages stylesheets, titles, meta tags, etc.
	provide_meta_context();

	// Determine if there is a logged in user cookie
	let (access_token, _) = use_cookie::<String, FromToStringCodec>("leptos_access_token");
	use db_utils::errors::ServicesError;

	use crate::server_fns::user::DisplayUserModel;
	// Needs to be a blocking resource because we need to wait for the cookie result before use in view
	let usr: Resource<Option<String>, Result<Option<DisplayUserModel>, ServerFnError<ServicesError>>> =
		create_blocking_resource(move || access_token.get(), get_current_user);

	view! {
		<Stylesheet id="leptos" href="/pkg/leptos-dev.css"/>

		// sets the document title
		<Title text="Welcome to AppName"/>

		<div class="flex flex-row bg-primary-900 dark:text-primary-300">
			<div class="">
				<NavBar/>
			</div>
			<div class="darkmode-toggle-position">
				<DarkModeToggle/>
			</div>
		</div>

		// content for this welcome page
		<Router>
			<main>
				<Routes>
					<Route
						path="/"
						view=move || {
							view! {
								<Suspense fallback=move || {
									"Loading...."
								}>
									{move || {
										usr.get()
											.map(|data| {
												let rr = data.unwrap();
												view! {
													<Show
														when=move || rr.is_none()
														fallback=move || view! { <Redirect path="/dashboard"/> }
													>
														<HomePage/>
													</Show>
												}
											})
									}}

								</Suspense>
							}
						}
					/>

					<Route
						path="/register"
						view=move || {
							view! {
								<Suspense fallback=move || {
									"Loading...."
								}>
									{move || {
										usr.get()
											.map(|data| {
												let rr = data.unwrap();
												view! {
													<Show
														when=move || rr.is_none()
														fallback=move || view! { <Redirect path="/dashboard"/> }
													>
														<RegisterPage/>
													</Show>
												}
											})
									}}

								</Suspense>
							}
						}
					/>

					<Route
						path="/login"
						view=move || {
							view! {
								<Suspense fallback=move || {
									"Loading...."
								}>
									{move || {
										usr.get()
											.map(|data| {
												let rr = data.unwrap();
												view! {
													<Show
														when=move || rr.is_none()
														fallback=move || view! { <Redirect path="/dashboard"/> }
													>
														<LoginPage/>
													</Show>
												}
											})
									}}

								</Suspense>
							}
						}
					/>

					<Route
						path="/dashboard"
						view=move || {
							view! {
								<Suspense fallback=move || {
									"Loading...."
								}>
									{move || {
										usr.get()
											.map(|data| {
												let rr = data.unwrap();
												view! {
													<Show
														when=move || rr.is_some()
														fallback=move || view! { <Redirect path="/login"/> }
													>

														<DashboardPage/>

													</Show>
												}
											})
									}}

								</Suspense>
							}
						}
					/>

					<Route path="/*any" view=NotFound/>
				</Routes>
			</main>
		</Router>
	}
}
