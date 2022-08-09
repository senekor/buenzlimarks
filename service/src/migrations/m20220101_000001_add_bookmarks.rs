use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Bookmarks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Bookmarks::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Bookmarks::Name).string().not_null())
                    .col(ColumnDef::new(Bookmarks::Url).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Bookmarks::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Bookmarks {
    Table,
    Id,
    Name,
    Url,
}
