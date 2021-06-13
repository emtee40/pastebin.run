// pastebin.run
// Copyright (C) 2020 Konrad Borowski
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

mod migration;
mod models;
mod schema;

use crate::models::language::Language;
use crate::models::paste::{self, ExtraPasteParameters, InsertionError};
use chrono::{Duration, Utc};
use diesel::PgConnection;
use rocket::fairing::AdHoc;
use rocket::form::Form;
use rocket::fs::{relative, FileServer};
use rocket::response::{Debug, Redirect};
use rocket_dyn_templates::Template;
use rocket_sync_db_pools::database;
use serde::Serialize;

#[database("main")]
struct Db(PgConnection);

#[derive(Serialize)]
struct Index {
    languages: Vec<Language>,
}

#[get("/")]
async fn index(db: Db) -> Result<Template, Debug<diesel::result::Error>> {
    let languages = db.run(|conn| Language::fetch(conn)).await?;
    Ok(Template::render("index", &Index { languages }))
}

#[derive(FromForm)]
pub struct PasteForm {
    language: String,
    code: String,
    share: Share,
    #[field(default = "")]
    stdin: String,
    stdout: Option<String>,
    stderr: Option<String>,
    status: Option<i32>,
}

#[derive(FromFormField)]
pub enum Share {
    Share,
    Share24,
}

#[post("/", data = "<form>")]
async fn insert_paste(db: Db, form: Form<PasteForm>) -> Result<Redirect, InsertionError> {
    let delete_at = match form.share {
        Share::Share => None,
        Share::Share24 => Some(Utc::now() + Duration::hours(24)),
    };
    let identifier = db
        .run(move |conn| {
            paste::insert(
                conn,
                delete_at,
                &form.language,
                &form.code,
                ExtraPasteParameters {
                    stdin: &form.stdin,
                    stdout: form.stdout.as_deref(),
                    stderr: form.stderr.as_deref(),
                    exit_code: form.status,
                },
            )
        })
        .await?;
    Ok(Redirect::to(uri!(display_paste(identifier))))
}

#[get("/<_identifier>")]
async fn display_paste(_identifier: &str) {
    unimplemented!()
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .attach(Db::fairing())
        .attach(AdHoc::on_ignite("Migrations", |rocket| async {
            Db::get_one(&rocket)
                .await
                .expect("a database")
                .run(|conn| {
                    diesel_migrations::run_pending_migrations(conn)?;
                    migration::run(conn)
                })
                .await
                .expect("database to be migrated");
            rocket
        }))
        .mount("/", routes![index, insert_paste, display_paste])
        .mount("/static", FileServer::from(relative!("static")))
}
