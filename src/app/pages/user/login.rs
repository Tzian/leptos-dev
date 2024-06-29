use super::*;
use crate::server_fns::user::login::UserLogin;

#[component]
pub fn LoginPage() -> impl IntoView
{
	view! {
		<Title text="Login"/>

		<p class="h0 m-t-10 text-center">"Login"</p>

		<Log/>
	}
}

#[island]
fn Log() -> impl IntoView
{
	let user_login_action = create_server_action::<UserLogin>();

	let err = Signal::derive(move || {
		(user_login_action.value())().map_or("".to_owned(), |result| {
			                             match result
			                             {
				                             Ok(_) => "Success:- User authenticated".to_string(),
			                                 Err(err) =>
			                                 {
				                                 let e = format!("{:?}", err);
				                                 if e.contains("No Record Found")
				                                 {
					                                 "Error:- Unable to find a user with those credentials. Please \
					                                  check and try again!"
					                                        .to_string()
				                                 }
				                                 else
				                                 {
					                                 "Error:- Unknown error occurred.".to_string()
				                                 }
			                                 }
			                             }
		                             })
	});

	let (read_ptype, write_ptype) = create_signal("password");

	view! {
		<Show when=move || err.get().contains("Success")>
			<div class="txt-success text-center font-bold mt-10">{err}</div>
		</Show>
		<Show when=move || err.get().contains("Error")>
			<div class="txt-error text-center font-bold mt-10">{err}</div>
		</Show>

		<div class="container mx-auto columns-1 text-center mt-10">
			<ActionForm action=user_login_action>
				<div>
					<label class="input-label" for="identity">
						"Username or Email"
					</label>
				</div>
				<div>
					<input
						class="input-fields"
						type="text"
						class="ml-9"
						name="identity"
						id="identity"
						required
					/>
				</div>

				<div class="mt-3">
					<label class="input-label" for="password">
						"Password"
					</label>
				</div>
				<div>
					<input
						class="input-fields"
						type=read_ptype
						class="ml-10"
						name="password"
						id="password"
						required
					/>
				</div>

				<div>
					<button
						class="text-xs sm-btn"
						type="button"
						on:click=move |_| {
							if read_ptype.get() == "password" {
								write_ptype.set("text")
							} else {
								write_ptype.set("password")
							}
						}
					>

						"Show Password"
					</button>
				</div>

				<div class="mt-5">
					<button class="std-btn" type="submit">
						"Login"
					</button>
				</div>
			</ActionForm>
		</div>
	}
}
