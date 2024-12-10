
mod routes_controller;
use routes_controller::routes;
use dotenv::dotenv;
use dotenvy_macro::dotenv;
mod contract;
mod setup_db;
use setup_db::set_up_db;
use sea_orm::DatabaseConnection;
mod entities;
use entities::{prelude::*, *};
use axum::{
 
    extract:: Extension,
};


#[derive(Debug,Clone)]
pub struct State{
   pub  db :DatabaseConnection
}
pub async fn run(){

    dotenv().ok();
    const PORT:&str = dotenv!("PORT");

    let db: DatabaseConnection = match set_up_db().await {
        Ok(db) => db,
          Err(err) => panic!("{}", err),
      };
 let state  =State{db:db};
 println!("{:?}",state.db);
        // build our application with a single route
        let app = routes(Extension(state)).await;    
        // run our app with hyper, listening globally on port 3000
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
        axum::serve(listener, app).await.unwrap();
}