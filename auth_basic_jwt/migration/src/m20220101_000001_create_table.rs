use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(UserInformation::Table)
                    .if_not_exists()
                    .col(
                        string(UserInformation::Id)
                            .unique_key()
                            .uuid()
                            .primary_key(),
                    )
                    .col(string(UserInformation::Username).not_null().unique_key())
                    .col(string(UserInformation::Password).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(UserInformation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserInformation {
    Table,
    Id,
    Username,
    Password,
}
