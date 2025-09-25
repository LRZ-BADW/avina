use std::str::FromStr;

use avina::{Api, Token};
use avina_test::{random_uuid, spawn_app};
use uuid::Uuid;

use super::assert_contains_server_state;

#[tokio::test]
async fn e2e_lib_normal_user_can_list_own_server_states() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(0, 0, 2)
        .await
        .expect("Failed to setup test project");
    let normal_user_1 = test_project.normals[0].user.clone();
    let token = test_project.normals[0].token.clone();
    let normal_user_2 = test_project.normals[1].user.clone();
    server
        .mock_keystone_auth(
            &token,
            &normal_user_1.openstack_id,
            &normal_user_1.name,
        )
        .mount(&server.keystone_server)
        .await;
    let flavor = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let server_id = Uuid::new_v4();
    let server_state_1 = server
        .setup_test_server_state_with_server_id(
            &flavor,
            &normal_user_1,
            server_id,
        )
        .await
        .expect("Failed to setup test server state 1");
    let server_state_2 = server
        .setup_test_server_state_with_server_id(
            &flavor,
            &normal_user_1,
            server_id,
        )
        .await
        .expect("Failed to setup test server state 2");
    let server_state_3 = server
        .setup_test_server_state(&flavor, &normal_user_1)
        .await
        .expect("Failed to setup test server state 3");
    let _server_state_4 = server
        .setup_test_server_state(&flavor, &normal_user_2)
        .await
        .expect("Failed to setup test server state 4");

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let server_states_1 = client.server_state.list().send().await.unwrap();
    let server_states_2 = client
        .server_state
        .list()
        .user(normal_user_1.id)
        .send()
        .await
        .unwrap();
    let server_states_3 = client
        .server_state
        .list()
        .server(server_id)
        .send()
        .await
        .unwrap();

    // assert
    assert_eq!(server_states_1.len(), 3);
    assert_contains_server_state(&server_states_1, &server_state_1);
    assert_contains_server_state(&server_states_1, &server_state_2);
    assert_contains_server_state(&server_states_1, &server_state_3);
    assert_eq!(server_states_2.len(), 3);
    assert_contains_server_state(&server_states_2, &server_state_1);
    assert_contains_server_state(&server_states_2, &server_state_2);
    assert_contains_server_state(&server_states_2, &server_state_3);
    assert_eq!(server_states_3.len(), 2);
    assert_contains_server_state(&server_states_3, &server_state_1);
    assert_contains_server_state(&server_states_3, &server_state_2);
}

#[tokio::test]
async fn e2e_lib_normal_user_cannot_use_other_server_state_list_filters() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(0, 0, 2)
        .await
        .expect("Failed to setup test project");
    let normal_user_1 = test_project.normals[0].user.clone();
    let project = test_project.project.clone();
    let token = test_project.normals[0].token.clone();
    let normal_user_2 = test_project.normals[1].user.clone();
    server
        .mock_keystone_auth(
            &token,
            &normal_user_1.openstack_id,
            &normal_user_1.name,
        )
        .mount(&server.keystone_server)
        .await;
    let flavor = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let server_id = Uuid::new_v4();
    let _server_state = server
        .setup_test_server_state_with_server_id(
            &flavor,
            &normal_user_2,
            server_id,
        )
        .await
        .expect("Failed to setup test server state 2");

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let server_states_1 = client.server_state.list().all().send().await;
    let server_states_2 =
        client.server_state.list().project(project.id).send().await;
    let server_states_3 = client
        .server_state
        .list()
        .user(normal_user_2.id)
        .send()
        .await;
    let server_states_4 =
        client.server_state.list().server(server_id).send().await;

    // assert
    assert!(server_states_1.is_err());
    assert!(server_states_2.is_err());
    assert_eq!(
        server_states_1.unwrap_err().to_string(),
        format!("Admin privileges required")
    );
    assert_eq!(
        server_states_2.unwrap_err().to_string(),
        "Resource not found".to_string()
    );
    assert!(server_states_3.is_err());
    assert_eq!(
        server_states_3.unwrap_err().to_string(),
        "Resource not found".to_string()
    );
    assert!(server_states_4.is_err());
    assert_eq!(
        server_states_4.unwrap_err().to_string(),
        "Resource not found".to_string()
    );
}

