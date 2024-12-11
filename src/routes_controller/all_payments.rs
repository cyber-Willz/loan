
use sea_orm::*;
use axum::extract::{Json,Extension};
use sea_orm::EntityTrait;
use crate::entities::prelude::Payments;
use serde_json::Value;
use chrono::NaiveDateTime; // For date handling

use serde::{Deserialize, Serialize};
use crate::State;
use sea_orm::DatabaseConnection; 
use axum::response:: IntoResponse;

use std::sync::Arc;
#[derive(Deserialize,Serialize, Debug)]
struct Payment{
pub transaction_id: i32,    
pub payment_date: NaiveDateTime,
pub payment_amount :f32
}
pub async fn all_payments(Extension(state): Extension<Arc<State>>)->impl IntoResponse{

let state=state.db.clone();
let db = &state as &DatabaseConnection;
    
let payments= Payments::find()

.from_raw_sql(Statement::from_sql_and_values(

        DbBackend::MySql,
        r#"SELECT * FROM loan_system.Payments"#,
        [],

))
.all(db)
.await.unwrap();



 
let  rust_payment: Vec<Payment> = payments.into_iter().map(|b| 

{
Payment{
transaction_id: b.transaction_id,    
payment_date: b.payment_date,
payment_amount :b. payment_amount

}}

).collect();


Json(rust_payment)




}