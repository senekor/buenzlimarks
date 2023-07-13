# Rust & axum backend

## Context

The backend of a web app can be written with virtually any language, contrary to the frontend.
buenzlimarks' backend must be resource efficient for deployment on cheap hardware.

This basically eliminates Python and strongly disfavors languages whose ecosystems are characterized by "enterprisey", resource-hungry frameworks like Java and C#.
JavaScript is quite performant with V8 and TypeScript allows writing quite robust and maintainable software.
C and C++ are performance beasts, but suffer from decades old lanaguage foot guns.

Go is highly performant, but its language design is outdated.
It has no algebraic data types or null safety, weak generics and modularity.
It even introduces some entirely new foot guns, like tedious to circumvent and error-prone zero-initialization of structs.

Rust has no garbage collection like C/C++ and matches them in performance.
It avoids all their foot guns and provides important modern language features like algebraic data types, null safety and strong generics.
It goes very far in preventing common developer mistakes at compile time.
For example, it guarantees thread safety.

When writing a web server, it is common to use a library / framework to provide abstractions over the HTTP protocol. A language with a strong ecosystem is necessary to reduce the amount of manual work required.

JavaScript has many such libraries, including ones specifically designed for the most popular UI frameworks.
The same language on the server and the client enables a very nice developer experience, [trpc](https://trpc.io/) being a great example of that.

Rust has a lively, almost volatile webserver ecosystem.
Actix Web and [axum](https://docs.rs/axum/latest/axum/) seem to be the current community-favorites.
Axum is quite new, made by the developers of tokio (the most popular async runtime) and is designed for simplicity and modularity.

All current developers are experienced with Rust, with is not the case for JavaScript.

## Decision

We will write the web app with Rust and axum.

## Status

Accepted.

## Consequences

The performance of the backend server will be perfect for running on constrained hardware.
Gluing together the modular pieces will require more work and architecural considerations.
The resulting code is expected to be highly maintainable and robust.

## Experience report
