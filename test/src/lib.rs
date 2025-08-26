use std::ops::Range;

use anyhow::Context;
use avina_api::{
    configuration::{DatabaseSettings, get_configuration},
    database::{
        accounting::server_state::{
            NewServerState, insert_server_state_into_db,
        },
        budgeting::{
            project_budget::{NewProjectBudget, insert_project_budget_into_db},
            user_budget::{NewUserBudget, insert_user_budget_into_db},
        },
        pricing::flavor_price::{NewFlavorPrice, insert_flavor_price_into_db},
        quota::flavor_quota::insert_flavor_quota_into_db,
        resources::{
            flavor::insert_flavor_into_db,
            flavor_group::insert_flavor_group_into_db,
        },
    },
    error::MinimalApiError,
    startup::{Application, get_connection_pool},
    telemetry::{get_subscriber, init_subscriber},
};
use avina_wire::{
    accounting::ServerState,
    budgeting::{ProjectBudget, UserBudget},
    pricing::FlavorPrice,
    quota::{FlavorQuota, FlavorQuotaCreateData},
    resources::{Flavor, FlavorCreateData, FlavorGroup, FlavorGroupCreateData},
    user::{Project, User},
};
use chrono::{DateTime, Datelike, FixedOffset, Utc};
use once_cell::sync::Lazy;
use rand::{Rng, distr::Alphanumeric, rng};
use serde_json::json;
use sqlx::{
    Connection, Executor, MySql, MySqlConnection, MySqlPool, Transaction,
};
use uuid::Uuid;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{header, method, path},
};

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(
            subscriber_name,
            default_filter_level,
            std::io::stdout,
        );
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(
            subscriber_name,
            default_filter_level,
            std::io::sink,
        );
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub _port: u16,
    pub db_pool: sqlx::MySqlPool,
    pub _api_client: reqwest::Client,
    pub keystone_server: MockServer,
    pub keystone_token: String,
}

pub struct TestUser {
    pub user: User,
    pub token: String,
}

pub struct TestProject {
    pub project: Project,
    pub admins: Vec<TestUser>,
    pub masters: Vec<TestUser>,
    pub normals: Vec<TestUser>,
}

impl TestApp {
    pub fn mock_keystone_auth(
        &self,
        token: &str,
        os_project_id: &str,
        os_project_name: &str,
    ) -> Mock {
        Mock::given(method("GET"))
            .and(path("/auth/tokens/"))
            .and(header("X-Subject-Token", token))
            .respond_with(
                ResponseTemplate::new(200)
                    .append_header("X-Subject-Token", &self.keystone_token)
                    .set_body_json(json!({
                        "token": {
                            "project": {
                                "id": os_project_id,
                                "name": os_project_name,
                            }
                        }
                    })),
            )
    }

    pub async fn setup_test_user_and_project(
        &self,
        admin: bool,
    ) -> Result<(User, Project, String), sqlx::Error> {
        let numbers = if admin { (1, 0, 0) } else { (0, 0, 1) };
        let test_project = self
            .setup_test_project(numbers.0, numbers.1, numbers.2)
            .await?;
        let test_user = if admin {
            &test_project.admins[0]
        } else {
            &test_project.normals[0]
        };
        Ok((
            test_user.user.clone(),
            test_project.project,
            test_user.token.clone(),
        ))
    }

    pub async fn setup_test_user(
        &self,
        transaction: &mut Transaction<'static, MySql>,
        project: &Project,
        is_staff: bool,
        role: u32,
    ) -> Result<TestUser, sqlx::Error> {
        let mut user = User {
            id: 1,
            name: random_alphanumeric_string(10),
            openstack_id: random_uuid(),
            project: project.id,
            project_name: project.name.clone(),
            is_staff,
            is_active: true,
            role,
        };
        user.id = insert_user_into_db(transaction, &user).await? as u32;
        let token = Uuid::new_v4().to_string();
        Ok(TestUser { user, token })
    }

