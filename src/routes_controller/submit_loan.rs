
use axum::extract::{Json,Extension};
use sea_orm::EntityTrait;
use sea_orm::DatabaseConnection;  
use serde_json::Value;
//serde_json::Value>

use crate::entities::loan_products;
use crate::contract::applied_fees::applied_fees;
use crate::State;
use sea_orm::*;
use axum:: extract;

use serde::Deserialize;
use serde_derive::Serialize;
use::std::sync::Arc;



#[derive(Deserialize,Serialize,Debug)]
pub struct NewLoanProduct{
    pub lender_id :i32,
    pub product_name:String, 
    pub original_loan_amount:f32,
    pub loan_amount:f32,
    pub number_of_months:f32,
    pub interest_rate:f32, 
    pub monthly_payment:f32,
    pub total_interest:f32, 
    pub total_principal_interest:f32,
    pub  product_description:String
}


#[derive(Deserialize,Debug)]
pub struct CreateLoanProduct{
    pub lender_id :i32,
    pub product_name:String, 
    pub original_loan_amount:f32,
    pub loan_amount:f32,
    pub number_of_months:f32,
    pub interest_rate:f32, 
    pub  product_description:String
    }


pub async fn submit_loan(Extension(state): Extension<Arc<State>>,extract::Json(payload):extract::Json<CreateLoanProduct>)-> Json<Value>{

    let state=state.db.clone();
    let db = &state as &DatabaseConnection;


   let output_loan =applied_fees( payload.original_loan_amount ,payload.loan_amount, payload.number_of_months, payload.interest_rate).await;


let insert_loan = loan_products::ActiveModel {
    lender_id: ActiveValue::Set(payload.lender_id),
    product_name: ActiveValue::Set(payload.product_name.to_owned()),
    original_loan_amount:ActiveValue::Set(payload.original_loan_amount),
    loan_amount:ActiveValue::Set(output_loan.loan_amount),
    number_of_months:ActiveValue::Set(output_loan.number_of_months),
     interest_rate:ActiveValue::Set(output_loan.interest_rate),
     monthly_payment:ActiveValue::Set(output_loan.monthly_payment),
     total_interest:ActiveValue::Set(output_loan.total_interest),
     total_principal_interest:ActiveValue::Set(output_loan.total_principal_interest),
     product_description:ActiveValue::Set(payload.product_description.to_owned()),
    ..Default::default() // all other attributes are `NotSet`
};

let res = loan_products::Entity::insert(insert_loan).exec(db).await.unwrap();





   
    Json( serde_json::json!({
                   "lender_id":payload.lender_id,
                   "product_name":payload.product_name,
                   "original_loan_amount":output_loan.original_loan_amount,
                   "loan_amount":output_loan.loan_amount,
                   "number_of_months":output_loan.number_of_months,
                    "interest_rate":output_loan.interest_rate,
                    "monthly_payment":output_loan.monthly_payment,
                    "total_interest":output_loan.total_interest,
                    "total_principal_interest":output_loan.total_principal_interest,
                     "product_description":payload.product_description
                     
                        }))
 

}