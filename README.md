# Bookend

A book library backend. Written in Rust.

## About

This project...

### Features

...

### Restrictions

...

### Tools

- Axum -
- Tokio -
- Serde -
- sqlx -
- Tower-http -

## Getting started

### Prerequisites and Dependencies

...

### Installation

... (docker)

### Configuration

... (env)

### Running

...

### Running tests

...

## Documentation

### Database schema

```mermaid
---
title: .draft
---
erDiagram
    LIBRARY ||--o{ BOOK : has
    LIBRARY ||--o{ CUSTOMER : has
    BOOK ||--|{ BOOK_AUTHORS: has
    AUTHOR ||--o{ BOOK_AUTHORS : writes
    BOOK ||--o{ BOOK_COPY : "exists as"
    BOOK_COPY ||--o{ BORROW : "is borrowed in"
    CUSTOMER ||--o{ RESERVATION : places
    CUSTOMER ||--o{ BORROW: makes
    BOOK ||--o{ RESERVATION : "is requested in"

    BOOK {
        uuid id PK
        string isbn
        string title
        string description
        int year
    }
    AUTHOR {
        uuid id PK
        string first_name
        string last_name
    }
    BOOK_AUTHORS {
        uuid book_id PK, FK
        uuid author_id PK, FK
    }
    BOOK_COPY {
        uuid id PK
        uuid book_id FK
        string condition
    }
    CUSTOMER {
        uuid id PK
        string first_name
        string last_name
        string email
        timestamp registered_at
    }
    BORROW {
        uuid id PK
        uuid customer_id FK
        uuid book_id FK
        timestamp borrowed_at
        timestamp due_at
        timestamp returned_at
        int times_renewed
    }
    RESERVATION {
        uuid id PK
        uuid customer_id FK
        uuid book_id FK
        timestamp reserved_at
        string status "active | fulfilled | cancelled"
    }
```


### API

...

### Design decisions

**Database**
- A separate BOOK_COPY entry instead of a copy count on BOOK
    - allows per-physical-copy properties like *condition* (or shelf location, etc.)
    - each borrow references exactly one copy
- Reservations keep status instead of deletion on fulfill/cancel
    - allows misuse prevention (think of client reserving-cancelling multiple times)
    - allows analytics, if that is preferred.
- Junction table for AUTHOR and BOOK instead of author list on book
    - forced decision by many-to-many relationship


## Roadmap

...

## Authors

[Sky11y](https://github.com/Sky11y/)

## License

[MIT]


