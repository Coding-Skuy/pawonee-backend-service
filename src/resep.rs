//! Handler resep: rekomendasi dari bank resep grade-2.

use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bahan {
    pub nama: String,
    #[serde(default)]
    pub jumlah: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resep {
    pub id: String,
    pub nama: String,
    pub bahan: Vec<Bahan>,
    pub langkah: Vec<String>,
    pub menit: u32,
    #[serde(rename = "levelPedas")]
    pub level_pedas: u8,
    #[serde(rename = "estimasiBiaya")]
    pub estimasi_biaya: i64,
    #[serde(default = "grade_bawaan")]
    pub grade: u8,
}

fn grade_bawaan() -> u8 {
    2
}

/// Bank resep bawaan (cermin ringkas `models/bank_resep_grade2.json` di pawonee-ai-models).
/// Data penuh tinggal di database `pawonee`; ini menjamin endpoint hidup tanpa DB.
fn bank_bawaan() -> Vec<Resep> {
    vec![
        Resep {
            id: "tempe-tumis-bawang".into(),
            nama: "Tempe Tumis Bawang".into(),
            bahan: vec![
                Bahan { nama: "tempe".into(), jumlah: "1 papan".into() },
                Bahan { nama: "bawang".into(), jumlah: "5 siung".into() },
            ],
            langkah: vec![
                "Iris tempe dan bawang.".into(),
                "Tumis bawang hingga harum, masukkan tempe.".into(),
                "Bumbui garam dan kecap, masak 8 menit.".into(),
            ],
            menit: 15,
            level_pedas: 1,
            estimasi_biaya: 12000,
            grade: 2,
        },
        Resep {
            id: "sayur-bening-bayam".into(),
            nama: "Sayur Bening Bayam".into(),
            bahan: vec![
                Bahan { nama: "bayam".into(), jumlah: "1 ikat".into() },
                Bahan { nama: "bawang".into(), jumlah: "3 siung".into() },
            ],
            langkah: vec![
                "Didihkan 600 ml air.".into(),
                "Masukkan bawang, lalu bayam.".into(),
                "Bumbui dan masak 5 menit.".into(),
            ],
            menit: 10,
            level_pedas: 0,
            estimasi_biaya: 8000,
            grade: 2,
        },
    ]
}

pub async fn rekomendasi(Query(params): Query<HashMap<String, String>>) -> Json<Vec<Resep>> {
    let dapur: Vec<String> = params
        .get("bahan")
        .map(|s| s.split(',').map(|b| b.trim().to_lowercase()).collect())
        .unwrap_or_default();
    let hasil = bank_bawaan()
        .into_iter()
        .filter(|r| {
            dapur.is_empty()
                || r.bahan
                    .iter()
                    .any(|b| dapur.iter().any(|d| d == &b.nama.to_lowercase()))
        })
        .collect();
    Json(hasil)
}
