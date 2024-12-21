
use axum::{extract::{Json,Extension},http:: StatusCode};
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
use crate::entities::prelude::{LoanTransactions,LoanProducts,BorrowerPaymentLedger};
use crate::loan_products::Model;

use serde_derive::{Deserialize,Serialize};

use::std::sync::Arc;

use crate::contract::borrower_schedule::{Payment};
use chrono::{NaiveDateTime,NaiveTime,NaiveDate};

#[derive(Deserialize,Debug,Clone)]
pub struct PaymentDetails{
    pub payment_id: i32,
    pub product_id:i32,
    pub borrower_id:i32,
    pub transaction_id: i32,    
    pub source_type : SourceType,
    pub description:String,                   
    pub gross_amount :f32,                        
    pub currency :Currency,  
    }
    #[derive(Deserialize,Serialize, Debug,Clone)]
    pub struct Loan {
    pub  product_id: i32,
    pub  product_name: String,
    pub  loan_amount: f32,
    pub  interest_rate: f32,
    pub  number_of_months: f32,
    pub  monthly_payment: f32,
    pub  total_interest : f32,
    pub total_principal_interest:f32,
    pub  payments: Vec<Payment>,

  
    }

    pub struct Ledger {

        pub    loans: Arc<Vec<Loan>>,
              
        }
           
        impl Ledger {
        
        pub  async  fn new(loans: Arc<Vec<Loan>>) -> Self {
        
        Self {
             loans: loans,
                       
             }
        }
        
        
        pub async fn loan_principal(&self) -> f32 {
        
            let ans  = self.loans.clone();
            let loan =ans[0].clone();
            let new = loan.loan_amount;
            new
            
            }
        
        pub async fn total_interest(&self) -> f32 {
        
                let ans  = self.loans.clone();
                let loan =ans[0].clone();
                let new = loan.total_interest;
                new
                
                }    
        
        pub async fn total_principal_interest(&self) -> f32 {
        
            let ans  = self.loans.clone();
            let loan =ans[0].clone();
            let new = loan.total_principal_interest;
            new
            
            }
            
           
        pub async fn total_paid(&self) -> f32 {
        
        let ans  = self.loans.clone();
        let loan =ans[0].clone();
        let new  = loan.payments.iter().map(|p| p.payment_amount).sum();
        new
        
        }
           
           
        pub async fn outstanding_balance(&self) -> f32 {
        
        let ans  = self.loans.clone();
        let loan =ans[0].clone();
        let total_paid = self.total_paid().await;
        let total_due = loan.monthly_payment * loan.number_of_months;
        total_due - total_paid
        
        }

    pub async fn payment_detail_method(&self,payment_details:PaymentDetails) -> (f32,f32,f32) {
        let ans  = self.loans.clone();
        let loan =ans[0].clone();
        let tp = self.total_paid().await;
        let service_fee = change_in_loan_amount_on_service_fee(loan.loan_amount).await;
            //Calculating Equated Monthly Installment(EMI)
    
    let emi: f32 =  loan.loan_amount *loan.interest_rate/(1.0-(loan.interest_rate+1.0).powf(-loan.number_of_months));
    // let total_amount =emi*loan.number_of_months; 
    let total_p_interest = emi*loan.number_of_months-loan.loan_amount;
        let mut service_fee_deducted  = 0.0;
        let mut net_amount=0.0;
        let mut net_savings=0.0;
        if tp <= loan.total_principal_interest {
         if service_fee_deducted <= service_fee.delta_loan_amount+loan.loan_amount {
            service_fee_deducted += payment_details.gross_amount*3.0/100.0;
       }
       else if net_amount<=total_p_interest{
        net_amount+= payment_details.gross_amount-payment_details.gross_amount*3.0/100.0;

            
            }
            else{
                net_savings+=payment_details.gross_amount;
            }
       
(service_fee_deducted,net_amount,net_savings)

        }
    else{
        (0.0,0.0,0.0)
    }}
           
        
        
        pub async fn complete_schedule(&self)-> Value {
        let principal=self. loan_principal().await;
        let total_interest=self.total_interest().await;
        let total_principal_interest =self.total_principal_interest().await;
        // let gen_schedule =self.generate_schedule().await;
        // let total_paid =self.total_paid().await;
        // let outstanding_debt  = self.outstanding_balance().await;
        
        serde_json::json!({
        "principal":principal,
        "total_interest":total_interest,
        "total_principal_interest" : total_principal_interest,
        // "total_paid":total_paid,
        // "outstanding_debt":outstanding_debt,
        // "schedule":gen_schedule,
                  
        })}
            
            
        }
               

//Json<Value>
pub async fn submit_payment_details(Extension(state): Extension<Arc<State>>,extract::Json(payload):extract::Json<PaymentDetails>)->Json<Value>{

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

        if b.borrower_id ==payload.borrower_id {

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


let mut res_loan_product: Vec<Model>  =vec![];
let loan_product= LoanProducts::find_by_id(payload.product_id).one(db).await.map_err(|_err|{
    StatusCode::INTERNAL_SERVER_ERROR
  });
 
let ans  = 
if let Ok(product )=loan_product{
if let Some(product)=product
{
    res_loan_product.push(product);

    let  loan_product_info: Vec<Loan> = res_loan_product.into_iter().map(|b| 

        {
            Loan {
       
                product_id: b.product_id,
                product_name: b.product_name,
                loan_amount: b.loan_amount,
                interest_rate: b.interest_rate,
                number_of_months: b.number_of_months,
                monthly_payment: b.monthly_payment,
                total_interest : b.total_interest,
                total_principal_interest: b.total_principal_interest,
                payments: payments.clone()
              
    
                }
    }).collect();
    
    let new_loan_info: Arc<Vec<Loan>> =Arc::new(loan_product_info);
    
    let ledger  =Ledger::new(new_loan_info);
    
    let  gen =ledger.await.payment_detail_method(payload.clone()).await;

let insert_loan =payment_details::ActiveModel {

    payment_id: ActiveValue::Set(payload.payment_id),
    product_id:ActiveValue::Set(payload.product_id),
    transaction_id:ActiveValue::Set(payload.transaction_id),
    source_type:ActiveValue::Set(payload.source_type.clone()),
    description:ActiveValue::Set(Some(payload.description.to_string())),
    gross_payment_amount:ActiveValue::Set(payload.gross_amount),
    service_fee_deducted:ActiveValue::Set(gen.0),
    net_payment_amount:ActiveValue::Set(gen.1),
    net_savings :ActiveValue::Set(gen.2),
    currency :ActiveValue::Set(payload.currency),
   
    ..Default::default() // all other attributes are `NotSet`
};



let res = payment_details::Entity::insert(insert_loan).exec(db).await.unwrap();



let res =serde_json::json!({
        "Insert":"Was successful"});

  Json(res)

}
else {
  
let res =serde_json::json!({
"500":"Not Found"});

Json(res)

}
}else if let Err(res)=loan_product{

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