mod entities;

use std::env;
use std::time::Duration;
use dotenv::dotenv;
use sea_orm::{ActiveModelTrait, ActiveValue, ConnectOptions, Database, DbConn, DbErr, DeleteResult, EntityTrait};
use sea_orm::ActiveValue::Set;
use crate::entities::{todos, users};
use crate::entities::prelude::Todos;

pub async  fn check_connection() -> Result<(), DbErr> {
    let db = establish_connection().await?;

    assert!(db.ping().await.is_ok());
    db.clone().close().await.expect("panic!");

    Ok(())
}

pub async fn insert_user(db: &DbConn) -> Result<users::Model, DbErr> {
    let user = users::ActiveModel {
        id: ActiveValue::NotSet,
        name: Set("John Smith".to_string())
    };

    let user: users::Model = user.insert(db).await?;

    Ok(user)
}

pub async fn insert_todo(db: &DbConn, user: &users::Model) -> Result<todos::Model, DbErr> {
    let todo = todos::ActiveModel {
        id: ActiveValue::NotSet,
        title: Set("Test".to_string()),
        description: Set("".to_string()),
        done: Default::default(),
        created_by: Set(user.id),
        updated_by:  Set(user.id),
    };

    let todo: todos::Model = todo.insert(db).await?;

    Ok(todo)
}

pub async fn select_todo(db: &DbConn, todo: todos::Model) -> Result<Option<todos::Model>, DbErr> {
    let selected: Option<todos::Model> = Todos::find_by_id(todo.id).one(db).await?;
    Ok(selected)
}

// pub async fn select_todos_by_user(db: &DbConn, user: &users::Model) -> Result<Vec<todos::Model>, DbErr> {
//     let selected: Vec<todos::Model> = user.find_related(Todos).all(db).await?;
//     Ok(selected)
// }

pub async fn update_todo(db: &DbConn, todo: todos::Model) -> Result<todos::Model, DbErr> {
    let mut target: todos::ActiveModel = todo.into();
    target.done = Set(true);

    let todo: todos::Model = target.update(db).await?;
    Ok(todo)
}

pub async fn delete_todo(db: &DbConn, todo: todos::Model) -> Result<(), DbErr> {
    let target: todos::ActiveModel = todo.into();
    let _: DeleteResult = target.delete(db).await?;
    Ok(())
}

pub async fn establish_connection() -> Result<DbConn, DbErr> {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("DATABASE_URL is not found.");

    let mut opt = ConnectOptions::new(url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    Database::connect(opt).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn connection_works() {
        let result = check_connection().await;
        assert_eq!(result, Ok(()));
    }

    #[tokio::test]
    async fn it_works() {
        let db = establish_connection().await.expect("connection error!");

        match insert_user(&db).await {
            Ok(user) => {
                println!("{:?}", user);

                match insert_todo(&db, &user).await {
                    Ok(todo) => {
                        println!("{:?}", todo);

                        // not working
                        // let result = select_todos_by_user(&db, &user).await;

                        match select_todo(&db, todo).await {
                            Ok(result) => {
                                match result {
                                    None => {}
                                    Some(todo) => {
                                        println!("{:?}", todo);

                                        match update_todo(&db, todo).await {
                                            Ok(todo) => {
                                                println!("{:?}", todo);

                                                let result = delete_todo(&db, todo).await;
                                                assert!(result.is_ok());
                                            }
                                            Err(_) => {}
                                        }
                                    }
                                }
                            }
                            Err(_) => {}
                        }
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
    }
}