#[tokio::test]
async fn e2e_lib_master_user_cannot_use_other_server_state_all_filter() {
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

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let list = client.server_state.list().all().send().await;

    // assert
    assert!(list.is_err());
    assert_eq!(
        list.unwrap_err().to_string(),
        format!("Admin privileges required")
    );
}

#[tokio::test]
async fn e2e_lib_master_user_can_list_own_projects_and_users_server_states() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(0, 1, 1)
        .await
        .expect("Failed to setup test project");
    let master_user = test_project.masters[0].user.clone();
    let token = test_project.masters[0].token.clone();
    let normal_user_1 = test_project.normals[0].user.clone();
    let project = test_project.project.clone();
    let test_project2 = server
        .setup_test_project(0, 0, 1)
        .await
        .expect("Failed to setup test project");
    let normal_user_2 = test_project2.normals[0].user.clone();
    server
        .mock_keystone_auth(
            &token,
            &master_user.openstack_id,
            &master_user.name,
        )
        .mount(&server.keystone_server)
        .await;
    let flavor = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let server_id = Uuid::new_v4();
    let server_state_1 = server
        .setup_test_server_state_with_server_id(
            &flavor,
            &normal_user_1,
            server_id,
        )
        .await
        .expect("Failed to setup test server state 1");
    let server_state_2 = server
        .setup_test_server_state_with_server_id(
            &flavor,
            &normal_user_1,
            server_id,
        )
        .await
        .expect("Failed to setup test server state 2");

    let server_state_3 = server
        .setup_test_server_state(&flavor, &master_user)
        .await
        .expect("Failed to setup test server state 3");
    let server_state_4 = server
        .setup_test_server_state(&flavor, &normal_user_1)
        .await
        .expect("Failed to setup test server state 4");
    let _server_state_5 = server
        .setup_test_server_state(&flavor, &normal_user_2)
        .await
        .expect("Failed to setup test server state 5");

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let server_states_1 = client.server_state.list().send().await.unwrap();
    let server_states_2 = client
        .server_state
        .list()
        .user(normal_user_1.id)
        .send()
        .await
        .unwrap();
    let server_states_3 = client
        .server_state
        .list()
        .project(project.id)
        .send()
        .await
        .unwrap();
    let server_states_4 = client
        .server_state
        .list()
        .server(server_id)
        .send()
        .await
        .unwrap();

    // assert
    assert_eq!(server_states_1.len(), 1);
    assert_contains_server_state(&server_states_1, &server_state_3);
    assert_eq!(server_states_2.len(), 3);
    assert_contains_server_state(&server_states_2, &server_state_1);
    assert_contains_server_state(&server_states_2, &server_state_2);
    assert_eq!(server_states_3.len(), 4);
    assert_contains_server_state(&server_states_3, &server_state_1);
    assert_contains_server_state(&server_states_3, &server_state_2);
    assert_contains_server_state(&server_states_3, &server_state_3);
    assert_contains_server_state(&server_states_3, &server_state_4);
    assert_eq!(server_states_4.len(), 2);
    assert_contains_server_state(&server_states_4, &server_state_1);
    assert_contains_server_state(&server_states_4, &server_state_2);
}

#[tokio::test]
async fn e2e_lib_master_user_cannot_list_other_projects_and_users_server_states()
 {
    // arrange
    let server = spawn_app().await;
    let test_project_1 = server
        .setup_test_project(0, 1, 0)
        .await
        .expect("Failed to setup test project");
    let master_user = test_project_1.masters[0].user.clone();
    let token = test_project_1.masters[0].token.clone();
    let test_project_2 = server
        .setup_test_project(0, 0, 1)
        .await
        .expect("Failed to setup test project");
    let normal_user = test_project_2.normals[0].user.clone();
    server
        .mock_keystone_auth(
            &token,
            &master_user.openstack_id,
            &master_user.name,
        )
        .mount(&server.keystone_server)
        .await;
    let flavor = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let server_id = Uuid::new_v4();
    let _server_state = server
        .setup_test_server_state_with_server_id(
            &flavor,
            &normal_user,
            server_id,
        )
        .await
        .expect("Failed to setup test server state 1");

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let server_states_1 =
        client.server_state.list().user(normal_user.id).send().await;
    let server_states_2 = client
        .server_state
        .list()
        .project(test_project_2.project.id)
        .send()
        .await;
    let server_states_3 =
        client.server_state.list().server(server_id).send().await;

    // assert
    assert!(server_states_1.is_err());
    assert_eq!(
        server_states_1.unwrap_err().to_string(),
        "Resource not found".to_string()
    );
    assert!(server_states_2.is_err());
    assert_eq!(
        server_states_2.unwrap_err().to_string(),
        "Resource not found".to_string()
    );
    assert!(server_states_3.is_err());
    assert_eq!(
        server_states_3.unwrap_err().to_string(),
        "Resource not found".to_string()
    );
}

