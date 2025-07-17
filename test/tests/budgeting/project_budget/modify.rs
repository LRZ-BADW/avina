use std::str::FromStr;

use avina::{Api, Token};
use avina_api::database::budgeting::project_budget::NewProjectBudget;
use avina_test::spawn_app;
use chrono::{Datelike, Utc};

#[tokio::test]
async fn e2e_lib_admin_cannot_modify_project_budget() {
    // arrange
    let server = spawn_app().await;

    let test_project_1 = server
        .setup_test_project(1, 0, 0)
        .await
        .expect("Failed to setup test project");
    let admin = test_project_1.admins[0].user.clone();
    let token = test_project_1.admins[0].token.clone();
    let project_1 = test_project_1.project;
    let test_project_2 = server
        .setup_test_project(1, 0, 0)
        .await
        .expect("Failed to setup test project");
    let project_2 = test_project_2.project;

    server
        .mock_keystone_auth(&token, &admin.openstack_id, &admin.name)
        .mount(&server.keystone_server)
        .await;

    let new_project_budget_1 = NewProjectBudget {
        project_id: project_1.id as u64,
        year: Utc::now().year() as u32 - 1,
        amount: 100,
    };
    let new_project_budget_2 = NewProjectBudget {
        project_id: project_2.id as u64,
        year: Utc::now().year() as u32,
        amount: 100,
    };

    let project_budget_1 = server
        .setup_test_project_budget_with_new_project_budget(
            &project_1,
            &new_project_budget_1,
        )
        .await
        .expect("Failed to setup test user budget");

    let project_budget_2 = server
        .setup_test_project_budget_with_new_project_budget(
            &project_2,
            &new_project_budget_2,
        )
        .await
        .expect("Failed to setup test user budget");

    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();
    let new_project_budget_amount = 0;
    let request_1 = client
        .project_budget
        .modify(project_budget_1.id)
        .amount(new_project_budget_amount)
        .send()
        .await;
    let get_1 = client.project_budget.get(project_budget_1.id).await;

    let request_2 = client
        .project_budget
        .modify(project_budget_2.id)
        .amount(new_project_budget_amount)
        .send()
        .await;
    let get_2 = client.project_budget.get(project_budget_1.id).await;

    assert!(request_1.is_err());
    assert_eq!(
        request_1.unwrap_err().to_string(),
        "Changing past budgets not allowed".to_string()
    );
    assert_eq!(get_1.unwrap().amount, project_budget_1.amount);
    assert!(request_2.is_err());
    assert_eq!(
        request_2.unwrap_err().to_string(),
        "Cost already exceeds desired budget amount".to_string()
    );
    assert_eq!(get_2.unwrap().amount, project_budget_2.amount);
}

#[tokio::test]
async fn e2e_lib_admin_can_force_modify_project_budget() {
    // arrange
    let server = spawn_app().await;

    let test_project = server
        .setup_test_project(1, 1, 0)
        .await
        .expect("Failed to setup test project");
    let admin = test_project.admins[0].user.clone();
    let token = test_project.admins[0].token.clone();
    let project_1 = test_project.project;
    let test_project_2 = server
        .setup_test_project(1, 0, 0)
        .await
        .expect("Failed to setup test project");
    let project_2 = test_project_2.project;

    server
        .mock_keystone_auth(&token, &admin.openstack_id, &admin.name)
        .mount(&server.keystone_server)
        .await;

    let new_project_budget_1 = NewProjectBudget {
        project_id: project_1.id as u64,
        year: Utc::now().year() as u32 - 1,
        amount: 100,
    };
    let new_project_budget_2 = NewProjectBudget {
        project_id: project_2.id as u64,
        year: Utc::now().year() as u32,
        amount: 100,
    };

    let project_budget_1 = server
        .setup_test_project_budget_with_new_project_budget(
            &project_1,
            &new_project_budget_1,
        )
        .await
        .expect("Failed to setup test user budget");

    let project_budget_2 = server
        .setup_test_project_budget_with_new_project_budget(
            &project_2,
            &new_project_budget_2,
        )
        .await
        .expect("Failed to setup test user budget");
    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    let new_project_budget_amount = 0;
    let request_1 = client
        .project_budget
        .modify(project_budget_1.id)
        .amount(new_project_budget_amount)
        .force()
        .send()
        .await;
    let get_1 = client.project_budget.get(project_budget_1.id).await;

    let request_2 = client
        .project_budget
        .modify(project_budget_2.id)
        .amount(new_project_budget_amount)
        .force()
        .send()
        .await;
    let get_2 = client.project_budget.get(project_budget_1.id).await;

    assert!(request_1.is_ok());
    assert_eq!(get_1.unwrap().amount, new_project_budget_amount);
    assert!(request_2.is_ok());
    assert_eq!(get_2.unwrap().amount, new_project_budget_amount);
}

