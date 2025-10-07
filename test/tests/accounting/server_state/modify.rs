use std::str::FromStr;

use avina::{Api, Token};
use avina_test::{random_alphanumeric_string, spawn_app};
use uuid::Uuid;

use super::assert_equal_server_states;

#[tokio::test]
async fn e2e_lib_server_state_modify_denies_access_to_normal_user() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(0, 0, 1)
        .await
        .expect("Failed to setup test project");
    let user = test_project.normals[0].user.clone();
    let token = test_project.normals[0].token.clone();
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let flavor = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let server_state = server
        .setup_test_server_state(&flavor, &user)
        .await
        .expect("Failed to setup test server state");

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let modify = client.server_state.modify(server_state.id).send().await;

    // assert
    assert!(modify.is_err());
    assert_eq!(
        modify.unwrap_err().to_string(),
        format!("Admin privileges required")
    );
}

#[tokio::test]
async fn e2e_lib_server_state_modify_denies_access_to_master_user() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(0, 1, 0)
        .await
        .expect("Failed to setup test project");
    let user = test_project.masters[0].user.clone();
    let token = test_project.masters[0].token.clone();
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let flavor = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let server_state = server
        .setup_test_server_state(&flavor, &user)
        .await
        .expect("Failed to setup test server state");

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let modify = client.server_state.modify(server_state.id).send().await;

    // assert
    assert!(modify.is_err());
    assert_eq!(
        modify.unwrap_err().to_string(),
        format!("Admin privileges required")
    );
}

#[tokio::test]
async fn e2e_lib_server_state_modify_and_get_works() {
    // arrange
    let server = spawn_app().await;
    let (user, _project, token) = server
        .setup_test_user_and_project(true)
        .await
        .expect("Failed to setup test user and project.");
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let flavor = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let server_state = server
        .setup_test_server_state(&flavor, &user)
        .await
        .expect("Failed to setup test server state");

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act and assert 1 - modify
    let instance_id = Uuid::new_v4();
    let instance_name = random_alphanumeric_string(10);
    let modified = client
        .server_state
        .modify(server_state.id)
        .instance_id(instance_id)
        .instance_name(instance_name.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(instance_id, modified.instance_id);
    assert_eq!(instance_name, modified.instance_name);

    // act and assert 2 - get
    let retrieved = client.server_state.get(modified.id).await.unwrap();
    assert_equal_server_states(&modified, &retrieved);
}
