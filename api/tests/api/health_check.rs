use avina_test::{random_uuid, spawn_app};

#[tokio::test]
async fn health_check_works() {
    // arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // act
    let response = client
        .get(format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn secured_health_check_returns_unauthorized_for_missing_token() {
    // arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let token = random_uuid();
    app.mock_keystone_auth(&token, "project_id", "project_name")
        .mount(&app.keystone_server)
        .await;

    // act
    let response = client
        .get(format!("{}/api/secured_health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn secured_health_check_returns_unauthorized_for_wrong_token() {
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
        .get(format!("{}/api/secured_health_check", &app.address))
        .header("X-Auth-Token", wrong_token)
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn secured_health_check_works_with_valid_token() {
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
        .get(format!("{}/api/secured_health_check", &app.address))
        .header("X-Auth-Token", token)
        .send()
        .await
        .expect("Failed to execute request.");

    // assert
    assert_eq!(response.status().as_u16(), 200);
}
