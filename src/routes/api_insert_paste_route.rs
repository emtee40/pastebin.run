// SPDX-FileCopyrightText: 2021 - 2023 Konrad Borowski <konrad@borowski.pw>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::models::paste::{self, ExtraPasteParameters, InsertionError};
use crate::models::Cors;
use crate::Db;
use chrono::Utc;
use iso8601_duration::Duration;
use rocket::form::{self, Error, Form, FromFormField, ValueField};

#[derive(FromForm)]
pub struct PasteForm {
    #[field(default = Expiration(None))]
    expiration: Expiration,
    #[field(default = "plaintext")]
    language: String,
    code: String,
}

struct Expiration(Option<Duration>);

impl<'r> FromFormField<'r> for Expiration {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Ok(Self(Some(Duration::parse(field.value).map_err(|_| {
            Error::validation("invalid ISO 8601 duration")
        })?)))
    }
}

#[post("/api/v1/pastes", data = "<form>")]
pub async fn api_insert_paste(
    db: Db,
    form: Form<PasteForm>,
) -> Result<Cors<String>, InsertionError> {
    let identifier = db
        .run(move |conn| {
            paste::insert(
                conn,
                form.expiration.0.map(|expiration| Utc::now() + expiration),
                &form.language,
                &form.code,
                ExtraPasteParameters {
                    stdin: "",
                    output: None,
                    exit_code: None,
                },
            )
        })
        .await?;
    Ok(Cors(identifier))
}
