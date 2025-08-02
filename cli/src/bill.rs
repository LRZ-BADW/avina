use std::error::Error;

use crate::common::{Format, print_single_object};

pub(crate) async fn bill_get(
    api: avina::Api,
    format: Format,
) -> Result<(), Box<dyn Error>> {
    print_single_object(&api.bill.get().await?, format)
}
