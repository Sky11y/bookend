use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, query_as};
// use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ModelController {
    db: PgPool,
}

impl ModelController {
    pub fn new(pool: PgPool) -> Self {
        Self { db: pool }
    }

    pub async fn list_books(&self) -> Result<Vec<Book>> {
        Ok(query_as!(Book, "SELECT * FROM books ORDER BY id")
            .fetch_all(&self.db)
            .await?)
    }

    pub async fn get_book(&self, id: i64) -> Result<Book> {
        query_as!(Book, "SELECT * FROM books WHERE id = $1", id)
            .fetch_optional(&self.db)
            .await?
            .ok_or(Error::NotFound)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    id: i64,
    isbn: String,
    title: String,
    description: String,
    published: i32,
}

#[derive(Debug, Serialize)]
pub struct Author {
    id: i64,
    first_name: String,
    last_name: String,
}

#[derive(Debug, Serialize)]
pub struct BookAuthors {
    book_id: i64,
    author_id: i64,
}

// #[derive(Debug, Serialize)]
// pub struct BookCopy {
//     id: Uuid,
//     book_id: Uuid,
//     condition: String,
// }
//
// #[derive(Debug, Serialize)]
// pub struct Customer {
//     id: Uuid,
//     first_name: String,
//     last_name: String,
//     email: String,
//     created_at: Instant,
// }
//
// #[derive(Debug, Serialize)]
// pub struct Borrow {
//     id: Uuid,
//     customer_id: Uuid,
//     book_id: Uuid,
//     borrowed_at: Instant,
//     due_at: Instant,
//     returned_at: Instant,
//     times_renewed: u8,
// }
//
// #[derive(Debug, Serialize)]
// pub struct Reservation {
//     id: Uuid,
//     customer_id: Uuid,
//     book_id: Uuid,
//     created_at: Instant,
//     status: String,
// }
