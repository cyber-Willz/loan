
use axum::extract::{Json, Path,Extension};
use axum::http::StatusCode;
use crate::State;
use sea_orm::*;
use sea_orm::DatabaseConnection;  
use crate::entities::loan_products;
use std::sync::Arc;



pub async fn delete_loan(Extension(state): Extension<Arc<State>>,Path(product_id): Path<i32>)->Json<serde_json::Value>{

let state=state.db.clone();
let db = &state as &DatabaseConnection;

let delete_loan=  loan_products::ActiveModel {
product_id: ActiveValue::Set(product_id), // The primary key must be set
..Default::default()

    };


let res = delete_loan.delete(db).await;
    

let ans  = 
if let Ok(res)=res {
if let(res)=res
{
    
let deleted_res =serde_json::json!({
"StatusCode::201":"Value was delete from the database"});

Json(deleted_res)

}
else {
  
let res =serde_json::json!({
"500":"Not Found"});

Json(res)

}
}else if let Err(res)=res{

let Db_res =serde_json::json!({
"500":"INTERNAL_SERVER_ERROR"});

Json( Db_res)

}
else{

let err_res =serde_json::json!({
"401":"UNAUTHORIZED"});

Json(err_res)

  };
 
 ans


}