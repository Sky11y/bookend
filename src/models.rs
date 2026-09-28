use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Book {
    id: i32,
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
