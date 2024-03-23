use core::time;
use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use axum::Router;
use diesel::{data_types::PgInterval, prelude::*, sql_types::Time};
use models::Score;
use serde::{Deserialize, Serialize};

use crate::models::NewScore;

mod models;
mod schema;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // tokio::spawn(async move {
    // });

    // let conf = leptos::leptos_config::LeptosOptions::builder()
    //     .build();

    let addr = SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let app = Router::new()
        .route("/", axum::routing::get(leaderboards))
        .route("/submit", axum::routing::get(add_score));
    // .with_state(conf.clone());

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

fn db() -> ConnectionResult<PgConnection> {
    PgConnection::establish(&env::var("DATABASE_URL").unwrap())
}

async fn leaderboards() -> String {
    use self::schema::scores::dsl::*;
    let mut conn = db().unwrap();
    let score_list = scores
        .order(time.desc())
        .limit(10)
        .load::<Score>(&mut conn)
        .unwrap();

    let mut result = String::new();

    for s in score_list.iter() {
        result.push_str(&format!("{}: {}\n", &s.name, s.time.microseconds))
    }
    if score_list.is_empty() {
        "No scores found".into()
    } else {
        result
    }
}

async fn add_score() -> String {
    use self::schema::scores;
    let mut conn = db().unwrap();
    let new_score = NewScore {
        name: "forsen".into(),
        time: PgInterval::from_microseconds(200),
    };

    diesel::insert_into(scores::table)
        .values(&new_score)
        .returning(Score::as_returning())
        .get_result(&mut conn)
        .expect("Error submitting score");

    "submitted".into()
}

// #[leptos::server]
// async fn get_scores() -> Result<Vec<ScoreResponse>, ServerFnError> {
// }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScoreResponse {
    name: String,
    time: time::Duration,
}
