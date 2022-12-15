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
                    .col(ColumnDef::new(Download::FileName).text().not_null())
                    .col(ColumnDef::new(Download::Host).text().not_null())
                    .col(ColumnDef::new(Download::Url).text().not_null())
                    .col(
                        ColumnDef::new(Download::Status)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Download::RawSize).float().not_null())
                    .col(ColumnDef::new(Download::AdjustedSize).float().not_null())
                    .col(ColumnDef::new(Download::ByteUnit).text().not_null())
                    .col(ColumnDef::new(Download::Progress).text().not_null())
                    .col(ColumnDef::new(Download::TempFile).text().not_null())
                    .to_owned(),
            )
            .await

        // manager
        //     .create_table(
        //         Table::create()
        //             .table(Post::Table)
        //             .if_not_exists()
        //             .col(
        //                 ColumnDef::new(Post::Id)
        //                     .integer()
        //                     .not_null()
        //                     .auto_increment()
        //                     .primary_key(),
        //             )
        //             .col(ColumnDef::new(Post::Title).string().not_null())
        //             .col(ColumnDef::new(Post::Text).string().not_null())
        //             .to_owned(),
        //     )
        //     .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
    FileName,
    Host,
    Status,
    Url,
    RawSize,
    AdjustedSize,
    ByteUnit,
    Progress,
    TempFile,
}
