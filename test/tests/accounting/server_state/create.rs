use std::str::FromStr;

use avina::{Api, Token};
use avina_test::{random_alphanumeric_string, random_uuid, spawn_app};
use chrono::{DateTime, FixedOffset, Utc};
use uuid::Uuid;

use super::assert_equal_server_states;

#[tokio::test]
async fn e2e_lib_server_state_create_denies_access_to_normal_user() {
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

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let begin = DateTime::<FixedOffset>::from(Utc::now());
    let instance_id = Uuid::new_v4();
    let instance_name = random_alphanumeric_string(10);
    let status = "ACTIVE".to_string();
    let create = client
        .server_state
        .create(
            begin,
            instance_id,
            instance_name,
            flavor.id,
            status,
            user.id,
        )
        .send()
        .await;

    // assert
    assert!(create.is_err());
    assert_eq!(
        create.unwrap_err().to_string(),
        format!("Admin privileges required")
    );
}

#[tokio::test]
async fn e2e_lib_server_state_create_denies_access_to_master_user() {
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

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let begin = DateTime::<FixedOffset>::from(Utc::now());
    let instance_id = Uuid::new_v4();
    let instance_name = random_alphanumeric_string(10);
    let status = "ACTIVE".to_string();
    let create = client
        .server_state
        .create(
            begin,
            instance_id,
            instance_name,
            flavor.id,
            status,
            user.id,
        )
        .send()
        .await;

    // assert
    assert!(create.is_err());
    assert_eq!(
        create.unwrap_err().to_string(),
        format!("Admin privileges required")
    );
}

#[tokio::test]
async fn e2e_lib_server_state_create_works() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(1, 0, 0)
        .await
        .expect("Failed to setup test project");
    let user = test_project.admins[0].user.clone();
    let token = test_project.admins[0].token.clone();
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let flavor = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let begin = DateTime::<FixedOffset>::from(Utc::now());
    let instance_id = Uuid::new_v4();
    let instance_name = random_alphanumeric_string(10);
    let status = "ACTIVE".to_string();
    let created = client
        .server_state
        .create(
            begin,
            instance_id.clone(),
            instance_name.clone(),
            flavor.id,
            status.clone(),
            user.id,
        )
        .send()
        .await
        .unwrap();

    // assert
    assert_eq!(begin, created.begin);
    assert_eq!(instance_id, created.instance_id);
    assert_eq!(instance_name, created.instance_name);
    assert_eq!(flavor.id, created.flavor);
    assert_eq!(flavor.name, created.flavor_name);
    assert_eq!(status, created.status);
    assert_eq!(user.id, created.user);
    assert_eq!(user.name, created.username);
}

#[tokio::test]
async fn e2e_lib_server_state_create_and_get_works() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(1, 0, 0)
        .await
        .expect("Failed to setup test project");
    let user = test_project.admins[0].user.clone();
    let token = test_project.admins[0].token.clone();
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let flavor = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act and assert 1 - create
    let begin = DateTime::<FixedOffset>::from(Utc::now());
    let instance_id = Uuid::new_v4();
    let instance_name = random_alphanumeric_string(10);
    let status = "ACTIVE".to_string();
    let created = client
        .server_state
        .create(
            begin,
            instance_id,
            instance_name.clone(),
            flavor.id,
            status.clone(),
            user.id,
        )
        .send()
        .await
        .unwrap();
    assert_eq!(begin, created.begin);
    assert_eq!(instance_id, created.instance_id);
    assert_eq!(instance_name, created.instance_name);
    assert_eq!(flavor.id, created.flavor);
    assert_eq!(flavor.name, created.flavor_name);
    assert_eq!(status, created.status);
    assert_eq!(user.id, created.user);
    assert_eq!(user.name, created.username);

    // act and assert 2 - get
    let retrieved = client.server_state.get(created.id).await.unwrap();
    assert_equal_server_states(&retrieved, &created);
}

#[tokio::test]
async fn e2e_lib_server_state_create_and_list_works() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(1, 0, 0)
        .await
        .expect("Failed to setup test project");
    let user = test_project.admins[0].user.clone();
    let token = test_project.admins[0].token.clone();
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let flavor = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act and assert 1 - create
    let begin = DateTime::<FixedOffset>::from(Utc::now());
    let instance_id = Uuid::new_v4();
    let instance_name = random_alphanumeric_string(10);
    let status = "ACTIVE".to_string();
    let created = client
        .server_state
        .create(
            begin,
            instance_id.clone(),
            instance_name.clone(),
            flavor.id,
            status.clone(),
            user.id,
        )
        .send()
        .await
        .unwrap();
    assert_eq!(begin, created.begin);
    assert_eq!(instance_id, created.instance_id);
    assert_eq!(instance_name, created.instance_name);
    assert_eq!(flavor.id, created.flavor);
    assert_eq!(flavor.name, created.flavor_name);
    assert_eq!(status, created.status);
    assert_eq!(user.id, created.user);
    assert_eq!(user.name, created.username);

    // act and assert 2 - list
    let server_states = client.server_state.list().all().send().await.unwrap();
    assert_eq!(server_states.len(), 1);
    assert_equal_server_states(&server_states[0], &created);
}
