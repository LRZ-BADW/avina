use actix_web::{
    HttpResponse, Scope,
    web::{ReqData, get, scope},
};
use anyhow::Context;
use avina_wire::{bill::Bill, user::User};
use tera::{Context as TeraContext, Tera};
use typst_as_lib::TypstEngine;

use crate::{authorization::require_admin_user, error::NormalApiError};

pub fn bill_scope() -> Scope {
    scope("/bill").route("", get().to(bill_get))
}

#[tracing::instrument(name = "bill_get")]
async fn bill_get(user: ReqData<User>) -> Result<HttpResponse, NormalApiError> {
    require_admin_user(&user)?;

    let bill_template = include_str!("../../assets/bill.typ.j2");
    let mut tera = Tera::default();
    tera.add_raw_template("bill", bill_template)
        .context("Failed to add template.")?;

    let mut context = TeraContext::new();
    context.insert("preliminary", "true");
    context.insert("compact", "false");
    context.insert("organization", "Organization");
    context.insert("institute", "Institute");
    context.insert("street_and_number", "Street 123");
    context.insert("postcode", "12345");
    context.insert("city", "City");
    context.insert("person_address", "Person Address");
    context.insert("letter_address", "Letter Address");
    context.insert("person_title", "Person Title");
    context.insert("prename", "Prename");
    context.insert("surname", "Surname");
    context.insert("addr_recipient", "Address Recipient");
    context.insert("recipient", "Recipient");
    context.insert("project", "ab12cde");
    context.insert("project_name", "Project Name");
    context.insert("bookkeeping_number", "abc123");
    context.insert("now_day", "01");
    context.insert("now_month", "Januar");
    context.insert("now_year", "2025");
    context.insert("bill_day", "01");
    context.insert("bill_month", "Januar");
    context.insert("bill_year", "2025");
    context.insert("user_class", "1");
    // context.insert("tax_percent", "13");
    // "master_users": master_users,
    // "status": status,
    // "cost": cost,

    let main_typ = tera
        .render("bill", &context)
        .context("Failed to render bill.")?;
    let logo_svg: &[u8] = include_bytes!("../../assets/logo.svg");
    let nimbus_sans_regular_otf: &[u8] =
        include_bytes!("../../assets/nimbus-sans-regular.otf");
    let nimbus_sans_bold_otf: &[u8] =
        include_bytes!("../../assets/nimbus-sans-bold.otf");

    let template = TypstEngine::builder()
        .fonts([nimbus_sans_regular_otf, nimbus_sans_bold_otf])
        .with_static_source_file_resolver([("main.typ", main_typ)])
        .with_static_file_resolver([("logo.svg", logo_svg)])
        .build();

    let doc = template
        .compile("main.typ")
        .output
        .context("Failed to build typst document.")?;
    let options = Default::default();
    // TODO: what about context?
    let pdf = typst_pdf::pdf(&doc, &options)
        .expect("Failed ot build pdf with typst.");

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(Bill { amount: 0.0, pdf }))
}
