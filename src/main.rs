//! Pawonee backend: API resep + preferensi di atas database `pawonee`.
//! Selaras dengan modul `:shared:pantry-resep` milik Pawonee.

mod preferensi;
mod resep;

use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct Kesehatan {
    status: &'static str,
}

async fn kesehatan() -> Json<Kesehatan> {
    Json(Kesehatan { status: "ok" })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/kesehatan", get(kesehatan))
        .route(
            "/v1/resep/rekomendasi",
            get(resep::rekomendasi),
        )
        .route(
            "/v1/preferensi/:id",
            get(preferensi::baca).put(preferensi::simpan),
        );

    let alamat = SocketAddr::from(([0, 0, 0, 0], 8080));
    let pendengar = tokio::net::TcpListener::bind(alamat).await.unwrap();
    axum::serve(pendengar, app).await.unwrap();
}