    pub async fn setup_test_project(
        &self,
        admin_number: usize,
        master_number: usize,
        normal_number: usize,
    ) -> Result<TestProject, sqlx::Error> {
        let mut transaction = self
            .db_pool
            .begin()
            .await
            .expect("Failed to begin transaction.");

        let mut project = Project {
            id: 1,
            name: random_alphanumeric_string(10),
            openstack_id: random_uuid(),
            user_class: random_number(1..6),
        };
        project.id =
            insert_project_into_db(&mut transaction, &project).await? as u32;

        let mut test_project = TestProject {
            project,
            admins: Vec::new(),
            masters: Vec::new(),
            normals: Vec::new(),
        };

        for _ in 0..admin_number {
            test_project.admins.push(
                self.setup_test_user(
                    &mut transaction,
                    &test_project.project,
                    true,
                    1,
                )
                .await?,
            );
        }

        for _ in 0..master_number {
            test_project.masters.push(
                self.setup_test_user(
                    &mut transaction,
                    &test_project.project,
                    false,
                    2,
                )
                .await?,
            );
        }

        for _ in 0..normal_number {
            test_project.normals.push(
                self.setup_test_user(
                    &mut transaction,
                    &test_project.project,
                    false,
                    1,
                )
                .await?,
            );
        }

        transaction.commit().await?;

        Ok(test_project)
    }

    pub async fn setup_test_flavor_group(
        &self,
        project_id: u32,
    ) -> Result<FlavorGroup, MinimalApiError> {
        let mut transaction = self
            .db_pool
            .begin()
            .await
            .expect("Failed to begin transaction.");
        let flavor_group_create = FlavorGroupCreateData {
            name: random_alphanumeric_string(10),
            flavors: Vec::new(),
        };
        let flavor_group_id = insert_flavor_group_into_db(
            &mut transaction,
            &flavor_group_create,
            project_id as u64,
        )
        .await? as u32;
        transaction
            .commit()
            .await
            .context("Failed to commit transaction")?;
        let flavor = FlavorGroup {
            id: flavor_group_id,
            name: flavor_group_create.name,
            project: project_id,
            flavors: flavor_group_create.flavors,
        };
        Ok(flavor)
    }

    pub async fn setup_test_flavor(&self) -> Result<Flavor, MinimalApiError> {
        let mut transaction = self
            .db_pool
            .begin()
            .await
            .expect("Failed to begin transaction.");
        let flavor_create = FlavorCreateData {
            name: random_alphanumeric_string(10),
            openstack_id: random_uuid(),
            group: None,
            weight: None,
        };
        let flavor_id = insert_flavor_into_db(&mut transaction, &flavor_create)
            .await? as u32;
        transaction
            .commit()
            .await
            .context("Failed to commit transaction")?;
        let flavor = Flavor {
            id: flavor_id,
            name: flavor_create.name,
            openstack_id: flavor_create.openstack_id,
            group: None,
            group_name: None,
            weight: 0,
        };
        Ok(flavor)
    }

    pub async fn setup_test_server_state(
        &self,
        flavor: &Flavor,
        user: &User,
    ) -> Result<ServerState, MinimalApiError> {
        let mut transaction = self
            .db_pool
            .begin()
            .await
            .expect("Failed to begin transaction.");
        let begin = DateTime::<FixedOffset>::from(Utc::now());
        let new_server_state = NewServerState {
            begin: begin.to_utc(),
            end: None,
            instance_id: random_uuid(),
            instance_name: random_alphanumeric_string(10),
            flavor: flavor.id,
            status: "ACTIVE".to_string(),
            user: user.id,
        };
        let server_state_id =
            insert_server_state_into_db(&mut transaction, &new_server_state)
                .await? as u32;
        transaction
            .commit()
            .await
            .context("Failed to commit transaction")?;
        let server_state = ServerState {
            id: server_state_id,
            begin,
            end: None,
            instance_id: new_server_state.instance_id,
            instance_name: new_server_state.instance_name,
            flavor: new_server_state.flavor,
            flavor_name: flavor.name.clone(),
            status: new_server_state.status,
            user: user.id,
            username: user.name.clone(),
        };
        Ok(server_state)
    }

