use std::str::FromStr;

use avina::{Api, Token};
use avina_api::database::pricing::flavor_price::NewFlavorPrice;
use avina_test::spawn_app;
use avina_wire::user::UserClass;
use chrono::{Datelike, TimeZone, Utc};

#[tokio::test]
async fn e2e_lib_flavor_price_list() {
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

    let user_class_1 = UserClass::UC1;
    let flavor_1 = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let new_flavor_price_1 = NewFlavorPrice {
        flavor_id: flavor_1.id as u64,
        user_class: user_class_1,
        unit_price: 0_f64,
        start_time: Utc
            .with_ymd_and_hms(Utc::now().year(), 1, 1, 1, 1, 1)
            .unwrap(),
    };
    let flavor_price_1 = server
        .setup_test_flavor_price_with_new_flavor_price(
            &flavor_1,
            new_flavor_price_1,
        )
        .await
        .expect("Failed to setup test flavor group");

    let user_class_2 = UserClass::UC2;
    let flavor_2 = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let new_flavor_price_2 = NewFlavorPrice {
        flavor_id: flavor_2.id as u64,
        user_class: user_class_2,
        unit_price: 0_f64,
        start_time: Utc
            .with_ymd_and_hms(Utc::now().year(), 1, 1, 1, 1, 1)
            .unwrap(),
    };
    let flavor_price_2 = server
        .setup_test_flavor_price_with_new_flavor_price(
            &flavor_2,
            new_flavor_price_2,
        )
        .await
        .expect("Failed to setup test flavor group");

    let user_class_3 = UserClass::UC3;
    let flavor_3 = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let new_flavor_price_3 = NewFlavorPrice {
        flavor_id: flavor_3.id as u64,
        user_class: user_class_3,
        unit_price: 0_f64,
        start_time: Utc
            .with_ymd_and_hms(Utc::now().year(), 1, 1, 1, 1, 1)
            .unwrap(),
    };
    let flavor_price_3 = server
        .setup_test_flavor_price_with_new_flavor_price(
            &flavor_3,
            new_flavor_price_3,
        )
        .await
        .expect("Failed to setup test flavor group");

    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    let list = client.flavor_price.list().send().await.unwrap();

    assert_eq!(list.len(), 3);
    assert!(list.contains(&flavor_price_1));
    assert!(list.contains(&flavor_price_2));
    assert!(list.contains(&flavor_price_3));
}

#[tokio::test]
async fn e2e_lib_flavor_price_list_for_user_class() {
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

    let user_class_1 = UserClass::UC1;
    let flavor_1 = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let new_flavor_price_1 = NewFlavorPrice {
        flavor_id: flavor_1.id as u64,
        user_class: user_class_1,
        unit_price: 0_f64,
        start_time: Utc
            .with_ymd_and_hms(Utc::now().year(), 1, 1, 1, 1, 1)
            .unwrap(),
    };
    let flavor_price_1 = server
        .setup_test_flavor_price_with_new_flavor_price(
            &flavor_1,
            new_flavor_price_1,
        )
        .await
        .expect("Failed to setup test flavor group");

    let user_class_2 = UserClass::UC2;
    let flavor_2 = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let new_flavor_price_2 = NewFlavorPrice {
        flavor_id: flavor_2.id as u64,
        user_class: user_class_2,
        unit_price: 0_f64,
        start_time: Utc
            .with_ymd_and_hms(Utc::now().year(), 1, 1, 1, 1, 1)
            .unwrap(),
    };
    let _flavor_price_2 = server
        .setup_test_flavor_price_with_new_flavor_price(
            &flavor_2,
            new_flavor_price_2,
        )
        .await
        .expect("Failed to setup test flavor group");

    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    let list = client
        .flavor_price
        .list()
        .user_class(user_class_1)
        .send()
        .await
        .unwrap();

    assert_eq!(list.len(), 1);
    assert!(list.contains(&flavor_price_1));
}

#[tokio::test]
async fn e2e_lib_flavor_price_list_current() {
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

    let user_class_1 = UserClass::UC1;
    let flavor_1 = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let new_flavor_price_1 = NewFlavorPrice {
        flavor_id: flavor_1.id as u64,
        user_class: user_class_1,
        unit_price: 0_f64,
        start_time: Utc
            .with_ymd_and_hms(Utc::now().year(), 1, 1, 1, 1, 1)
            .unwrap(),
    };
    let flavor_price_1 = server
        .setup_test_flavor_price_with_new_flavor_price(
            &flavor_1,
            new_flavor_price_1,
        )
        .await
        .expect("Failed to setup test flavor group");

    let user_class_2 = UserClass::UC2;
    let flavor_2 = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let new_flavor_price_2 = NewFlavorPrice {
        flavor_id: flavor_2.id as u64,
        user_class: user_class_2,
        unit_price: 0_f64,
        start_time: Utc
            .with_ymd_and_hms(Utc::now().year(), 1, 1, 1, 1, 1)
            .unwrap(),
    };
    let flavor_price_2 = server
        .setup_test_flavor_price_with_new_flavor_price(
            &flavor_2,
            new_flavor_price_2,
        )
        .await
        .expect("Failed to setup test flavor group");

    let user_class_3 = UserClass::UC3;
    let flavor_3 = server
        .setup_test_flavor()
        .await
        .expect("Failed to setup test flavor");
    let new_flavor_price_3 = NewFlavorPrice {
        flavor_id: flavor_3.id as u64,
        user_class: user_class_3,
        unit_price: 0_f64,
        start_time: Utc::now().with_year(2100).unwrap(),
    };
    let _flavor_price_3 = server
        .setup_test_flavor_price_with_new_flavor_price(
            &flavor_3,
            new_flavor_price_3,
        )
        .await
        .expect("Failed to setup test flavor group");

    let client = Api::new(
        format!("{}/api", &server.address),
        Token::from_str(&token).unwrap(),
        None,
        None,
    )
    .unwrap();

    let list = client.flavor_price.list().current().send().await.unwrap();

    assert_eq!(list.len(), 2);
    assert!(list.contains(&flavor_price_1));
    assert!(list.contains(&flavor_price_2));
}
