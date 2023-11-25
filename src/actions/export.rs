use crate::model::masked_email::MaskedEmail;

const EMPTY_VALUE: &str = "";

pub fn export_tsv(emails: &[MaskedEmail]) -> () {
    // print headers
    println!("EMAIL\tSITE\tDESCRIPTION\tSTATUS");

    // print records
    for email in emails {
        println!(
            "{}\t{}\t{}\t{}",
            email.email,
            email
                .web_site
                .as_ref()
                .map(String::as_str)
                .unwrap_or(EMPTY_VALUE),
            email
                .description
                .as_ref()
                .map(String::as_str)
                .unwrap_or(EMPTY_VALUE),
            email.state
        )
    }
}
