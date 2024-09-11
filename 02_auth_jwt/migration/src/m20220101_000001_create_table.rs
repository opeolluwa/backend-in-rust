use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserInformation::Table)
                    .if_not_exists()
                    .col(uuid(UserInformation::Id).unique_key().primary_key())
                    .col(string(UserInformation::Password))
                    .col(string(UserInformation::FirstName))
                    .col(string(UserInformation::LastName))
                    .col(string(UserInformation::Email).unique_key())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserInformation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserInformation {
    Table,
    Id,
    FirstName,
    LastName,
    Password,
    Email,
}
