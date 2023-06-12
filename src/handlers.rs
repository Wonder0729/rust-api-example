use crate::{models, ItemsDb, Result};
use warp::{http::StatusCode, reply, Reply};

pub async fn get_shopping_list_items(items_db: ItemsDb) -> Result<impl Reply> {
    let local_db = items_db.lock().await;
    let local_db: Vec<models::ShoppingListItem> = local_db.values().cloned().collect();
    Ok(reply::with_status(reply::json(&local_db), StatusCode::OK))
}

pub async fn create_shopping_list_item(
    mut shopping_list_item: models::ShoppingListItem,
    items_db: ItemsDb,
) -> Result<impl Reply> {
    println!("Received UserData: {:?}", shopping_list_item);
    let mut local_db = items_db.lock().await;
    let key_count = local_db.keys().len();
    shopping_list_item.item_id = Some(key_count);
    local_db.insert(key_count, shopping_list_item.clone());
    Ok(reply::with_status(
        reply::json(&shopping_list_item),
        StatusCode::CREATED,
    ))
}

pub async fn get_shopping_list_item_by_id(id: usize, items_db: ItemsDb) -> Result<impl Reply> {
    let local_db = items_db.lock().await;
    let shopping_list_item = match local_db.get(&id) {
        Some(item) => item,
        _ => {
            return Ok(reply::with_status(
                reply::json(&"{}"),
                StatusCode::NOT_FOUND,
            ));
        }
    };

    Ok(reply::with_status(
        reply::json(&shopping_list_item),
        StatusCode::OK
    ))
}

pub async fn update_shopping_list_item_by_id(
    id: usize,
    updated_data: models::UpdateShoppingListItem,
    items_db: ItemsDb,
) -> Result<impl Reply> {
    let mut local_db = items_db.lock().await;
    let mut shopping_list_item = match local_db.get(&id) {
        Some(item) => item.clone(),
        _ => {
            return Ok(reply::with_status(
                reply::json(&"{}"),
                StatusCode::NOT_FOUND,
            ));
        }
    };

    if let Some(name) = updated_data.name {
        println!("updating name from {} to {}", shopping_list_item.name, name);
        shopping_list_item.name = name;
    };
    if let Some(description) = updated_data.description {
        println!(
            "updating description from {} to {}",
            shopping_list_item.description, description
        );
        shopping_list_item.description = description;
    };
    if let Some(item_type) = updated_data.item_type {
        println!(
            "updating item_type from {:?} to {:?}",
            shopping_list_item.item_type, item_type
        );
        shopping_list_item.item_type = item_type;
    };
    if let Some(price) = updated_data.price {
        println!(
            "updating price from {} to {}",
            shopping_list_item.price, price
        );
        shopping_list_item.price = price;
    };
    *local_db.get_mut(&id).unwrap() = shopping_list_item.clone();
    Ok(reply::with_status(
        reply::json(&shopping_list_item),
        StatusCode::OK,
    ))
}

pub async fn delete_shopping_list_item_by_id(id: usize, items_db: ItemsDb) -> Result<impl Reply> {
    let mut local_db = items_db.lock().await;

    println!("deleting shopping list item with id: {}", id);
    local_db.remove(&id);

    Ok(reply::with_status(
        reply::html("delete success"),
        StatusCode::OK,
    ))
}