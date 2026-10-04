//! Handler preferensi pengguna (tingkat pedas, alergi, anggaran).

use axum::{extract::Path, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Preferensi {
    #[serde(rename = "penggunaId")]
    pub pengguna_id: String,
    #[serde(rename = "levelPedasMaks")]
    pub level_pedas_maks: u8,
    pub alergi: Vec<String>,
    #[serde(rename = "anggaranMaks")]
    pub anggaran_maks: i64,
}

fn bawaan(id: &str) -> Preferensi {
    Preferensi {
        pengguna_id: id.to_string(),
        level_pedas_maks: 3,
        alergi: vec![],
        anggaran_maks: 50000,
    }
}

pub async fn baca(Path(id): Path<String>) -> Json<Preferensi> {
    // Nilai bawaan agar kontrak API jelas sejak hari pertama;
    // implementasi database `pawonee` via sqlx mengikuti migrasi ./migrations.
    Json(bawaan(&id))
}

pub async fn simpan(Path(id): Path<String>, Json(masuk): Json<Preferensi>) -> Json<Preferensi> {
    let mut p = masuk;
    p.pengguna_id = id;
    // Penyimpanan upsert ke tabel `preferensi` mengikuti migrasi ./migrations.
    Json(p)
}
