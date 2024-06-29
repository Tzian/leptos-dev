use super::*;

#[island]
pub fn Sidebar() -> impl IntoView
{
	view! {
		<div class="sidebar text-center border-2 border-secondary-400 p-3 mt-3">
			<div>
				<a href="/employees" class="std-btn">
					"Employees"
				</a>
			</div>
			<div class="mt-5">
				<a href="/clients" class="std-btn">
					"Clients"
				</a>
			</div>
			<div class="mt-5">
				<a href="/jobs" class="std-btn">
					"Jobs"
				</a>
			</div>
			<div class="mt-5">
				<a href="/calendar" class="std-btn">
					"Calendar"
				</a>
			</div>
			<div class="mt-5">
				<a href="/reports" class="std-btn">
					"Reports"
				</a>
			</div>
			<div class="mt-5">
				<a href="/settings" class="std-btn">
					"Settings"
				</a>
			</div>
		</div>
	}
}
