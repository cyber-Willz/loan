
use axum::{extract::{Json,Extension,Path},http:: StatusCode,};
use axum::extract;
use crate::{entities::prelude::BorrowerPaymentLedger};
use serde_json::Value;
use chrono::{NaiveDateTime,NaiveTime,NaiveDate};
use crate::loan_products::Model;
use serde::{Deserialize, Serialize};
use crate::State;
use sea_orm::DatabaseConnection; 
use axum::response:: IntoResponse;
use std::sync::Arc;

use crate::entities::prelude::LoanProducts;
use sea_orm::EntityTrait;
use crate::contract::borrower_schedule::{Loan,Payment,Ledger};


#[derive(Deserialize,Serialize, Debug)]
pub struct BorrowerInsert {
   pub  borrower_id: i32,
   pub  product_id: i32,
}

pub async fn borrower_loan_schedule(Extension(state): Extension<Arc<State>>,extract::Json(payload):extract::Json<BorrowerInsert>)->impl IntoResponse{

let borrower_index = BorrowerInsert{ borrower_id:payload.borrower_id, product_id:payload.product_id };

    // You do not need a match block to destructure structs:
let BorrowerInsert{  borrower_id ,product_id } = borrower_index;   
let borrower_id  = borrower_id;



let state=state.db.clone();
let db = &state as &DatabaseConnection;


let mut payments=Vec::new();


let borrower_loan_ledger=BorrowerPaymentLedger::find()
.all(db)
.await.map_err(|err|{StatusCode::INTERNAL_SERVER_ERROR});



let d = NaiveDate::from_ymd_opt(2011, 09, 11).unwrap();
let t: NaiveTime = NaiveTime::from_hms_milli_opt(00, 00, 00, 00).unwrap();
let err_datetime = NaiveDateTime::new(d, t);

if let Ok(loan_ledger) =borrower_loan_ledger{
 
let  loan_ledger_payments: Vec<Payment> = loan_ledger.into_iter().map(|b| 

    {

        if b.borrower_id == borrower_id {

            Payment {

                ledger_id: b.ledger_id,    
                payment_date: b.payment_date,
                payment_amount :b.payment_amount
                }
                    
        }
        else{

            Payment {
                ledger_id: b.ledger_id,    
                payment_date: err_datetime,
                payment_amount :0.0
                }
        }
            
            
    }).collect();

for elem in loan_ledger_payments.into_iter() {
payments.push(elem)


}
}

//Note to self Use a find by id on loan_requests to create an authentication logic to only allow approved borrowers to 
// generate an loan_amortization_payments
let d = NaiveDate::from_ymd_opt(2024, 12, 1).unwrap();
let t: NaiveTime = NaiveTime::from_hms_milli_opt(00, 00, 00, 00).unwrap();
let start_datetime = NaiveDateTime::new(d, t);


let product_id  = product_id;

let mut res_loan_product: Vec<Model>  =vec![];

let loan_product:Result<Option<Model>, StatusCode> = LoanProducts::
find_by_id(product_id).one(db)
.await.map_err(|err|{StatusCode::INTERNAL_SERVER_ERROR});


if let Ok(loan_product)=loan_product{
if let Some(loan_product) =loan_product{

res_loan_product.push(loan_product);


let  loan_product_info: Vec<Loan> = res_loan_product.into_iter().map(|b| 

    {
        Loan {
   
            product_id: b.product_id,
            product_name: b.product_name,
            loan_amount: b.loan_amount,
            interest_rate: b.interest_rate,
            number_of_months: b.number_of_months,
            start_date: start_datetime ,
            payments: payments.clone(),

            }
}).collect();

let new_loan_info: Arc<Vec<Loan>> =Arc::new(loan_product_info);

let ledger  =Ledger::new(new_loan_info);

let  gen =ledger.await.complete_schedule().await;


let mut  vec =Vec::new();
vec.push(gen);

Json(vec)

   
}

else {

let mut  vec =Vec::new();

let res =serde_json::json!({
            "500":"Not Found"});
            vec.push(res);
    
Json(vec)
}


}

else if let Err(_loan_product_err)=loan_product{

let mut  vec =Vec::new();

let res =serde_json::json!({
            "500":"INTERNAL_SERVER_ERROR"});
            vec.push(res);
    
Json(vec)

}

else {


let mut  new_err =Vec::new();

let err_res =serde_json::json!({
        "401":"UNAUTHORIZED"});
        new_err.push(err_res);

Json(new_err)

}



}