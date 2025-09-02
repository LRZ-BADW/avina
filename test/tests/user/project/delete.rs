use std::str::FromStr;

use avina::{Api, Token};
use avina_test::{random_alphanumeric_string, random_uuid, spawn_app};
use avina_wire::user::ProjectRetrieved;

#[tokio::test]
async fn e2e_lib_project_delete_denies_access_to_normal_user() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(0, 0, 1)
        .await
        .expect("Failed to setup test project");
    let user = test_project.normals[0].user.clone();
    let token = test_project.normals[0].token.clone();
    let project = test_project.project.clone();
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
    let delete = client.project.delete(project.id).await;

    // assert
    assert!(delete.is_err());
    assert_eq!(
        delete.unwrap_err().to_string(),
        format!("Admin privileges required")
    );
}

#[tokio::test]
async fn e2e_lib_project_delete_denies_access_to_master_user() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(0, 1, 0)
        .await
        .expect("Failed to setup test project");
    let user = test_project.masters[0].user.clone();
    let token = test_project.masters[0].token.clone();
    let project = test_project.project.clone();
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
    let delete = client.project.delete(project.id).await;

    // assert
    assert!(delete.is_err());
    assert_eq!(
        delete.unwrap_err().to_string(),
        format!("Admin privileges required")
    );
}

#[tokio::test]
async fn e2e_lib_project_create_get_delete_get_works() {
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

    // act and assert 3 - delete
    client.project.delete(created.id).await.unwrap();

    // act and assert 4 - get
    let get = client.project.get(created.id).await;
    assert!(get.is_err());
    assert_eq!(get.unwrap_err().to_string(), format!("Resource not found"));
}

// TODO: test what happens when deleting non-empty project
