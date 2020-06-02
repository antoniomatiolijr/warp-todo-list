use crate::{data::*, db, error::Error::*, DBPool, Result};
use serde_derive::Deserialize;
use warp::{http::StatusCode, reject, reply::json, Reply};

pub async fn health_handler(db_pool: DBPool) -> Result<impl Reply> {
    let db = db::get_db_con(&db_pool)
        .await
        .map_err(|e| reject::custom(e))?;

    db.execute("SELECT 1", &[])
        .await
        .map_err(|e| reject::custom(DBQueryError(e)))?;
    Ok(json(&HealthResponse {
        message: String::from("Deu bom"),
    }))
}
