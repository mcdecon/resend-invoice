

use std::collections::HashMap;

use tiberius::{error::Error, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

pub struct InvoiceHeaderRepository<'a> {
	db_client: Client<Compat<TcpStream>>,
	http_client: reqwest::Client,
	base_url: &'a str
}

impl<'a> InvoiceHeaderRepository<'a> {
	pub async fn new(conn_string: &str, base_url: &'a str) -> Result<InvoiceHeaderRepository<'a>, Error> {
		let config = Config::from_ado_string(conn_string)?;
		let tcp = TcpStream::connect(config.get_addr()).await?;
		tcp.set_nodelay(true)?;
		
		let db_client = Client::connect(config, tcp.compat_write()).await?;

		let http_client = reqwest::Client::new();

		Ok(
			InvoiceHeaderRepository {
				db_client,
				http_client,
				base_url
			}
		)
	}

	pub async fn update810_flags(&mut self, invoice_ids: &[i32]) -> Result<(), String> {
		for id in invoice_ids {
			let sql = format!(r#"
				update dbo.INVOICE_HEADER
				set EDI_810SENT = 0,
					EDI_810SENTDT = getdate()
				where INVOICEID = {id}
			"#);

			let result = self.db_client.execute(sql, &[])
				.await
				.map_err(|e| format!("Error updating EDI_810SENT for invoiceId {} error: {}", id, e))?;

			println!("Successfully Updated EDI_810SENT invoiceId {id}");
		}
		println!("o-----------------------------------------------o");
		println!("| Completed updating all EDI_810SENT flags to 0 |");
		println!("o-----------------------------------------------o");

		Ok(())
	}

	pub async fn send_invoices(&mut self, invoice_ids: &[i32]) -> Result<(), String> {
		for id in invoice_ids {
			let url = format!("{}/SendInvoice/{id}", self.base_url);

			println!("{}", url);
			println!("SendInvoice InvoiceId: {id}");
			
			let res = self.http_client.post(url)
				.json(&HashMap::from([
					("InvoiceId", id)
				]))
				.send()
				.await
				.map_err(|e| format!("Error sending invoice for invoiceId {} error: {}", id, e))?;

			let response_text = res.text().await.map_err(|e| e.to_string())?;
			
			println!("Response: {}", response_text);
		}

		Ok(())
	}
}