    pub async fn setup_test_server_state_with_server_id(
        &self,
        flavor: &Flavor,
        user: &User,
        server_id: &str,
    ) -> Result<ServerState, MinimalApiError> {
        let mut transaction = self
            .db_pool
            .begin()
            .await
            .expect("Failed to begin transaction.");
        let begin = DateTime::<FixedOffset>::from(Utc::now());
        let new_server_state = NewServerState {
            begin: begin.to_utc(),
            end: None,
            instance_id: server_id.to_string(),
            instance_name: random_alphanumeric_string(10),
            flavor: flavor.id,
            status: "ACTIVE".to_string(),
            user: user.id,
        };
        let server_state_id =
            insert_server_state_into_db(&mut transaction, &new_server_state)
                .await? as u32;
        transaction
            .commit()
            .await
            .context("Failed to commit transaction")?;
        let server_state = ServerState {
            id: server_state_id,
            begin: new_server_state.begin.fixed_offset(),
            end: None,
            instance_id: new_server_state.instance_id,
            instance_name: new_server_state.instance_name,
            flavor: new_server_state.flavor,
            flavor_name: flavor.name.clone(),
            status: new_server_state.status,
            user: user.id,
            username: user.name.clone(),
        };
        Ok(server_state)
    }

    pub async fn setup_test_user_budget(
        &self,
        user: &User,
    ) -> Result<UserBudget, MinimalApiError> {
        let new_project_budget = NewUserBudget {
            user_id: user.id as u64,
            year: Utc::now().year() as u32,
            amount: 0,
        };
        self.setup_test_user_budget_with_new_user_budget(
            user,
            &new_project_budget,
        )
        .await
    }

    pub async fn setup_test_user_budget_with_new_user_budget(
        &self,
        user: &User,
        new_user_budget: &NewUserBudget,
    ) -> Result<UserBudget, MinimalApiError> {
        let mut transaction = self
            .db_pool
            .begin()
            .await
            .expect("Failed to begin transaction.");
        let user_budget_id =
            insert_user_budget_into_db(&mut transaction, new_user_budget)
                .await? as u32;
        transaction
            .commit()
            .await
            .context("Failed to commit transaction")?;
        let user_budget = UserBudget {
            id: user_budget_id,
            user: user.id,
            username: user.name.clone(),
            year: new_user_budget.year,
            amount: new_user_budget.amount as u32,
        };
        Ok(user_budget)
    }

    pub async fn setup_test_project_budget(
        &self,
        project: &Project,
    ) -> Result<ProjectBudget, MinimalApiError> {
        let new_project_budget = NewProjectBudget {
            project_id: project.id as u64,
            year: Utc::now().year() as u32,
            amount: 0,
        };
        self.setup_test_project_budget_with_new_project_budget(
            project,
            &new_project_budget,
        )
        .await
    }

    pub async fn setup_test_project_budget_with_new_project_budget(
        &self,
        project: &Project,
        new_project_budget: &NewProjectBudget,
    ) -> Result<ProjectBudget, MinimalApiError> {
        let mut transaction = self
            .db_pool
            .begin()
            .await
            .expect("Failed to begin transaction.");

        let project_budget_id =
            insert_project_budget_into_db(&mut transaction, new_project_budget)
                .await? as u32;
        transaction
            .commit()
            .await
            .context("Failed to commit transaction")?;
        let project_budget = ProjectBudget {
            id: project_budget_id,
            project: project.id,
            project_name: project.name.clone(),
            year: new_project_budget.year,
            amount: new_project_budget.amount as u32,
        };
        Ok(project_budget)
    }

    pub async fn setup_test_flavor_price(
        &self,
        flavor: &Flavor,
    ) -> Result<FlavorPrice, MinimalApiError> {
        let start_time = DateTime::<FixedOffset>::from(Utc::now());
        let new_flavor_price = NewFlavorPrice {
            flavor_id: flavor.id as u64,
            user_class: 1,
            unit_price: random_number(1..1000) as f64,
            start_time: start_time.to_utc(),
        };
        self.setup_test_flavor_price_with_new_flavor_price(
            flavor,
            new_flavor_price,
        )
        .await
    }

