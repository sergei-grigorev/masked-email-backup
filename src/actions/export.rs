use crate::model::masked_email::MaskedEmail;

pub fn export_tsv(emails: &[MaskedEmail]) {
    // print headers
    println!("EMAIL\tSITE\tDESCRIPTION\tSTATUS");

    // print records
    for email in emails {
        println!(
            "{}\t{}\t{}\t{}",
            email.email,
            email.web_site.as_deref().unwrap_or_default(),
            email.description.as_deref().unwrap_or_default(),
            email.state
        )
    }
}
