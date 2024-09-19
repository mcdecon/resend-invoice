mod file_reader;
mod invoice_repository;

use std::{env, path::Path};

use dotenv::dotenv;
use file_reader::get_invoice_ids;
use invoice_repository::InvoiceHeaderRepository;

#[tokio::main]
async fn main() -> Result<(), String> {
	dotenv().ok();

	let path_str = env::var("FILE_PATH").expect("FILE_PATH must be set.");
	let path = Path::new(&path_str);

	let invoice_ids = get_invoice_ids(path).map_err(|e| e.to_string())?;

	let conn_string = env::var("CONNECTION_STRING").expect("CONNECTION_STRING env var must be set");
	let base_url = env::var("BASE_URL").expect("BASE_URL env var must be set ");

	let mut invoice_header_repository = InvoiceHeaderRepository::new(
		&conn_string,
		&base_url
	).await
	.map_err(|e| e.to_string())?;

	let update810_result = invoice_header_repository.update810_flags(&invoice_ids).await;

	if update810_result.is_err() {
		println!("Updating EDI_810 Flags failed, cancelling SendInvoice calls");
		return update810_result;
	}

	invoice_header_repository.send_invoices(&invoice_ids).await?;
	
	Ok(())
}
