use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(string(Users::Id).not_null().primary_key())
                    .col(string(Users::Email).not_null())
                    .col(string(Users::UserKey).not_null())
                    .col(string(Users::Salt).not_null())
                    .col(string(Users::Vault).not_null())
                    .col(integer(Users::Iterations).not_null())
                    .col(string(Users::Vaultiv).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Email,
    UserKey,
    Salt,
    Vault,
    Iterations,
    Vaultiv,
}
