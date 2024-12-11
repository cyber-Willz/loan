use axum::{
    http:: StatusCode,
    extract:: {Extension,Json},
  
};
use crate::entities::prelude::LoanRequests;
use sea_orm::EntityTrait;
use std::sync::Arc;
use crate::State;
use sea_orm::*; 
use serde_json::Value;

pub async fn all_loan_requests(Extension(state): Extension<Arc<State>>)->Json<Vec<Value>>{

let state=state.db.clone();
let db = &state as &DatabaseConnection;


let loan_requests:Result<Vec<JsonValue>, StatusCode> = LoanRequests::find()

.from_raw_sql(Statement::from_sql_and_values(
   
    DbBackend::MySql,
    r#"SELECT * FROM loan_system.LoanRequests"#,
    [],

))

.into_json()
.all(db)
.await.map_err(|err| StatusCode::INTERNAL_SERVER_ERROR);

if let Ok(loan_requests)=loan_requests{
    Json(loan_requests)

}

else if let Err(_err) = loan_requests{

let mut  vec =Vec::new();

let res =serde_json::json!({
        "500":"INTERNAL_SERVER_ERROR"});
        vec.push(res);

Json(vec)

}

else{

    
let mut  new_err =Vec::new();

let err_res =serde_json::json!({
        "401":"UNAUTHORIZED"});
        new_err.push(err_res);

Json(new_err)

}


}