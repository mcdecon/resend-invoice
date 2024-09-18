use std::io::{Error};
use std::path::Path;
use std::fs;

pub fn get_invoice_ids(path: &Path) -> Result<Vec<i32>, Error> {
	let file_contents = fs::read_to_string(path)?;

	let invoices: Vec<i32> = file_contents
		.lines()
		.filter_map(|line| line.parse::<i32>().ok())
		.collect();

	Ok(invoices)
}