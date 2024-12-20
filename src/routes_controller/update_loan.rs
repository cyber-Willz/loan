
use axum::extract::{Json,Extension};

use sea_orm::DatabaseConnection;  
use serde_json::Value;
//serde_json::Value>

use crate::entities::loan_products;
use crate::contract::applied_fees::applied_fees;
use crate::contract::change_in_loan_amount_on_service_fee::change_in_loan_amount_on_service_fee;
use crate::State;
use sea_orm::*;
use axum:: extract;

use serde::Deserialize;
use::std::sync::Arc;


#[derive(Deserialize,Debug)]                            
pub struct UpdateLoanProduct{
        pub product_id:i32,
        pub lender_id :i32,
        pub product_name:String, 
        pub original_loan_amount:f32,
        pub loan_amount:f32,
        pub number_of_months:f32,
        pub interest_rate:f32, 
        pub  product_description:String
    }

    
pub async fn update_loan(Extension(state): Extension<Arc<State>>,extract::Json(payload):extract::Json<UpdateLoanProduct>)-> Json<Value>{
    let state=state.db.clone();
    let db = &state as &DatabaseConnection;

let loan_amount_service_fee =change_in_loan_amount_on_service_fee(payload.loan_amount.clone()).await;
let loan_service_fee =loan_amount_service_fee.delta_loan_amount+payload.loan_amount;
let output_loan =applied_fees( payload.original_loan_amount ,payload.loan_amount,  loan_service_fee,payload.number_of_months, payload.interest_rate).await;
 

// Into ActiveModel


   let update_loan = loan_products::ActiveModel {
    product_id:ActiveValue::NotSet,
    lender_id: ActiveValue::Set(payload.lender_id),
    product_name: ActiveValue::Set(payload.product_name.to_owned()),
    original_loan_amount:ActiveValue::Set(output_loan.original_loan_amount),
    loan_amount:ActiveValue::Set(output_loan.loan_amount),
    number_of_months:ActiveValue::Set(output_loan.number_of_months),
    loan_amount_service_fee:ActiveValue::Set(loan_service_fee),
     interest_rate:ActiveValue::Set(output_loan.interest_rate),
     monthly_payment:ActiveValue::Set(output_loan.monthly_payment),
     total_interest:ActiveValue::Set(output_loan.total_interest),
     total_principal_interest:ActiveValue::Set(output_loan.total_principal_interest),
     product_description:ActiveValue::Set(payload.product_description.to_owned()),
    ..Default::default() // all other attributes are `NotSet`
}; 

let res = update_loan.update(db).await.unwrap();

  Json( serde_json::json!({
        "product_id":res.product_id,
       "lender_id":res.lender_id,
       "product_name":res.product_name,
       "original_loan_amount":res.original_loan_amount,
       "loan_amount":res.loan_amount,
       "number_of_months":res.number_of_months,
        "interest_rate":res.interest_rate,
        "monthly_payment":res.monthly_payment,
        "total_interest":res.total_interest,
        "total_principal_interest":res.total_principal_interest,
         "product_description":res.product_description
         
            }))


          

        
}


