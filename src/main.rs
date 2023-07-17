use sea_orm_migration::prelude::*;

use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Iden};
use sea_orm_migration::SchemaManager;
use time::OffsetDateTime;

pub mod entities;

#[derive(Iden)]
pub enum UnitState {
    Table,
    CreatedDate,
}

async fn create_tables(db: &DatabaseConnection) -> Result<(), DbErr> {
    let manager = SchemaManager::new(db);

    manager
        .create_table(
            Table::create()
                .table(UnitState::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(UnitState::CreatedDate)
                        .date_time()
                        .primary_key()
                        .not_null(),
                )
                .to_owned(),
        )
        .await?;

    Ok(())
}

async fn drop_tables(db: &DatabaseConnection) -> Result<(), DbErr> {
    let manager = SchemaManager::new(db);

    manager
        .drop_table(Table::drop().table(UnitState::Table).to_owned())
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let database_url = "mysql://test:test@localhost:23456/test";
    let db = sea_orm::Database::connect(database_url)
        .await
        .expect("Database Connected");

    create_tables(&db).await.unwrap();

    entities::unit_state::ActiveModel {
        created_date: Set(OffsetDateTime::now_utc()),
    }
    .insert(&db)
    .await
    .unwrap();

    drop_tables(&db).await.unwrap();
}