    pub async fn setup_test_flavor_price_with_new_flavor_price(
        &self,
        flavor: &Flavor,
        new_flavor_price: NewFlavorPrice,
    ) -> Result<FlavorPrice, MinimalApiError> {
        let mut transaction = self
            .db_pool
            .begin()
            .await
            .expect("Failed to begin transaction.");

        let flavor_price_id =
            insert_flavor_price_into_db(&mut transaction, &new_flavor_price)
                .await? as u32;
        transaction
            .commit()
            .await
            .context("Failed to commit transaction")?;
        let flavor_price = FlavorPrice {
            id: flavor_price_id,
            flavor: flavor.id,
            flavor_name: flavor.name.clone(),
            user_class: new_flavor_price.user_class,
            unit_price: new_flavor_price.unit_price,
            start_time: new_flavor_price.start_time.into(),
        };
        Ok(flavor_price)
    }

    pub async fn setup_test_flavor_quota(
        &self,
        flavor_group: &FlavorGroup,
        user: &User,
    ) -> Result<FlavorQuota, MinimalApiError> {
        let mut transaction = self
            .db_pool
            .begin()
            .await
            .expect("Failed to begin transaction.");
        let new_flavor_quota = FlavorQuotaCreateData {
            flavor_group: flavor_group.id,
            user: user.id,
            quota: random_number(1..1000) as i64,
        };
        let flavor_quota_id =
            insert_flavor_quota_into_db(&mut transaction, &new_flavor_quota)
                .await? as u32;
        transaction
            .commit()
            .await
            .context("Failed to commit transaction")?;
        let flavor_quota = FlavorQuota {
            id: flavor_quota_id,
            flavor_group: flavor_group.id,
            flavor_group_name: flavor_group.name.clone(),
            user: user.id,
            username: user.name.clone(),
            quota: new_flavor_quota.quota,
        };
        Ok(flavor_quota)
    }
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let keystone_server = MockServer::start().await;
    let keystone_token = Uuid::new_v4().to_string();

    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        c.database.database_name = Uuid::new_v4().simple().to_string();
        c.application.port = 0;
        c.openstack.keystone_endpoint = keystone_server.uri();
        c.application.insert_admin = false;
        c
    };

    configure_database(&configuration.database).await;

    Mock::given(method("POST"))
        .and(path("/auth/tokens/"))
        .respond_with(
            ResponseTemplate::new(201)
                .append_header("X-Subject-Token", &keystone_token),
        )
        .mount(&keystone_server)
        .await;
    // TODO check data sent to keystone

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    let application_port = application.port();
    #[allow(clippy::let_underscore_future)]
    let _ = tokio::spawn(application.run_until_stopped());
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    TestApp {
        address: format!("http://127.0.0.1:{application_port}"),
        _port: application_port,
        db_pool: get_connection_pool(&configuration.database),
        _api_client: client,
        keystone_server,
        keystone_token,
    }
}

async fn configure_database(config: &DatabaseSettings) -> MySqlPool {
    // Create database
    let mut connection = MySqlConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to MariaDB.");
    connection
        .execute(
            format!(
                "CREATE DATABASE IF NOT EXISTS `{}`;",
                config.database_name
            )
            .as_str(),
        )
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = MySqlPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to MariaDB.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database.");

    connection_pool
}

pub async fn insert_project_into_db(
    transaction: &mut Transaction<'static, MySql>,
    project: &Project,
) -> Result<u64, sqlx::Error> {
    let query = sqlx::query!(
        r#"
            INSERT INTO user_project (
            name,
            openstack_id,
            user_class
            )
            VALUES (?, ?, ?)
        "#,
        project.name,
        project.openstack_id,
        project.user_class,
    );
    transaction
        .execute(query)
        .await
        .map(|result| result.last_insert_id())
}

pub async fn insert_user_into_db(
    transaction: &mut Transaction<'static, MySql>,
    user: &User,
) -> Result<u64, sqlx::Error> {
    let query = sqlx::query!(
        r#"
            INSERT INTO user_user (
            password,
            name,
            openstack_id,
            project_id,
            role,
            is_staff,
            is_active
            )
            VALUES ("", ?, ?, ?, ?, ?, ?)
        "#,
        user.name,
        user.openstack_id,
        user.project,
        user.role,
        user.is_staff,
        user.is_active,
    );
    transaction
        .execute(query)
        .await
        .map(|result| result.last_insert_id())
}

pub fn random_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn random_alphanumeric_string(length: usize) -> String {
    rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn random_number(range: Range<u32>) -> u32 {
    rng().random_range(range)
}

pub fn random_bool() -> bool {
    rng().random_bool(0.5)
}
