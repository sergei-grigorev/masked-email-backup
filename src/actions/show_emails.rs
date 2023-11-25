use std::{borrow::Cow, sync::Arc};

use skim::{
    prelude::{unbounded, SkimOptionsBuilder},
    Skim, SkimItem, SkimItemReceiver, SkimItemSender,
};

use crate::model::masked_email::MaskedEmail;

struct WrappedMaskedEmail {
    id: String,
    email: String,
    domain: String,
    description: String,
}

impl SkimItem for WrappedMaskedEmail {
    fn text(&self) -> skim::prelude::Cow<str> {
        Cow::Owned(format!(
            "[{}] \"{}\" | \"{}\"",
            self.email, self.domain, self.description
        ))
    }

    fn output(&self) -> Cow<str> {
        Cow::Borrowed(self.id.as_str())
    }
}

pub fn interact(emails: &[MaskedEmail]) {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .build()
        .unwrap();

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();
    for email in emails {
        tx.send(Arc::new(WrappedMaskedEmail {
            id: email.internal_id.clone(),
            email: email.email.clone(),
            domain: email.web_site.as_deref().unwrap_or_default().to_owned(),
            description: email.description.as_deref().unwrap_or_default().to_owned(),
        }))
        .unwrap();
    }
    drop(tx);

    // `run_with` would read and show items from the stream
    let selected_items = Skim::run_with(&options, Some(rx))
        .map(|out| out.selected_items)
        .unwrap_or_default();

    println!();

    for item in selected_items.iter() {
        let item_id = item.output();

        if let Some(email) = emails.iter().find(|e| e.internal_id == item_id) {
            println!("{:#?}", email);
        }
    }
}
