use axum::{
     
    extract:: Extension,
    routing::get,
    Router,
};

mod home_page;
mod avaliable_loans;
use avaliable_loans::avaliable_loans;
use home_page::home_page;
use crate::State;


pub async fn routes(Extension(state): Extension<State>)->Router{
    Router::new()
    .route("/", get(home_page))
// .route("/loans", get(avaliable_loans))
.layer(Extension(state))
  
}