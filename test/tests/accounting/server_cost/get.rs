use std::str::FromStr;

use avina::{Api, Token};
use avina_api::database::{
    accounting::server_state::NewServerState,
    pricing::flavor_price::NewFlavorPrice,
};
use avina_test::{random_alphanumeric_string, random_uuid, spawn_app};
use avina_wire::user::{Project, UserClass};
use chrono::{Datelike, TimeZone, Utc};
use uuid::Uuid;

#[tokio::test]
async fn e2e_lib_server_cost_for_server() {
    let server = spawn_app().await;

    let user_class = UserClass::UC1;
    let project_new = Project {
        id: 1,
        name: random_alphanumeric_string(10),
        openstack_id: random_uuid(),
        user_class,
    };
    let test_project = server
        .setup_test_project_with_project(0, 1, 0, project_new)
        .await
        .expect("Failed to setup test project");
    let master_user = test_project.masters[0].user.clone();
    let token = test_project.masters[0].token.clone();
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
    let start_time = Utc
        .with_ymd_and_hms(Utc::now().year(), 1, 1, 1, 1, 1)
        .unwrap();
    let end_time = Utc
        .with_ymd_and_hms(Utc::now().year(), 11, 1, 1, 1, 1)
        .unwrap();
    let new_flavor_price = NewFlavorPrice {
        flavor_id: flavor.id as u64,
        user_class,
        unit_price: 1000_f64,
        start_time,
    };
    let _flavor_price = server
        .setup_test_flavor_price_with_new_flavor_price(
            &flavor,
            new_flavor_price,
        )
        .await
        .expect("Failed to setup test flavor group");
    let instance_id = Uuid::new_v4();
    let new_server_state = NewServerState {
        begin: start_time.fixed_offset().into(),
        end: end_time.into(),
        instance_id,
        instance_name: random_alphanumeric_string(10),
        flavor: flavor.id,
        status: "ACTIVE".to_string(),
        user: master_user.id,
    };
    let _server_state = server
        .setup_test_server_state_with_server_state(
            &flavor,
            &master_user,
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

    let cost = client
        .server_cost
        .get()
        .begin(start_time.into())
        .end(end_time.into())
        .server(instance_id)
        .await
        .unwrap();

    assert_eq!(cost.total.round(), 833.0);
}

#[tokio::test]
async fn e2e_lib_server_cost_for_user_detail() {
    let server = spawn_app().await;

    let user_class = UserClass::UC1;
    let project_new = Project {
        id: 1,
        name: random_alphanumeric_string(10),
        openstack_id: random_uuid(),
        user_class,
    };
    let test_project = server
        .setup_test_project_with_project(0, 1, 0, project_new)
        .await
        .expect("Failed to setup test project");
    let master_user = test_project.masters[0].user.clone();
    let token = test_project.masters[0].token.clone();
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
    let start_time = Utc
        .with_ymd_and_hms(Utc::now().year(), 1, 1, 1, 1, 1)
        .unwrap();
    let end_time = Utc
        .with_ymd_and_hms(Utc::now().year(), 11, 1, 1, 1, 1)
        .unwrap();
    let new_flavor_price = NewFlavorPrice {
        flavor_id: flavor.id as u64,
        user_class,
        unit_price: 1000_f64,
        start_time,
    };
    let _flavor_price = server
        .setup_test_flavor_price_with_new_flavor_price(
            &flavor,
            new_flavor_price,
        )
        .await
        .expect("Failed to setup test flavor group");
    let instance_id = Uuid::new_v4();
    let new_server_state = NewServerState {
        begin: start_time.fixed_offset().into(),
        end: end_time.into(),
        instance_id,
        instance_name: random_alphanumeric_string(10),
        flavor: flavor.id,
        status: "ACTIVE".to_string(),
        user: master_user.id,
    };
    let _server_state = server
        .setup_test_server_state_with_server_state(
            &flavor,
            &master_user,
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

    let cost = client
        .server_cost
        .get()
        .begin(start_time.into())
        .end(end_time.into())
        .server_detail(instance_id)
        .await
        .unwrap();

    assert_eq!(cost.total.round(), 833.0);
}

#[tokio::test]
async fn e2e_lib_server_cost_for_user() {
    let server = spawn_app().await;

    let user_class = UserClass::UC1;
    let project_new = Project {
        id: 1,
        name: random_alphanumeric_string(10),
        openstack_id: random_uuid(),
        user_class,
    };
    let test_project = server
        .setup_test_project_with_project(0, 1, 0, project_new)
        .await
        .expect("Failed to setup test project");
    let master_user = test_project.masters[0].user.clone();
    let token = test_project.masters[0].token.clone();
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
    let start_time = Utc
        .with_ymd_and_hms(Utc::now().year(), 1, 1, 1, 1, 1)
        .unwrap();
    let end_time = Utc
        .with_ymd_and_hms(Utc::now().year(), 11, 1, 1, 1, 1)
        .unwrap();
    let new_flavor_price = NewFlavorPrice {
        flavor_id: flavor.id as u64,
        user_class,
        unit_price: 1000_f64,
        start_time,
    };
    let _flavor_price = server
        .setup_test_flavor_price_with_new_flavor_price(
            &flavor,
            new_flavor_price,
        )
        .await
        .expect("Failed to setup test flavor group");
    let instance_id_1 = Uuid::new_v4();
    let new_server_state_1 = NewServerState {
        begin: start_time,
        end: end_time.into(),
        instance_id: instance_id_1,
        instance_name: random_alphanumeric_string(10),
        flavor: flavor.id,
        status: "ACTIVE".to_string(),
        user: master_user.id,
    };
    let _server_state_1 = server
        .setup_test_server_state_with_server_state(
            &flavor,
            &master_user,
            new_server_state_1,
        )
        .await
        .expect("Failed to setup test user budget");
    let instance_id_2 = Uuid::new_v4();
    let new_server_state_2 = NewServerState {
        begin: start_time.fixed_offset().into(),
        end: end_time.into(),
        instance_id: instance_id_2,
        instance_name: random_alphanumeric_string(10),
        flavor: flavor.id,
        status: "ACTIVE".to_string(),
        user: master_user.id,
    };
    let _server_state_2 = server
        .setup_test_server_state_with_server_state(
            &flavor,
            &master_user,
            new_server_state_2,
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

    let cost = client
        .server_cost
        .get()
        .begin(start_time.into())
        .end(end_time.into())
        .user(master_user.id)
        .await
        .unwrap();

    assert_eq!(cost.total.round(), 1666.0);
}

#[tokio::test]
async fn e2e_lib_server_cost_mine() {
    let server = spawn_app().await;

    let user_class = UserClass::UC1;
    let project_new = Project {
        id: 1,
        name: random_alphanumeric_string(10),
        openstack_id: random_uuid(),
        user_class,
    };
    let test_project = server
        .setup_test_project_with_project(0, 1, 0, project_new)
        .await
        .expect("Failed to setup test project");
    let master_user = test_project.masters[0].user.clone();
    let token = test_project.masters[0].token.clone();
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
    let start_time = Utc
        .with_ymd_and_hms(Utc::now().year(), 1, 1, 1, 1, 1)
        .unwrap();
    let end_time = Utc
        .with_ymd_and_hms(Utc::now().year(), 11, 1, 1, 1, 1)
        .unwrap();
    let new_flavor_price = NewFlavorPrice {
        flavor_id: flavor.id as u64,
        user_class,
        unit_price: 1000_f64,
        start_time,
    };
    let _flavor_price = server
        .setup_test_flavor_price_with_new_flavor_price(
            &flavor,
            new_flavor_price,
        )
        .await
        .expect("Failed to setup test flavor group");
    let instance_id = Uuid::new_v4();
    let new_server_state = NewServerState {
        begin: start_time.fixed_offset().into(),
        end: end_time.into(),
        instance_id,
        instance_name: random_alphanumeric_string(10),
        flavor: flavor.id,
        status: "ACTIVE".to_string(),
        user: master_user.id,
    };
    let _server_state = server
        .setup_test_server_state_with_server_state(
            &flavor,
            &master_user,
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

    let cost = client
        .server_cost
        .get()
        .begin(start_time.into())
        .end(end_time.into())
        .mine()
        .await
        .unwrap();

    assert_eq!(cost.total.round(), 833.0);
}

#[tokio::test]
async fn e2e_lib_server_cost_for_project() {
    let server = spawn_app().await;

    let user_class = UserClass::UC1;
    let project_new = Project {
        id: 1,
        name: random_alphanumeric_string(10),
        openstack_id: random_uuid(),
        user_class,
    };
    let test_project = server
        .setup_test_project_with_project(0, 1, 1, project_new)
        .await
        .expect("Failed to setup test project");
    let master_user = test_project.masters[0].user.clone();
    let normal_user = test_project.normals[0].user.clone();
    let project = test_project.project;
    let token = test_project.masters[0].token.clone();
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
    let start_time = Utc
        .with_ymd_and_hms(Utc::now().year(), 1, 1, 1, 1, 1)
        .unwrap();
    let end_time = Utc
        .with_ymd_and_hms(Utc::now().year(), 11, 1, 1, 1, 1)
        .unwrap();
    let new_flavor_price = NewFlavorPrice {
        flavor_id: flavor.id as u64,
        user_class,
        unit_price: 1000_f64,
        start_time,
    };
    let _flavor_price = server
        .setup_test_flavor_price_with_new_flavor_price(
            &flavor,
            new_flavor_price,
        )
        .await
        .expect("Failed to setup test flavor group");
    let instance_id_1 = Uuid::new_v4();
    let new_server_state_1 = NewServerState {
        begin: start_time,
        end: end_time.into(),
        instance_id: instance_id_1,
        instance_name: random_alphanumeric_string(10),
        flavor: flavor.id,
        status: "ACTIVE".to_string(),
        user: master_user.id,
    };
    let _server_state_1 = server
        .setup_test_server_state_with_server_state(
            &flavor,
            &master_user,
            new_server_state_1,
        )
        .await
        .expect("Failed to setup test user budget");
    let instance_id_2 = Uuid::new_v4();
    let new_server_state_2 = NewServerState {
        begin: start_time.fixed_offset().into(),
        end: end_time.into(),
        instance_id: instance_id_2,
        instance_name: random_alphanumeric_string(10),
        flavor: flavor.id,
        status: "ACTIVE".to_string(),
        user: normal_user.id,
    };
    let _server_state_2 = server
        .setup_test_server_state_with_server_state(
            &flavor,
            &normal_user,
            new_server_state_2,
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

    let cost = client
        .server_cost
        .get()
        .begin(start_time.into())
        .end(end_time.into())
        .project(project.id)
        .await
        .unwrap();

    assert_eq!(cost.total.round(), 1666.0);
}

#[tokio::test]
async fn e2e_lib_server_cost_all() {
    let server = spawn_app().await;

    let user_class = UserClass::UC1;
    let project_new_1 = Project {
        id: 1,
        name: random_alphanumeric_string(10),
        openstack_id: random_uuid(),
        user_class,
    };
    let project_new_2 = Project {
        id: 2,
        name: random_alphanumeric_string(10),
        openstack_id: random_uuid(),
        user_class,
    };
    let test_project_1 = server
        .setup_test_project_with_project(1, 1, 0, project_new_1)
        .await
        .expect("Failed to setup test project");
    let admin = test_project_1.admins[0].user.clone();
    let master_user = test_project_1.masters[0].user.clone();
    let token = test_project_1.masters[0].token.clone();

    let test_project_2 = server
        .setup_test_project_with_project(0, 0, 1, project_new_2)
        .await
        .expect("Failed to setup test project");
    let normal_user = test_project_2.normals[0].user.clone();

    server
        .mock_keystone_auth(&token, &admin.openstack_id, &admin.name)
        .mount(&server.keystone_server)
        .await;

    let flavor = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let start_time = Utc
        .with_ymd_and_hms(Utc::now().year(), 1, 1, 1, 0, 0)
        .unwrap();
    let end_time = Utc
        .with_ymd_and_hms(Utc::now().year(), 11, 1, 1, 1, 1)
        .unwrap();
    let new_flavor_price = NewFlavorPrice {
        flavor_id: flavor.id as u64,
        user_class,
        unit_price: 1000_f64,
        start_time,
    };
    let _flavor_price = server
        .setup_test_flavor_price_with_new_flavor_price(
            &flavor,
            new_flavor_price,
        )
        .await
        .expect("Failed to setup test flavor group");
    let instance_id_1 = Uuid::new_v4();
    let new_server_state_1 = NewServerState {
        begin: start_time,
        end: end_time.into(),
        instance_id: instance_id_1,
        instance_name: random_alphanumeric_string(10),
        flavor: flavor.id,
        status: "ACTIVE".to_string(),
        user: master_user.id,
    };
    let _server_state_1 = server
        .setup_test_server_state_with_server_state(
            &flavor,
            &master_user,
            new_server_state_1,
        )
        .await
        .expect("Failed to setup test user budget");
    let instance_id_2 = Uuid::new_v4();
    let new_server_state_2 = NewServerState {
        begin: start_time.fixed_offset().into(),
        end: end_time.into(),
        instance_id: instance_id_2,
        instance_name: random_alphanumeric_string(10),
        flavor: flavor.id,
        status: "ACTIVE".to_string(),
        user: normal_user.id,
    };
    let _server_state_2 = server
        .setup_test_server_state_with_server_state(
            &flavor,
            &normal_user,
            new_server_state_2,
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

    let cost = client
        .server_cost
        .get()
        .begin(start_time.into())
        .end(end_time.into())
        .all()
        .await
        .unwrap();

    assert_eq!(cost.total.round(), 1666.0);
}