#[tokio::test]
async fn e2e_lib_server_state_list_server_filter_works_across_projects_for_admin_user()
 {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(1, 0, 1)
        .await
        .expect("Failed to setup test project");
    let user = test_project.admins[0].user.clone();
    let token = test_project.admins[0].token.clone();
    let user2 = test_project.normals[0].user.clone();
    let test_project2 = server
        .setup_test_project(0, 0, 1)
        .await
        .expect("Failed to setup test project");
    let user3 = test_project2.normals[0].user.clone();
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let flavor = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let server_id = Uuid::new_v4();
    let server_state1 = server
        .setup_test_server_state_with_server_id(&flavor, &user, server_id)
        .await
        .expect("Failed to setup test server state 1");
    let server_state2 = server
        .setup_test_server_state_with_server_id(&flavor, &user2, server_id)
        .await
        .expect("Failed to setup test server state 2");
    let server_state3 = server
        .setup_test_server_state_with_server_id(&flavor, &user3, server_id)
        .await
        .expect("Failed to setup test server state 3");
    let _server_state4 = server
        .setup_test_server_state(&flavor, &user)
        .await
        .expect("Failed to setup test server state 4");
    let _server_state5 = server
        .setup_test_server_state(&flavor, &user2)
        .await
        .expect("Failed to setup test server state 5");
    let _server_state6 = server
        .setup_test_server_state(&flavor, &user3)
        .await
        .expect("Failed to setup test server state 6");

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let server_states = client
        .server_state
        .list()
        .server(server_id)
        .send()
        .await
        .unwrap();

    // assert
    assert_eq!(server_states.len(), 3);
    assert_contains_server_state(&server_states, &server_state1);
    assert_contains_server_state(&server_states, &server_state2);
    assert_contains_server_state(&server_states, &server_state3);
}

#[tokio::test]
async fn e2e_lib_admin_user_can_use_any_user_list_filters() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(1, 0, 1)
        .await
        .expect("Failed to setup test project");
    let user = test_project.admins[0].user.clone();
    let token = test_project.admins[0].token.clone();
    let user2 = test_project.normals[0].user.clone();
    let project = test_project.project.clone();
    let test_project2 = server
        .setup_test_project(0, 1, 1)
        .await
        .expect("Failed to setup test project");
    let user3 = test_project2.masters[0].user.clone();
    let user4 = test_project2.normals[0].user.clone();
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let flavor = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let server_state1 = server
        .setup_test_server_state(&flavor, &user)
        .await
        .expect("Failed to setup test server state 1");
    let server_state2 = server
        .setup_test_server_state(&flavor, &user2)
        .await
        .expect("Failed to setup test server state 2");
    let server_state3 = server
        .setup_test_server_state(&flavor, &user3)
        .await
        .expect("Failed to setup test server state 3");
    let server_state4 = server
        .setup_test_server_state(&flavor, &user4)
        .await
        .expect("Failed to setup test server state 4");
    let server_state5 = server
        .setup_test_server_state(&flavor, &user4)
        .await
        .expect("Failed to setup test server state 5");

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let server_states1 = client.server_state.list().send().await.unwrap();
    let server_states2 = client
        .server_state
        .list()
        .server(server_state5.instance_id)
        .send()
        .await
        .unwrap();
    let server_states3 = client
        .server_state
        .list()
        .user(user2.id)
        .send()
        .await
        .unwrap();
    let server_states4 = client
        .server_state
        .list()
        .project(project.id)
        .send()
        .await
        .unwrap();
    let server_states5 = client.server_state.list().all().send().await.unwrap();

    // assert
    assert_eq!(server_states1.len(), 1);
    assert_contains_server_state(&server_states1, &server_state1);
    assert_eq!(server_states2.len(), 1);
    assert_contains_server_state(&server_states2, &server_state5);
    assert_eq!(server_states3.len(), 1);
    assert_contains_server_state(&server_states3, &server_state2);
    assert_eq!(server_states4.len(), 2);
    assert_contains_server_state(&server_states4, &server_state1);
    assert_contains_server_state(&server_states4, &server_state2);
    assert_eq!(server_states5.len(), 5);
    assert_contains_server_state(&server_states5, &server_state1);
    assert_contains_server_state(&server_states5, &server_state2);
    assert_contains_server_state(&server_states5, &server_state3);
    assert_contains_server_state(&server_states5, &server_state4);
    assert_contains_server_state(&server_states5, &server_state5);
}

