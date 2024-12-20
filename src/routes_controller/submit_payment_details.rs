
use axum::extract::{Json,Extension};
use sea_orm::EntityTrait;
use sea_orm::DatabaseConnection;  
use serde_json::Value;
//serde_json::Value>

use crate::entities::payment_details;
use crate::entities::sea_orm_active_enums::{SourceType,Currency};
use crate::contract::change_in_loan_amount_on_service_fee::change_in_loan_amount_on_service_fee;

use crate::State;
use sea_orm::*;
use axum:: extract;
use crate::entities::prelude::LoanTransactions;


use serde_derive::Deserialize;

use::std::sync::Arc;

#[derive(Deserialize,Debug)]
pub struct PaymentDetails{
    pub payment_id: i32,
    pub product_id:i32,
    pub transaction_id: i32,    
    pub source_type : SourceType,
    pub description:String,                   
    pub gross_amount :f32,                        
    pub currency :Currency,  
    }


//Json<Value>
pub async fn submit_payment_details(Extension(state): Extension<Arc<State>>,extract::Json(payload):extract::Json<PaymentDetails>)->(){

let state=state.db.clone();
let db = &state as &DatabaseConnection;





 




// let insert_loan =payment_details::ActiveModel {

//     payment_id: ActiveValue::Set(payload.payment_id),
//     transaction_id:ActiveValue::Set(payload.transaction_id),
//     source_type:ActiveValue::Set(payload.source_type.clone()),
//     description:ActiveValue::Set(Some(payload.description.to_string())),
//     gross_payment_amount:ActiveValue::Set(payload.gross_amount),
//     service_fee_deducted:ActiveValue::Set(active_service_fee_deducted),
//     net_payment_amount:ActiveValue::Set(net_amount),
//     // net_savings :ActiveValue::Set(()),
//     currency :ActiveValue::Set(payload.currency.clone()),
   
//     ..Default::default() // all other attributes are `NotSet`
// };



// let res = payment_details::Entity::insert(insert_loan).exec(db).await.unwrap();



// Json( serde_json::json!({
//                    "payment_id":payload.payment_id,
//                    "transaction_id":payload.transaction_id,
//                    "source_type":payload.source_type,
//                    "description":payload.description,
//                    "gross_amount":payload.gross_amount,
//                    "service_fee_deducted":active_service_fee_deducted,
//                    "net_amount":net_amount,
//                    "currency":payload.currency,
//                 }))
 

}