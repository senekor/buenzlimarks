# Persistence with SeaORM - SUPERSEDED

## Context

An ORM is used to avoid the tedium of manually writing SQL queries.
[Diesel](https://diesel.rs/) seems to be the "default" choice in the Rust community and version 2.0 is about to be released.
[SeaORM](https://www.sea-ql.org/SeaORM/) is another, younger alternative worthy of consideration.
It hasn't yet reached 1.0, though.
Both ORMs support PostgreSQL, MySQL and SQLite.
SQLite will probably suffice for our purposes and a later migration to Postgres should be easy enough.

Diesel isn't well suited for async!

Diesel seems appealing, being the more "mature" option as well as having stronger compile time checks.
However, SeaORM has quite lovely documentation, especially for newcomers to the concept of an ORM, while Diesel is lacking in that area.

Our developer team has little to no experience with ORMs.

## Decision

We will write our persistence layer with SeaORM.

## Status

Superseded by [Simple Filesystem Database](./004_filesystem_database.md).

## Consequences

Database access is abstracted away, saving our developers from manual labor.
Future migrations from and to database implementations are easy.

## Experience report

An ORM feels quite heavy-weight, even overkill, for our simple web app.
Although the documentation is great, the concept of an ORM, combined with relational databases, increase the learning curve for our junior developer significantly.