#[tokio::test]
async fn e2e_lib_master_user_can_combine_server_state_list_filters() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(0, 1, 1)
        .await
        .expect("Failed to setup test project");
    let user = test_project.masters[0].user.clone();
    let token = test_project.masters[0].token.clone();
    let project = test_project.project.clone();
    let user2 = test_project.normals[0].user.clone();
    let test_project2 = server
        .setup_test_project(0, 0, 1)
        .await
        .expect("Failed to setup test project");
    let user3 = test_project2.normals[0].user.clone();
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let flavor = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let server_id = Uuid::new_v4();
    let server_state1 = server
        .setup_test_server_state_with_server_id(&flavor, &user, server_id)
        .await
        .expect("Failed to setup test server state 1");
    let server_state2 = server
        .setup_test_server_state_with_server_id(&flavor, &user2, server_id)
        .await
        .expect("Failed to setup test server state 2");
    let _server_state3 = server
        .setup_test_server_state_with_server_id(&flavor, &user3, server_id)
        .await
        .expect("Failed to setup test server state 3");
    let _server_state4 = server
        .setup_test_server_state(&flavor, &user)
        .await
        .expect("Failed to setup test server state 4");
    let _server_state5 = server
        .setup_test_server_state(&flavor, &user2)
        .await
        .expect("Failed to setup test server state 5");
    let _server_state6 = server
        .setup_test_server_state(&flavor, &user3)
        .await
        .expect("Failed to setup test server state 6");

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let server_states1 = client
        .server_state
        .list()
        .project(project.id)
        .server(server_id)
        .send()
        .await
        .unwrap();
    let server_states2 = client
        .server_state
        .list()
        .user(user.id)
        .server(server_id)
        .send()
        .await
        .unwrap();

    // assert
    assert_eq!(server_states1.len(), 2);
    assert_contains_server_state(&server_states1, &server_state1);
    assert_contains_server_state(&server_states1, &server_state2);
    assert_eq!(server_states2.len(), 1);
    assert_contains_server_state(&server_states2, &server_state1);
}

#[tokio::test]
async fn e2e_lib_admin_user_can_combine_server_state_list_filters() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(1, 0, 1)
        .await
        .expect("Failed to setup test project");
    let user = test_project.admins[0].user.clone();
    let token = test_project.admins[0].token.clone();
    let project = test_project.project.clone();
    let user2 = test_project.normals[0].user.clone();
    let test_project2 = server
        .setup_test_project(0, 0, 1)
        .await
        .expect("Failed to setup test project");
    let user3 = test_project2.normals[0].user.clone();
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let flavor = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let server_id = Uuid::new_v4();
    let server_state1 = server
        .setup_test_server_state_with_server_id(&flavor, &user, server_id)
        .await
        .expect("Failed to setup test server state 1");
    let server_state2 = server
        .setup_test_server_state_with_server_id(&flavor, &user2, server_id)
        .await
        .expect("Failed to setup test server state 2");
    let _server_state3 = server
        .setup_test_server_state_with_server_id(&flavor, &user3, server_id)
        .await
        .expect("Failed to setup test server state 3");
    let _server_state4 = server
        .setup_test_server_state(&flavor, &user)
        .await
        .expect("Failed to setup test server state 4");
    let _server_state5 = server
        .setup_test_server_state(&flavor, &user2)
        .await
        .expect("Failed to setup test server state 5");
    let _server_state6 = server
        .setup_test_server_state(&flavor, &user3)
        .await
        .expect("Failed to setup test server state 6");

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let server_states1 = client
        .server_state
        .list()
        .project(project.id)
        .server(server_id)
        .send()
        .await
        .unwrap();
    let server_states2 = client
        .server_state
        .list()
        .user(user.id)
        .server(server_id)
        .send()
        .await
        .unwrap();

    // assert
    assert_eq!(server_states1.len(), 2);
    assert_contains_server_state(&server_states1, &server_state1);
    assert_contains_server_state(&server_states1, &server_state2);
    assert_eq!(server_states2.len(), 1);
    assert_contains_server_state(&server_states2, &server_state1);
}
