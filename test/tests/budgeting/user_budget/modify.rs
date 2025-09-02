use std::str::FromStr;

use avina::{Api, Token};
use avina_api::database::{
    accounting::server_state::NewServerState,
    budgeting::{project_budget::NewProjectBudget, user_budget::NewUserBudget},
    pricing::flavor_price::NewFlavorPrice,
};
use avina_test::{random_alphanumeric_string, random_uuid, spawn_app};
use avina_wire::user::{Project, UserClass};
use chrono::{Datelike, TimeZone, Utc};

#[tokio::test]
async fn e2e_lib_admin_cannot_modify_user_budget() {
    // arrange
    let server = spawn_app().await;

    let project_new = Project {
        id: 1,
        name: random_alphanumeric_string(10),
        openstack_id: random_uuid(),
        user_class: UserClass::UC1,
    };
    let test_project_1 = server
        .setup_test_project_with_project(1, 2, 0, project_new)
        .await
        .expect("Failed to setup test project");
    let admin = test_project_1.admins[0].user.clone();
    let token = test_project_1.admins[0].token.clone();
    let master_user_1 = test_project_1.masters[0].user.clone();
    let master_user_2 = test_project_1.masters[1].user.clone();
    let project_1 = test_project_1.project;
    let test_project_2 = server
        .setup_test_project(0, 1, 0)
        .await
        .expect("Failed to setup test project");
    let project_2 = test_project_2.project;
    let master_user_3 = test_project_2.masters[0].user.clone();

    server
        .mock_keystone_auth(&token, &admin.openstack_id, &admin.name)
        .mount(&server.keystone_server)
        .await;

    let new_project_budget_1 = NewProjectBudget {
        project_id: project_1.id as u64,
        year: Utc::now().year() as u32,
        amount: 100,
    };
    let new_project_budget_2 = NewProjectBudget {
        project_id: project_2.id as u64,
        year: Utc::now().year() as u32,
        amount: 100,
    };

    let _project_budget_1 = server
        .setup_test_project_budget_with_new_project_budget(
            &project_1,
            &new_project_budget_1,
        )
        .await
        .expect("Failed to setup test user budget");

    let _project_budget_2 = server
        .setup_test_project_budget_with_new_project_budget(
            &project_2,
            &new_project_budget_2,
        )
        .await
        .expect("Failed to setup test user budget");

    let new_user_budget_1 = NewUserBudget {
        user_id: master_user_1.id as u64,
        year: Utc::now().year() as u32,
        amount: 10,
    };

    let new_user_budget_2 = NewUserBudget {
        user_id: master_user_2.id as u64,
        year: Utc::now().year() as u32,
        amount: 0,
    };

    let new_user_budget_3 = NewUserBudget {
        user_id: master_user_3.id as u64,
        year: Utc::now().year() as u32 - 1,
        amount: 100,
    };

    let user_budget_1 = server
        .setup_test_user_budget_with_new_user_budget(
            &master_user_1,
            &new_user_budget_1,
        )
        .await
        .expect("Failed to setup test user budget");

    let user_budget_2 = server
        .setup_test_user_budget_with_new_user_budget(
            &master_user_2,
            &new_user_budget_2,
        )
        .await
        .expect("Failed to setup test user budget");

    let user_budget_3 = server
        .setup_test_user_budget_with_new_user_budget(
            &master_user_3,
            &new_user_budget_3,
        )
        .await
        .expect("Failed to setup test user budget");
    let start_time = Utc
        .with_ymd_and_hms(Utc::now().year(), 1, 1, 1, 0, 0)
        .unwrap()
        .fixed_offset()
        .into();

    let flavor = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let new_flavor_price = NewFlavorPrice {
        flavor_id: flavor.id as u64,
        user_class: project_1.user_class,
        unit_price: 200_f64, // cost: 133
        start_time,
    };
    let _flavor_price = server
        .setup_test_flavor_price_with_new_flavor_price(
            &flavor,
            new_flavor_price,
        )
        .await
        .expect("Failed to setup test flavor group");
    let new_server_state = NewServerState {
        begin: Utc
            .with_ymd_and_hms(Utc::now().year(), 1, 1, 1, 0, 0)
            .unwrap()
            .fixed_offset()
            .into(),
        end: Some(
            Utc.with_ymd_and_hms(Utc::now().year(), 12, 1, 1, 0, 0)
                .unwrap()
                .fixed_offset()
                .into(),
        ),
        instance_id: random_alphanumeric_string(10),
        instance_name: random_alphanumeric_string(10),
        flavor: flavor.id,
        status: "ACTIVE".to_string(),
        user: master_user_1.id,
    };
    let _server_state = server
        .setup_test_server_state_with_server_state(
            &flavor,
            &master_user_1,
            new_server_state,
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

    let request_1 = client
        .user_budget
        .modify(user_budget_1.id)
        .amount(110)
        .send()
        .await;
    let get_1 = client.user_budget.get(user_budget_1.id).await;

    let request_2 = client
        .user_budget
        .modify(user_budget_2.id)
        .amount(0)
        .send()
        .await;
    let get_2 = client.user_budget.get(user_budget_2.id).await;

    let request_3 = client
        .user_budget
        .modify(user_budget_3.id)
        .amount(1000)
        .send()
        .await;
    let get_3 = client.user_budget.get(user_budget_3.id).await;

    assert!(request_1.is_err());
    assert_eq!(
        request_1.unwrap_err().to_string(),
        "Cost already exceeds desired budget amount".to_string()
    );
    assert_eq!(get_1.unwrap().amount, new_user_budget_1.amount as u32);

    assert!(request_2.is_err());
    assert_eq!(
        request_2.unwrap_err().to_string(),
        "Cost already exceeds desired budget amount".to_string()
    );
    assert_eq!(get_2.unwrap().amount, new_user_budget_2.amount as u32);

    assert!(request_3.is_err());
    assert_eq!(
        request_3.unwrap_err().to_string(),
        "Changing past budgets not allowed".to_string()
    );
    assert_eq!(get_3.unwrap().amount, new_user_budget_3.amount as u32);
}

#[tokio::test]
async fn e2e_lib_admin_can_force_modify_user_budget() {
    // arrange
    let server = spawn_app().await;

    let test_project_1 = server
        .setup_test_project(1, 1, 0)
        .await
        .expect("Failed to setup test project");
    let admin = test_project_1.admins[0].user.clone();
    let master_user = test_project_1.masters[0].user.clone();
    let token = test_project_1.admins[0].token.clone();

    server
        .mock_keystone_auth(&token, &admin.openstack_id, &admin.name)
        .mount(&server.keystone_server)
        .await;

    let new_user_budget_1 = NewUserBudget {
        user_id: admin.id as u64,
        year: Utc::now().year() as u32 - 1,
        amount: 0,
    };

    let new_user_budget_2 = NewUserBudget {
        user_id: master_user.id as u64,
        year: Utc::now().year() as u32,
        amount: 0,
    };

    let user_budget_1 = server
        .setup_test_user_budget_with_new_user_budget(&admin, &new_user_budget_1)
        .await
        .expect("Failed to setup test user budget");

    let user_budget_2 = server
        .setup_test_user_budget_with_new_user_budget(&admin, &new_user_budget_2)
        .await
        .expect("Failed to setup test user budget");

    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();
    let new_user_budget_amount = 0;
    let request_1 = client
        .user_budget
        .modify(user_budget_1.id)
        .amount(new_user_budget_amount)
        .force()
        .send()
        .await;
    let get_1 = client.user_budget.get(user_budget_1.id).await;

    let request_2 = client
        .user_budget
        .modify(user_budget_2.id)
        .amount(new_user_budget_amount)
        .force()
        .send()
        .await;
    let get_2 = client.user_budget.get(user_budget_2.id).await;

    assert!(request_1.is_ok());
    assert_eq!(get_1.unwrap().amount, new_user_budget_amount);
    assert!(request_2.is_ok());
    assert_eq!(get_2.unwrap().amount, new_user_budget_amount);
}

#[tokio::test]
async fn e2e_lib_master_user_cannot_force_modify_user_budget() {
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

    let user_budget = server
        .setup_test_user_budget(&normal_user)
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
        .user_budget
        .modify(user_budget.id)
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
async fn e2e_lib_normal_user_cannot_force_modify_user_budget() {
    // arrange
    let server = spawn_app().await;

    let test_project_1 = server
        .setup_test_project(0, 0, 1)
        .await
        .expect("Failed to setup test project");
    let normal_user_1 = test_project_1.normals[0].user.clone();
    let token = test_project_1.normals[0].token.clone();
    let test_project_2 = server
        .setup_test_project(0, 0, 1)
        .await
        .expect("Failed to setup test project");
    let normal_user_2 = test_project_2.normals[0].user.clone();

    server
        .mock_keystone_auth(
            &token,
            &normal_user_1.openstack_id,
            &normal_user_1.name,
        )
        .mount(&server.keystone_server)
        .await;

    let user_budget = server
        .setup_test_user_budget(&normal_user_2)
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
        .user_budget
        .modify(user_budget.id)
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
async fn e2e_lib_master_user_cannot_modify_other_project_user_budget() {
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

    let user_budget = server
        .setup_test_user_budget(&normal_user)
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
        .user_budget
        .modify(user_budget.id)
        .amount(100)
        .send()
        .await;

    assert!(request.is_err());
    assert_eq!(
        request.unwrap_err().to_string(),
        "Resource not found".to_string()
    );
}

#[tokio::test]
async fn e2e_lib_master_user_can_modify_project_user_budget() {
    // arrange
    let server = spawn_app().await;

    let test_project = server
        .setup_test_project(0, 1, 1)
        .await
        .expect("Failed to setup test project");
    let master_user = test_project.masters[0].user.clone();
    let token = test_project.masters[0].token.clone();
    let normal_user = test_project.normals[0].user.clone();

    server
        .mock_keystone_auth(
            &token,
            &master_user.openstack_id,
            &master_user.name,
        )
        .mount(&server.keystone_server)
        .await;

    let user_budget = server
        .setup_test_user_budget(&normal_user)
        .await
        .expect("Failed to setup test user budget");

    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    let new_user_budget_amount = 10;
    let request = client
        .user_budget
        .modify(user_budget.id)
        .amount(new_user_budget_amount)
        .send()
        .await;
    let get = client.user_budget.get(user_budget.id).await;

    assert!(request.is_ok());
    assert_eq!(get.unwrap().amount, new_user_budget_amount as u32);
}
