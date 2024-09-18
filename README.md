## Resend Invoices

Written in rust for it's high degree of safety and compiler checks, as well as blazing speed.

### To use
Clone the repository, install rustup and use that to install the latest toolchain. https://rustup.rs/

Replace the contents of invoice_nums.txt with the list of invoiceIds needed to be resent, be sure to remove the header if you copied it from an excel sheet.

Run `cargo run` to start and keep an eye on the console output to ensure it's running. Each call to /SendInvoice takes about a second so take a coffee break once it's running smoothly.

### TODO
1. Check the response body errorCode instead of just relying on the http status causing a Result::Error to be thrown
2. Collect all failed invoiceIds into a list and output to a file AND/OR add retry logic
3. Batch update810_flags into a single sql script instead of looping
