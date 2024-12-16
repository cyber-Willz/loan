
 
use chrono::{NaiveDateTime,Duration}; // For date handling
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;


#[derive(Deserialize,Serialize, Debug,Clone)]
pub struct Payment {

pub ledger_id: i32,    
pub payment_date: NaiveDateTime,
pub payment_amount :f32

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
pub  start_date: NaiveDateTime,
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
   

pub async fn generate_schedule(&self) -> Vec<Value> {

let ans  = self.loans.clone();
let loan =ans[0].clone();
let mut schedule = Vec::new();
let mut balance = loan.loan_amount;
let monthly_payment = loan.monthly_payment;
println!("{:?}",loan.interest_rate);
let rate = loan.interest_rate / 12.0 / 100.0;

   
for i in 0..loan.number_of_months  as u32 {
let date = loan.start_date + Duration::days((i * 30) as i64); // Approximate month
let interest = balance * rate;
let principal = monthly_payment - interest;
balance -= principal;
                
                   

schedule.push( serde_json::json!({
"date":date,
"monthly_payment":monthly_payment,
"principal":principal,
"interest":interest}));

}
   
schedule

}

pub async fn complete_schedule(&self)-> Value {
let principal=self. loan_principal().await;
let total_interest=self.total_interest().await;
let total_principal_interest =self.total_principal_interest().await;
let gen_schedule =self.generate_schedule().await;
let total_paid =self.total_paid().await;
let outstanding_debt  = self.outstanding_balance().await;

serde_json::json!({
"principal":principal,
"total_interest":total_interest,
"total_principal_interest" : total_principal_interest,
"total_paid":total_paid,
"outstanding_debt":outstanding_debt,
"schedule":gen_schedule,
          
})}
    
    
}
       

