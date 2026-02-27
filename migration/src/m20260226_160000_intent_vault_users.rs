use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Vault::Table)
                    .if_not_exists()
                    .col(string(Vault::Id).not_null().primary_key())
                    .col(string(Vault::Vault).not_null())
                    .col(string(Vault::Salt).not_null())
                    .col(integer(Vault::Iterations).not_null())
                    .col(string(Vault::Vaultiv).not_null())
                    .col(timestamp(Vault::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(integer(Vault::Version).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(string(User::Id).not_null().primary_key())
                    .col(string(User::Email).not_null())
                    .col(string(User::UserKey).not_null())
                    .col(string(User::VaultId).not_null().unique_key())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_vault_id")
                            .from(User::Table, User::VaultId)
                            .to(Vault::Table, Vault::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Intent::Table)
                    .if_not_exists()
                    .col(string(Intent::Id).not_null().primary_key())
                    .col(string(Intent::VaultId).not_null())
                    .col(string(Intent::IntentType).not_null())
                    .col(string(Intent::Payload).not_null())
                    .col(string(Intent::Status).not_null())
                    .col(integer(Intent::VaultVersion).not_null())
                    .col(timestamp(Intent::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(timestamp(Intent::AppliedAt).not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_intent_vault_id")
                            .from(Intent::Table, Intent::VaultId)
                            .to(Vault::Table, Vault::Id)
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Intent::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Vault::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Vault {
    Table,
    Id,
    Vault,
    Salt,
    Iterations,
    Vaultiv,
    CreatedAt,
    Version,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Email,
    UserKey,
    VaultId,
}

#[derive(DeriveIden)]
enum Intent {
    Table,
    Id,
    VaultId,
    IntentType,
    Payload,
    Status,
    VaultVersion,
    CreatedAt,
    AppliedAt,
}
