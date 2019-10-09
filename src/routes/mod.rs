mod api_language;
mod api_v1;
mod display_paste;
mod index;
mod insert_paste;
mod raw_paste;
mod run;

use crate::templates::{self, RenderRucte};
use crate::Connection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use futures03::TryFutureExt;
use std::ffi::OsStr;
use std::path::PathBuf;
use tokio_executor::blocking;
use warp::filters::BoxedFilter;
use warp::http::header::{
    HeaderMap, HeaderValue, CONTENT_SECURITY_POLICY, REFERRER_POLICY, X_FRAME_OPTIONS,
};
use warp::http::{Response, StatusCode};
use warp::{path, Filter, Rejection, Reply};

type PgPool = Pool<ConnectionManager<PgConnection>>;

fn connection(pool: PgPool) -> BoxedFilter<(Connection,)> {
    warp::any()
        .and_then(move || {
            let pool = pool.clone();
            blocking::run(move || pool.get().map_err(warp::reject::custom)).compat()
        })
        .boxed()
}

fn index(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    warp::path::end()
        .and(
            warp::post2()
                .and(warp::body::content_length_limit(1_000_000))
                .and(warp::body::form())
                .and(connection(pool.clone()))
                .and_then(insert_paste::insert_paste)
                .or(warp::get2().and(connection(pool)).and_then(index::index)),
        )
        .boxed()
}

fn display_paste(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    warp::path::param()
        .and(warp::path::end())
        .and(warp::get2())
        .and(connection(pool))
        .and_then(display_paste::display_paste)
        .boxed()
}

fn raw_paste(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    with_ext("txt")
        .and(warp::get2())
        .and(connection(pool))
        .and_then(raw_paste::raw_paste)
        .boxed()
}

fn api_v0(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    let root = path!("api" / "v0").and(connection(pool));
    let language = root
        .clone()
        .and(path!("language" / String))
        .and(warp::path::end())
        .and(warp::get2())
        .and_then(api_language::api_language);
    let run = root
        .and(path!("run" / String / String))
        .and(warp::post2())
        .and(warp::body::content_length_limit(1_000_000))
        .and(warp::body::form())
        .and(path!(String))
        .and_then(run::run);
    language.or(run).boxed()
}

fn api_v1_languages(pool: PgPool) -> BoxedFilter<(impl Reply,)> {
    path!("api" / "v1")
        .and(warp::path("languages"))
        .and(warp::path::end())
        .and(warp::get2())
        .and(connection(pool))
        .and_then(api_v1::languages::languages)
        .boxed()
}

fn static_dir() -> BoxedFilter<(impl Reply,)> {
    warp::path("static").and(warp::fs::dir("static")).boxed()
}

fn favicon() -> BoxedFilter<(impl Reply,)> {
    warp::path("favicon.ico")
        .and(warp::path::end())
        .and(warp::fs::file("static/favicon.ico"))
        .boxed()
}

