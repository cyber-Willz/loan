use axum::{
    extract:: Extension,
    routing::{get,post,put,delete},
    Router,
};

mod home_page;
mod avaliable_loans;
mod all_borrowers;
mod all_lenders;
mod all_payments;
mod all_loan_requests;
mod all_loan_transactions;
mod lender_loan_schedules;
mod submit_loan;
mod update_loan;
mod find_loan_by_id;
mod delete_loan;
mod borrower_loan_schedule;

use borrower_loan_schedule::borrower_loan_schedule;
use delete_loan::delete_loan;
use find_loan_by_id::find_loan_by_id;
use update_loan::update_loan;
use submit_loan::submit_loan;
use lender_loan_schedules::lender_loan_schedules;
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
.route("/find_loan_by_id/:product_id", get(find_loan_by_id))
.route("/lenders", get(all_lenders))
.route("/borrowers", get(all_borrowers))
.route("/loan_requests", get(all_loan_requests))
.route("/loan_transactions", get(all_loan_transactions))
.route("/payments", get(all_payments))
.route("/lender_loan_amortization/:product_id", get(lender_loan_schedules))
.route("/borrower_loan_amortization", post(borrower_loan_schedule))
.route("/submit_loan", post(submit_loan))
.route("/update_loan", put(update_loan))
.route("/delete_loan/:product_id", delete(delete_loan))
.layer(Extension(state))
  
}