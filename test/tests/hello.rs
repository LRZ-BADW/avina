use std::str::FromStr;

use avina::{Api, Token};
use avina_test::spawn_app;

#[tokio::test]
async fn e2e_lib_hello_user_works() {
    // arrange
    let server = spawn_app().await;
    let (user, project, token) = server
        .setup_test_user_and_project(false)
        .await
        .expect("Failed to setup test user and project.");
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let hello = client.hello.user().await.unwrap();

    // assert
    assert_eq!(
        hello.message,
        format!(
            "Hello, {} from project {} with user class {}",
            user.name, project.name, project.user_class
        )
    );
}

#[tokio::test]
async fn e2e_lib_hello_admin_denies_access_to_normal_user() {
    // arrange
    let server = spawn_app().await;
    let (user, _project, token) = server
        .setup_test_user_and_project(false)
        .await
        .expect("Failed to setup test user and project.");
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let hello = client.hello.admin().await;

    // assert
    assert!(hello.is_err());
    assert_eq!(
        hello.unwrap_err().to_string(),
        format!("Admin privileges required")
    );
}

#[tokio::test]
async fn e2e_lib_hello_admin_works() {
    // arrange
    let server = spawn_app().await;
    let (user, project, token) = server
        .setup_test_user_and_project(true)
        .await
        .expect("Failed to setup test user and project.");
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let hello = client.hello.admin().await.unwrap();

    // assert
    assert_eq!(
        hello.message,
        format!(
            "Hello, admin {} from project {} with user class {}",
            user.name, project.name, project.user_class
        )
    );
}