#[tokio::test]
async fn e2e_lib_master_user_cannot_force_modify_project_budget() {
    // arrange
    let server = spawn_app().await;

    let test_project = server
        .setup_test_project(0, 1, 0)
        .await
        .expect("Failed to setup test project");
    let master_user = test_project.masters[0].user.clone();
    let token = test_project.masters[0].token.clone();
    let project = test_project.project;

    server
        .mock_keystone_auth(
            &token,
            &master_user.openstack_id,
            &master_user.name,
        )
        .mount(&server.keystone_server)
        .await;

    let project_budget = server
        .setup_test_project_budget(&project)
        .await
        .expect("Failed to setup test user budget");

    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    let request = client
        .project_budget
        .modify(project_budget.id)
        .force()
        .send()
        .await;

    assert!(request.is_err());
    assert_eq!(
        request.unwrap_err().to_string(),
        "Admin privileges required".to_string()
    );
}

#[tokio::test]
async fn e2e_lib_normal_user_cannot_force_modify_project_budget() {
    // arrange
    let server = spawn_app().await;

    let test_project = server
        .setup_test_project(0, 0, 1)
        .await
        .expect("Failed to setup test project");
    let master_user = test_project.normals[0].user.clone();
    let token = test_project.normals[0].token.clone();
    let project = test_project.project;

    server
        .mock_keystone_auth(
            &token,
            &master_user.openstack_id,
            &master_user.name,
        )
        .mount(&server.keystone_server)
        .await;

    let project_budget = server
        .setup_test_project_budget(&project)
        .await
        .expect("Failed to setup test user budget");

    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    let request = client
        .project_budget
        .modify(project_budget.id)
        .force()
        .send()
        .await;

    assert!(request.is_err());
    assert_eq!(
        request.unwrap_err().to_string(),
        "Admin privileges required".to_string()
    );
}

#[tokio::test]
async fn e2e_lib_master_user_cannot_modify_other_project_budget() {
    // arrange
    let server = spawn_app().await;

    let test_project_1 = server
        .setup_test_project(0, 1, 0)
        .await
        .expect("Failed to setup test project");
    let master_user = test_project_1.masters[0].user.clone();
    let token = test_project_1.masters[0].token.clone();
    let test_project_2 = server
        .setup_test_project(0, 1, 0)
        .await
        .expect("Failed to setup test project");
    let project_2 = test_project_2.project;

    server
        .mock_keystone_auth(
            &token,
            &master_user.openstack_id,
            &master_user.name,
        )
        .mount(&server.keystone_server)
        .await;

    let project_budget = server
        .setup_test_project_budget(&project_2)
        .await
        .expect("Failed to setup test user budget");

    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    let new_project_budget_amount = 100;
    let request = client
        .project_budget
        .modify(project_budget.id)
        .amount(new_project_budget_amount)
        .send()
        .await;

    assert!(request.is_err());
    assert_eq!(
        request.unwrap_err().to_string(),
        "Resource not found".to_string()
    );
}
