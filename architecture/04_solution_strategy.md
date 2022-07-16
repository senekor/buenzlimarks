# Solution Strategy

## Technological decisions

A web app is capable of fulfilling our key quality goal of first-class offline
support via a service worker. A web app is also the only pragmatic option as a
bookmark manager that doesn't run in the browser can't deliver a great user
experience. High performance is achieved with the UI framework
[Solid](https://www.solidjs.com/). The main developer of the frontend (me)
already has a lot of experience with React and Solid has a similar API which
allows a familiar developer experience.
[TypeScript](https://www.typescriptlang.org/) is chosen for its static type
analysis, which enables a better developer experience and more robust software
than JavaScript, for all but the smallest projects.
[TailwindCSS](https://tailwindcss.com/) enables beautiful and potentially
user-customizable styling. Also, Component libraries similar to Bootstrap,
MaterialUI etc. do have ports for Solid, but these may not be as polished, as
Solid is still a young framework. Tailwind is completely framework agnostic and
therefore delivers a mature developer experience. A browser extension is really
the only option to achieve a good user experience anywhere in the browser. Both
web app and browser extension will access the browser's
[indexedDB](https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API). It
is better suited for our relatively large amount of structured data than, for
example, web storage or even cookies.

On the backend, technological choices are less restricted by the browser
environment. [Rust](https://www.rust-lang.org/) is a modern language loved by
many developers, including the lead developer of BuenzliMarks. It was voted by
far the most loved language in the
[stack overflow developer survey](https://survey.stackoverflow.co/2022/#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages)
for 7 years in a row. Its highlights include:

- top tier performance equal to languages with manual memory management like C
  and C++
- a modern type system with optional types, algebraic data types and generics
- language extensibility via hygenic macros
- a welcoming community and free
  [learning resources](https://doc.rust-lang.org/stable/book/)
- a fantastic ecosystem of [high-quality libraries](https://crates.io/)
- a friendly compiler with helpful error messages and an equally delightful
  linter called clippy

The steep learning curve is considered the most significant downside of using
Rust. However, as providing learning opportunities for the developers is amoung
the project goals of BuenzliMarks, this is not a downside at all for our
purposes, on the contrary.

The lead developer of BuenzliMarks is of the personal opinion that Rust allows
developers to create software at a level of quality that makes them proud and
confident in their work. This may be a less tangible benefit than the ones
mentioned above, but no less important to foster a happy and motivated work
culture amoung the developers.

[Axum](https://docs.rs/axum/latest/axum/) was chosen as the web framework. There
were several other choices available:

- Actix: Known for its high performance but not necessarily for its developer
  friendliness.
- Rocket: Very developer friendly, but its stable version at the time of writing
  (4.x) relies on the nightly version of rust. Also, the community of
  maintainers seems to be somewhat small.

Axum is firmly rooted in the [tokio ecosystem](https://tokio.rs/), relying on
the tokio runtime, hyper and tower. Its API is simple, explicit and extensible.
For these reasons, axum has garnered much enthusiasm from the Rust community and
is likely to be well-supported in the future, making it a robust choice for this
project.

The decisions concerning the persistence layer are not yet set in stone. An ORM
will likely be used to avoid the tedium of manually writing SQL queries.
[Diesel](https://diesel.rs/) seems to be the "default" choice in the Rust
community and version 2.0 is about to be released.
[SeaORM](https://www.sea-ql.org/SeaORM/) is another, younger alternative worthy
of consideration. It hasn't yet reached 1.0, though. Both ORMs support
PostgreSQL, MySQL and SQLite. SQLite will probably suffice for our purposes and
a later migration to Postgres should be easy enough.

## Top-level / architectural design patterns

These will be documented as the development progresses. "Emergent architecture"
is the mantra for now.

## Quality goals

TODO

## Development process

The team organization is lean and informal, as there are only two developers and
a product owner. Scheduled meetings in the spirit of sprint review,
retrospective and planning are held every four weeks. Additional communication /
meetings may always be initiated by any team member. The junior backend
developer is responsible themselves to seek guidance by the lead developer when
needed.
