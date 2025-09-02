use std::str::FromStr;

use avina::{Api, Token};
use avina_test::{random_alphanumeric_string, random_uuid, spawn_app};
use avina_wire::user::UserClass;

#[tokio::test]
async fn e2e_lib_project_list_returns_own_project() {
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
    let projects = client.project.list().send().await.unwrap();

    // assert
    assert_eq!(projects.len(), 1);
    assert!(projects.contains(&project));
}

#[tokio::test]
async fn e2e_lib_project_list_all_denies_access_to_normal_user() {
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
    let list = client.project.list().all().send().await;

    // assert
    assert!(list.is_err());
    assert_eq!(
        list.unwrap_err().to_string(),
        format!("Admin privileges required")
    );
}

#[tokio::test]
async fn e2e_lib_project_list_by_user_class_denies_access_to_normal_user() {
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
    let list = client
        .project
        .list()
        .user_class(UserClass::UC1)
        .send()
        .await;

    // assert
    assert!(list.is_err());
    assert_eq!(
        list.unwrap_err().to_string(),
        format!("Admin privileges required")
    );
}

#[tokio::test]
async fn e2e_lib_project_list_all_works() {
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

    // act part 1 - create projects
    let mut expected = Vec::new();
    expected.push(project);
    for _ in 0..5 {
        let name = random_alphanumeric_string(10);
        let openstack_id = random_uuid();
        let user_class = rand::random();
        let created = client
            .project
            .create(name.clone(), openstack_id.clone())
            .user_class(user_class)
            .send()
            .await
            .unwrap();
        expected.push(created);
    }

    // act part 2 - list all projects
    let projects = client.project.list().all().send().await.unwrap();

    // assert
    assert_eq!(projects, expected);
}

#[tokio::test]
async fn e2e_lib_project_list_by_user_class_works() {
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

    let user_class = UserClass::UC3;

    // act part 1 - create projects
    let mut expected = Vec::new();
    if project.user_class == user_class {
        expected.push(project);
    }
    for _ in 0..=6 {
        let name = random_alphanumeric_string(10);
        let openstack_id = random_uuid();
        let created = client
            .project
            .create(name.clone(), openstack_id.clone())
            .user_class(user_class)
            .send()
            .await
            .unwrap();
        if created.user_class == user_class {
            expected.push(created);
        }
    }

    // act part 2 - list all projects
    let projects = client
        .project
        .list()
        .user_class(user_class)
        .send()
        .await
        .unwrap();

    // assert
    assert_eq!(projects, expected);
}
