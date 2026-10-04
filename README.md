# pawonee-backend-service

Backend Rust Pawonee — **Divisi Pawonee (AI Cooking Assistant)**, org `Coding-Skuy`. Menyajikan API resep dan preferensi di atas database `pawonee`. Bagian dari arsitektur **Opsi A ChefGenie**.

- TownHall: [Coding-Skuy/Pawonee-TownHall](https://github.com/Coding-Skuy/Pawonee-TownHall).
- Model domain selaras dengan modul `:shared:pantry-resep` **milik Pawonee** (sumber: `pawonee-app-kmp/shared/pantry-resep`) dan skema `models/schema.json` di `pawonee-ai-models`.
- Konsumen hilir: **Pedaree** membaca keluaran API/sinyal ini sebagai masukan rekomendasi pendampingnya.

## Teknologi (versi dipin)

| Komponen | Versi |
|---|---|
| Rust | 1.82.0 |
| tokio | 1.42.0 |
| axum | 0.8.1 |
| sqlx (postgres, runtime-tokio) | 0.8.2 |
| serde / serde_json | 1.0.215 / 1.0.133 |
| Postgres | 16.4 |

## Endpoint

- `GET /kesehatan` → `{ "status": "ok" }`
- `GET /v1/resep/rekomendasi?bahan=tempe,bawang` → daftar resep grade-2 yang cocok
- `GET /v1/preferensi/{id}` → preferensi pengguna
- `PUT /v1/preferensi/{id}` → simpan preferensi `{ levelPedasMaks, alergi, anggaranMaks }`

## Cara jalan

```bash
export DATABASE_URL=postgres://pawonee:pawonee@localhost:5432/pawonee
sqlx migrate run   # migrasi di ./migrations memakai database `pawonee`
cargo run
```

Atau via compose di `pawonee-infra-devops` (layanan `db` + `backend`).

## Repo terkait

- [pawonee-app-kmp](https://github.com/Coding-Skuy/pawonee-app-kmp) (pemilik `:shared:pantry-resep`)
- [pawonee-ai-models](https://github.com/Coding-Skuy/pawonee-ai-models) (bank resep grade-2 + skema)
- [pawonee-data-pipeline](https://github.com/Coding-Skuy/pawonee-data-pipeline) (sinyal ke Pedaree)
