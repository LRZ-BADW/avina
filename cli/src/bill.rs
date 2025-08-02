use std::{error::Error, fs::write};

use crate::common::{Format, print_single_object};

pub(crate) async fn bill_get(
    api: avina::Api,
    format: Format,
) -> Result<(), Box<dyn Error>> {
    let bill = api.bill.get().await?;
    write("bill.pdf", bill.pdf.clone())?;
    print_single_object(&bill, format)
}