pub fn routes(
    pool: Pool<ConnectionManager<PgConnection>>,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> {
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(concat!(
            "default-src 'none'; ",
            "script-src 'self'; ",
            "style-src 'self' 'unsafe-inline'; ",
            "connect-src 'self'; ",
            "img-src *; ",
            "object-src 'none'; ",
            "base-uri 'none'; ",
            "form-action 'self'; ",
            "frame-ancestors 'none'",
        )),
    );
    headers.insert(X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
    headers.insert(REFERRER_POLICY, HeaderValue::from_static("no-referrer"));
    index(pool.clone())
        .or(favicon())
        .or(raw_paste(pool.clone()))
        .or(display_paste(pool.clone()))
        .or(api_v0(pool.clone()))
        .or(api_v1_languages(pool.clone()))
        .or(static_dir())
        .recover(not_found)
        .with(warp::reply::with::headers(headers))
        .with(warp::log("pastebinrun"))
}

fn with_ext(ext: &'static str) -> impl Filter<Extract = (String,), Error = Rejection> + Copy {
    warp::path::param()
        .and(warp::path::end())
        .and_then(move |path: PathBuf| {
            match (path.extension(), path.file_stem().and_then(OsStr::to_str)) {
                (Some(received_ext), Some(file_stem)) if ext == received_ext => {
                    Ok(file_stem.to_string())
                }
                _ => Err(warp::reject::not_found()),
            }
        })
}

fn not_found(rejection: Rejection) -> Result<impl Reply, Rejection> {
    if rejection.is_not_found() {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .html(|o| templates::not_found(o))
    } else {
        Err(rejection)
    }
}

#[cfg(test)]
mod test {
    use super::routes;
    use diesel::r2d2::{ConnectionManager, CustomizeConnection, Pool};
    use diesel::Connection;
    use lazy_static::lazy_static;
    use scraper::{Html, Selector};
    use serde::Deserialize;
    use std::env;
    use std::str;
    use warp::filters::BoxedFilter;
    use warp::http::header::{CONTENT_LENGTH, LOCATION};
    use warp::reply::{Reply, Response};
    use warp::Filter;

    lazy_static! {
        static ref ROUTES: BoxedFilter<(Response,)> = {
            let pool = Pool::builder()
                .connection_customizer(Box::new(ExecuteWithinTransaction))
                .max_size(1)
                .build(ConnectionManager::new(env::var("DATABASE_URL").expect(
                    "Setting DATABASE_URL environment variable required to run tests",
                )))
                .expect("Couldn't create a connection connection");
            diesel_migrations::run_pending_migrations(&pool.get().unwrap()).unwrap();
            routes(pool).map(Reply::into_response).boxed()
        };
    }

    #[derive(Debug)]
    struct ExecuteWithinTransaction;

    impl<C, E> CustomizeConnection<C, E> for ExecuteWithinTransaction
    where
        C: Connection,
    {
        fn on_acquire(&self, conn: &mut C) -> Result<(), E> {
            conn.begin_test_transaction().unwrap();
            Ok(())
        }
    }

    fn get_sh_id() -> String {
        let response = warp::test::request().reply(&*ROUTES);
        let document = Html::parse_document(str::from_utf8(response.body()).unwrap());
        document
            .select(&Selector::parse("#language option").unwrap())
            .find(|element| element.text().next() == Some("Sh"))
            .expect("a language called Sh to exist")
            .value()
            .attr("value")
            .expect("an ID")
            .to_string()
    }

    #[test]
    fn test_language_api() {
        #[derive(Debug, Deserialize, PartialEq)]
        pub struct ApiLanguage<'a> {
            #[serde(borrow)]
            implementations: Vec<Implementation<'a>>,
        }

        #[derive(Debug, Deserialize, PartialEq)]
        pub struct Implementation<'a> {
            identifier: &'a str,
            label: &'a str,
            #[serde(borrow)]
            wrappers: Vec<Wrapper<'a>>,
        }

        #[derive(Debug, Deserialize, PartialEq)]
        #[serde(rename_all = "camelCase")]
        pub struct Wrapper<'a> {
            identifier: &'a str,
            label: &'a str,
            is_asm: bool,
            is_formatter: bool,
        }

        let response = warp::test::request()
            .path(&format!("/api/v0/language/{}", get_sh_id()))
            .reply(&*ROUTES);
        assert_eq!(
            serde_json::from_slice::<ApiLanguage>(response.body()).unwrap(),
            ApiLanguage {
                implementations: vec![Implementation {
                    identifier: "sh",
                    label: "sh",
                    wrappers: vec![Wrapper {
                        identifier: "run",
                        label: "Run",
                        is_asm: false,
                        is_formatter: false,
                    }],
                }],
            },
        );
    }

    #[test]
    fn test_raw_pastes() {
        let body = format!("language={}&code=abc", get_sh_id());
        let reply = warp::test::request()
            .method("POST")
            .header(CONTENT_LENGTH, body.len())
            .body(body)
            .reply(&*ROUTES);
        let location = reply.headers()[LOCATION].to_str().unwrap();
        assert_eq!(
            warp::test::request()
                .path(&format!("{}.txt", location))
                .reply(&*ROUTES)
                .body(),
            "abc"
        );
    }
}
