use dioxus::prelude::*;

#[component]
pub fn ProfilePage(api_url: String, token: String) -> Element {
    let user = api_call!(api_url, token, api, api.user.me().await);
    let role = if user.is_staff {
        "Administrator"
    } else if user.role == 2 {
        "Master User"
    } else if user.role == 1 {
        "User"
    } else {
        "Unknown"
    };
    rsx! {
        h2 { "Profile" }
        hr {}

        h3 { "User" }
        table {
            tr { td { b { "ID:" } }, td { "{user.id}" } }
            tr { td { b { "Name:" } }, td { "{user.name}" } }
            tr { td { b { "UUID:" } }, td { "{user.openstack_id}" } }
            tr { td { b { "Role:" } }, td { "{role}" } }
        }
        br {}

        h3 { "Project" }
        table {
            tr { td { b { "ID:" } }, td { "{user.project.id}" } }
            tr { td { b { "Name:" } }, td { "{user.project.name}" } }
            tr { td { b { "User Class:" } }, td { "{user.project.user_class}" } }
        }
        br {}

    }
}
