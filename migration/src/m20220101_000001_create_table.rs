/// Migration: Initialize the database.

use sea_schema::migration::{
    sea_query::{self, *},
    *,
};

use entities::{
    images,
    users,
    match_responses_2022,
    pit_responses_2022
};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            // Create Images table
            Table::create()
                .table(images::Entity)
                .if_not_exists()
                .col(
                    ColumnDef::new(images::Column::Id)
                        .auto_increment()
                        .primary_key()
                        .integer()
                        .unique_key()
                        .not_null()
                )
                .col(
                    ColumnDef::new(images::Column::B64)
                        .text()
                        .not_null()
                )
                .to_owned()
        ).await?;
        manager.create_table(
            // Create Users table
            Table::create()
                .table(users::Entity)
                .if_not_exists()
                .col(
                    ColumnDef::new(users::Column::Uuid)
                        .not_null()
                        .auto_increment()
                        .primary_key()
                        .unique_key()
                        .integer()
                )
                .col(
                    ColumnDef::new(users::Column::Username)
                        .not_null()
                        .string()
                )
                .col(
                    ColumnDef::new(users::Column::Created)
                        .not_null()
                        .integer()
                )
                .col(
                    ColumnDef::new(users::Column::Email)
                        .not_null()
                        .text()
                )
                .col(
                    ColumnDef::new(users::Column::PassHash)
                        .not_null()
                        .text()
                )
                .col(
                    ColumnDef::new(users::Column::Team)
                        .not_null()
                        .integer()
                ).to_owned()
        ).await?;
        // TODO: Add migrations for the other tables (match resp, pit resp, teams)
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        todo!()
    }
}
