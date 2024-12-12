
use sea_orm::*;
use axum::{extract::{Json,Extension},http:: StatusCode,};
use crate::{entities::prelude::LenderLoanProductLedger, loan_products};
use serde_json::Value;
use chrono::{NaiveDateTime,Duration,NaiveTime,NaiveDate,DateTime};
use crate::loan_products::Model;
use serde::{Deserialize, Serialize};
use crate::State;
use sea_orm::DatabaseConnection; 
use axum::response:: IntoResponse;
use std::sync::Arc;

use crate::entities::prelude::LoanProducts;
use sea_orm::EntityTrait;
use crate::contract::payment_schedule::{Loan,Payment,Ledger};


#[derive(Deserialize,Serialize, Debug)]
pub struct LoanProductLedger {
    pub product_ledger_id: i32,
    pub product_id: i32,
    pub payment_date: NaiveDateTime,
    pub payment_amount: f32,
}


pub async fn generate_loan_schedules(Extension(state): Extension<Arc<State>>)->impl IntoResponse{

let state=state.db.clone();
let db = &state as &DatabaseConnection;


let mut payments=Vec::new();


let loan_product_ledger= LenderLoanProductLedger::find()

.from_raw_sql(Statement::from_sql_and_values(

DbBackend::MySql,
r#"SELECT * FROM loan_system.LenderLoanProductLedger"#,
[],

))
.all(db)
.await.map_err(|err|{StatusCode::INTERNAL_SERVER_ERROR});


if let Ok(loan_ledger) =loan_product_ledger{
let  loan_ledger_payments: Vec<Payment> = loan_ledger.into_iter().map(|b| 

    {
             Payment {

                product_ledger_id: b.product_ledger_id,    
                payment_date: b.payment_date,
                payment_amount :b.payment_amount
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



let mut res_loan_product: Vec<Model>  =vec![];

let loan_product:Result<Option<Model>, StatusCode> = LoanProducts::
find_by_id(1).one(db)
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



let ledger  =Ledger::new(loan_product_info);

let  gen =ledger.complete_schedule();


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