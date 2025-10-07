use avina_test::{random_uuid, spawn_app};
use avina_wire::hello::Hello;

#[tokio::test]
async fn hello_admin_returns_unauthorized_for_missing_token() {
    // arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let token = random_uuid();
    app.mock_keystone_auth(&token, "project_id", "project_name")
        .mount(&app.keystone_server)
        .await;

    // act
    let response = client
        .get(format!("{}/api/hello/admin", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn hello_admin_returns_unauthorized_for_wrong_token() {
    // arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let token = random_uuid();
    app.mock_keystone_auth(&token, "project_id", "project_name")
        .mount(&app.keystone_server)
        .await;

    // act
    let wrong_token = random_uuid();
    let response = client
        .get(format!("{}/api/hello/admin", &app.address))
        .header("X-Auth-Token", wrong_token)
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn hello_admin_denies_access_to_normal_user() {
    // arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let (user, _project, token) = app
        .setup_test_user_and_project(false)
        .await
        .expect("Failed to setup test user and project.");
    app.mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&app.keystone_server)
        .await;

    // act
    let response = client
        .get(format!("{}/api/hello/admin", &app.address))
        .header("X-Auth-Token", token)
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert_eq!(response.status().as_u16(), 403);
}

#[tokio::test]
async fn hello_admin_works_for_admin_user() {
    // arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let (user, project, token) = app
        .setup_test_user_and_project(true)
        .await
        .expect("Failed to setup test user and project.");
    app.mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&app.keystone_server)
        .await;

    // act
    let response = client
        .get(format!("{}/api/hello/admin", &app.address))
        .header("X-Auth-Token", token)
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(
        response.headers().get("Content-Type").unwrap(),
        "application/json"
    );
    let hello =
        serde_json::from_str::<Hello>(&response.text().await.unwrap()).unwrap();
    assert_eq!(
        hello.message,
        format!(
            "Hello, admin {} from project {} with user class {}",
            user.name, project.name, project.user_class
        )
    );
}
