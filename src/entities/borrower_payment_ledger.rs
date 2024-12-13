//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "BorrowerPaymentLedger")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub ledger_id: i32,
    pub borrower_id: i32,
    pub transaction_id: i32,
    pub product_id: i32,
    pub payment_date: DateTime,
    #[sea_orm(column_type = "Float")]
    pub payment_amount: f32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::borrowers::Entity",
        from = "Column::BorrowerId",
        to = "super::borrowers::Column::BorrowerId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Borrowers,
    #[sea_orm(
        belongs_to = "super::loan_products::Entity",
        from = "Column::ProductId",
        to = "super::loan_products::Column::ProductId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    LoanProducts,
    #[sea_orm(
        belongs_to = "super::loan_transactions::Entity",
        from = "Column::TransactionId",
        to = "super::loan_transactions::Column::TransactionId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    LoanTransactions,
}

impl Related<super::borrowers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Borrowers.def()
    }
}

impl Related<super::loan_products::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LoanProducts.def()
    }
}

impl Related<super::loan_transactions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::LoanTransactions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
