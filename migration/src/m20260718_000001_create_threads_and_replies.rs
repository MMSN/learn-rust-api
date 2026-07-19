use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Threads::Table)
                    .if_not_exists()
                    .col(pk_auto(Threads::Id))
                    .col(integer(Threads::UserId).not_null())
                    .col(string(Threads::Title).string().not_null())
                    .col(string(Threads::Body).string().not_null())
                    .col(timestamp_with_time_zone(Threads::CreatedAt).not_null())
                    .col(timestamp_with_time_zone(Threads::UpdatedAt).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_threads_user")
                            .from(Threads::Table, Threads::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Replies::Table)
                    .if_not_exists()
                    .col(pk_auto(Replies::Id))
                    .col(integer(Replies::ThreadId).not_null())
                    .col(integer(Replies::UserId).not_null())
                    .col(integer(Replies::ParentReplyId).null())
                    .col(string(Replies::Body).string().not_null())
                    .col(timestamp_with_time_zone(Replies::CreatedAt).not_null())
                    .col(timestamp_with_time_zone(Replies::UpdatedAt).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_replies_thread")
                            .from(Replies::Table, Replies::ThreadId)
                            .to(Threads::Table, Threads::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_replies_user")
                            .from(Replies::Table, Replies::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_replies_parent_reply")
                            .from(Replies::Table, Replies::ParentReplyId)
                            .to(Replies::Table, Replies::Id)
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_threads_user_id")
                    .table(Threads::Table)
                    .col(Threads::UserId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_replies_thread_id")
                    .table(Replies::Table)
                    .col(Replies::ThreadId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_replies_user_id")
                    .table(Replies::Table)
                    .col(Replies::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Replies::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Threads::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Threads {
    Table,
    Id,
    UserId,
    Title,
    Body,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Replies {
    Table,
    Id,
    ThreadId,
    UserId,
    ParentReplyId,
    Body,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
}
