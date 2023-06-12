use crate::{models, ItemsDb, Result};
use warp::{http::StatusCode, reply, Reply};

pub async fn get_shopping_list_items(items_db: ItemsDb) -> Result<impl Reply> {
    let local_db = items_db.lock().await;
    let local_db: Vec<models::ShoppingListItem> = local_db.values().cloned().collect();
    Ok(reply::with_status(reply::json(&local_db), StatusCode::OK))
}