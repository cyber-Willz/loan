use axum::{extract::{Json, Path,Extension},http:: StatusCode};
use sea_orm::EntityTrait;
use sea_orm::DatabaseConnection;  
use serde_json::Value;
//serde_json::Value>
use crate::entities::prelude::LoanProducts;

use std::sync::Arc;
use crate::State;



pub async fn find_loan_by_id(Extension(state): Extension<Arc<State>>,Path(product_id): Path<i32>) ->Json<Value>{

let state=state.db.clone();
let db = &state as &DatabaseConnection;

  let res= LoanProducts::find_by_id(product_id).into_json().one(db).await.map_err(|_err|{
    StatusCode::INTERNAL_SERVER_ERROR
  });
 
let ans  = 
if let Ok(res)=res{
if let Some(res)=res
{
  Json(res)

}
else {
  
let res =serde_json::json!({
"500":"Not Found"});

Json(res)

}
}else if let Err(res)=res{

let res =serde_json::json!({
"500":"INTERNAL_SERVER_ERROR"});

Json(res)

}
else{

let err_res =serde_json::json!({
"401":"UNAUTHORIZED"});

Json(err_res)

  };
 
 ans

}