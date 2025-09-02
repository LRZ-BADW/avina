use std::str::FromStr;

use avina::{Api, Token};
use avina_test::{random_alphanumeric_string, random_uuid, spawn_app};
use avina_wire::user::ProjectRetrieved;

#[tokio::test]
async fn e2e_lib_project_create_denies_access_to_normal_user() {
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
    let name = random_alphanumeric_string(10);
    let openstack_id = random_uuid();
    let create = client.project.create(name, openstack_id).send().await;

    // assert
    assert!(create.is_err());
    assert_eq!(
        create.unwrap_err().to_string(),
        format!("Admin privileges required")
    );
}

#[tokio::test]
async fn e2e_lib_project_create_denies_access_to_master_user() {
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

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let name = random_alphanumeric_string(10);
    let openstack_id = random_uuid();
    let create = client.project.create(name, openstack_id).send().await;

    // assert
    assert!(create.is_err());
    assert_eq!(
        create.unwrap_err().to_string(),
        format!("Admin privileges required")
    );
}

#[tokio::test]
async fn e2e_lib_project_create_works() {
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

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
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

    // assert
    assert_eq!(name, created.name);
    assert_eq!(openstack_id, created.openstack_id);
    assert_eq!(user_class, created.user_class);
}

// TODO: how can we test internal server error responses?
#[tokio::test]
async fn e2e_lib_project_create_twice_returns_bad_request() {
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

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act and assert 1 - create
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
    assert_eq!(name, created.name);
    assert_eq!(openstack_id, created.openstack_id);
    assert_eq!(user_class, created.user_class);

    // act and assert 2 - create
    let create = client
        .project
        .create(name.clone(), openstack_id.clone())
        .user_class(user_class)
        .send()
        .await;
    match create {
        Err(avina::error::ApiError::ResponseError(message)) => {
            assert_eq!(
                message,
                "Failed to insert new project, a conflicting entry exists"
                    .to_string()
            );
        }
        _ => panic!("Expected ApiError::ResponseError"),
    }
}

#[tokio::test]
async fn e2e_lib_project_create_and_get_works() {
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

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act and assert 1 - create
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
    assert_eq!(name, created.name);
    assert_eq!(openstack_id, created.openstack_id);
    assert_eq!(user_class, created.user_class);

    // act and assert 2 - get
    let ProjectRetrieved::Detailed(detailed) =
        client.project.get(created.id).await.unwrap()
    else {
        panic!("Expected ProjectDetailed")
    };
    assert_eq!(detailed, created);
    assert_eq!(detailed.users.len(), 0);
    assert_eq!(detailed.flavor_groups.len(), 0);
}

#[tokio::test]
async fn e2e_lib_project_create_and_list_works() {
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

    // act and assert 1 - create
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
    assert_eq!(name, created.name);
    assert_eq!(openstack_id, created.openstack_id);
    assert_eq!(user_class, created.user_class);

    // act and assert 2 - list
    let projects = client.project.list().all().send().await.unwrap();
    assert_eq!(projects.len(), 2);
    assert_eq!(project, projects[0]);
    assert_eq!(created, projects[1]);
}
