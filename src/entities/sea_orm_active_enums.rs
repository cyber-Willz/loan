//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum,Deserialize,Serialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "currency")]
pub enum Currency {
    #[sea_orm(string_value = "BZ")]
    Bz,
    #[sea_orm(string_value = "USD")]
    Usd,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum,Deserialize,Serialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "source_type")]
pub enum SourceType {
    #[sea_orm(string_value = "Cheque")]
    Cheque,
    #[sea_orm(string_value = "Cash")]
    Cash,
    #[sea_orm(string_value = "Direct Transfer")]
    DirectTransfer,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "status")]
pub enum Status {
    #[sea_orm(string_value = "Active")]
    Active,
    #[sea_orm(string_value = "Closed")]
    Closed,
}
