use std::str::FromStr;

use avina::{Api, Token};
use avina_test::spawn_app;

#[tokio::test]
async fn e2e_lib_project_budget_delete_denies_access_to_normal_user() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(0, 0, 1)
        .await
        .expect("Failed to setup test project");
    let user = test_project.normals[0].user.clone();
    let token = test_project.normals[0].token.clone();
    let project = test_project.project;
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let project_budget = server
        .setup_test_project_budget(&project)
        .await
        .expect("Failed to setup test user budget");

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let delete = client.project_budget.delete(project_budget.id).await;

    // assert
    assert!(delete.is_err());
    assert_eq!(
        delete.unwrap_err().to_string(),
        format!("Admin privileges required")
    );
}

#[tokio::test]
async fn e2e_lib_project_budget_delete_denies_access_to_master_user() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(0, 1, 0)
        .await
        .expect("Failed to setup test project");
    let user = test_project.masters[0].user.clone();
    let token = test_project.masters[0].token.clone();
    let project = test_project.project;
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let project_budget = server
        .setup_test_project_budget(&project)
        .await
        .expect("Failed to setup test user budget");

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act
    let delete = client.project_budget.delete(project_budget.id).await;

    // assert
    assert!(delete.is_err());
    assert_eq!(
        delete.unwrap_err().to_string(),
        format!("Admin privileges required")
    );
}

#[tokio::test]
async fn e2e_lib_project_budget_delete_works() {
    // arrange
    let server = spawn_app().await;
    let test_project = server
        .setup_test_project(1, 0, 0)
        .await
        .expect("Failed to setup test project");
    let user = test_project.admins[0].user.clone();
    let token = test_project.admins[0].token.clone();
    let project = test_project.project;
    server
        .mock_keystone_auth(&token, &user.openstack_id, &user.name)
        .mount(&server.keystone_server)
        .await;
    let project_budget = server
        .setup_test_project_budget(&project)
        .await
        .expect("Failed to setup test user budget");

    // arrange
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    // act and assert 1 - delete
    client
        .project_budget
        .delete(project_budget.id)
        .await
        .unwrap();

    // act and assert 2 - get
    let get = client.project_budget.get(project_budget.id).await;
    assert!(get.is_err());
    assert_eq!(
        get.unwrap_err().to_string(),
        "Resource not found".to_string()
    );
}
