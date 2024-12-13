
 
use chrono::{NaiveDateTime,Duration}; // For date handling
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

#[derive(Deserialize,Serialize, Debug,Clone)]
pub struct Payment {

pub product_ledger_id: i32,    
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
    
   
pub async fn total_gain(&self) -> f32 {

let ans  = self.loans.clone();
let loan =ans[0].clone();
let new  = loan.payments.iter().map(|p| p.payment_amount).sum();
new

}
   
pub async fn monthly_payment(&self) -> f32 {

let ans  = self.loans.clone();
let loan =ans[0].clone();
let r = loan.interest_rate / 12.0 / 100.0; // Monthly interest rate
let n = loan.number_of_months ; // Number of payments
let p = loan.loan_amount;

   
if r == 0.0 {

return p / n; // No interest

}
   
let factor = (1.0 + r).powf(n);
           (p * r* factor) / (factor - 1.0)
          
}
   
   
pub async fn outstanding_balance(&self) -> f32 {

let ans  = self.loans.clone();
let loan =ans[0].clone();
let total_paid = self.total_gain().await;
let total_due = self.monthly_payment().await * loan.number_of_months;
total_due - total_paid

}
   

pub async fn generate_schedule(&self) -> Vec<Value> {

let ans  = self.loans.clone();
let loan =ans[0].clone();
let mut schedule = Vec::new();
let mut balance = loan.loan_amount;
let monthly_payment = self.monthly_payment().await;
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
        
let gen_schedule =self.generate_schedule().await;
let total_gain =self.total_gain().await;
let outstanding_gain  = self.outstanding_balance().await;

serde_json::json!({
"total_gain":total_gain,
"outstanding_gain":outstanding_gain,
"schedule":gen_schedule,
          
})}
    
    
}
       

