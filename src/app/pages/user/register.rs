use super::*;
use crate::server_fns::user::register::RegisterUser;

#[component]
pub fn RegisterPage() -> impl IntoView
{
	view! {
		<Title text="Register"/>

		<p class="h0 m-t-10 text-center">"Register New User"</p>

		<Reg/>
	}
}

#[island]
fn Reg() -> impl IntoView
{
	let register_user_action = create_server_action::<RegisterUser>();

	let err = Signal::derive(move || {
		(register_user_action.value())().map_or("".to_owned(), |result| {
			                                match result
			                                {
				                                Ok(_) => "Success:- New user registered".to_string(),
			                                    Err(err) => err.to_string()
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
			<ActionForm action=register_user_action>

				<div>
					<label class="input-label" for="username">
						"Username"
					</label>
				</div>
				<div>
					<input class="input-fields" type="text" name="username" id="username" required/>
				</div>

				<div class="mt-3">
					<label class="input-label" for="first_name">
						"First Name"
					</label>
				</div>
				<div>
					<input
						class="input-fields"
						type="text"
						name="first_name"
						id="first_name"
						required
					/>
				</div>

				<div class="mt-3">
					<label class="input-label" for="last_name">
						"Last Name"
					</label>
				</div>
				<div>
					<input
						class="input-fields"
						type="text"
						name="last_name"
						id="last_name"
						required
					/>
				</div>

				<div class="mt-3">
					<label class="input-label" for="date_of_birth">
						"Date of Birth"
					</label>
				</div>
				<div>
					<input
						class="input-fields"
						type="date"
						name="date_of_birth"
						id="date_of_birth"
						required
					/>
				</div>

				<div class="mt-3">
					<label class="input-label" for="email">
						"Email"
					</label>
				</div>
				<div>
					<input class="input-fields" type="email" name="email" id="email" required/>
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
						"Register"
					</button>
				</div>
			</ActionForm>
		</div>
	}
}
