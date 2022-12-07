use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Download::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Download::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Download::Name).string().not_null())
                    .col(ColumnDef::new(Download::RawSize).string().not_null())
                    .col(ColumnDef::new(Download::AdjustedSize).string().not_null())
                    .col(ColumnDef::new(Download::Unit).string().not_null())
                    .col(ColumnDef::new(Download::URL).string().not_null())
                    .col(ColumnDef::new(Download::Host).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Download::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Download {
    Table,
    Id,
    Name,
    RawSize,
    AdjustedSize,
    Unit,
    URL,
    Host,
}
