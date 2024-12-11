use axum::{
     
    extract:: Extension,
    routing::get,
    Router,
};

mod home_page;
mod avaliable_loans;
mod all_borrowers;
mod all_lenders;
mod all_payments;
mod all_loan_requests;
mod all_loan_transactions;
use all_loan_transactions::all_loan_transactions;
use all_loan_requests::all_loan_requests;
use all_payments::all_payments;
use all_lenders::all_lenders;
use all_borrowers::all_borrowers;
use avaliable_loans::avaliable_loans;
use home_page::home_page;

use crate::State;
use std::sync::Arc;

pub async fn routes(Extension(state): Extension<Arc<State>>)->Router{
Router::new()

.route("/", get(home_page))
.route("/loans", get(avaliable_loans))
.route("/lenders", get(all_lenders))
.route("/borrowers", get(all_borrowers))
.route("/loan_requests", get(all_loan_requests))
.route("/loan_transactions", get(all_loan_transactions))
.route("/payments", get(all_payments))
.layer(Extension(state))
  
}