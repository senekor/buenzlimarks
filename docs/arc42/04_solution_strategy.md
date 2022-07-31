# Solution Strategy

## Technological decisions

- [web app](#web-app)
- [Solid](#solid)
- [TailwindCSS](#tailwindcss)
- [indexedDB](#indexeddb)
- [rust](#rust)
- [axum](#axum)
- [implicit api specification](#implicit-api-specification)
- [TODO persistence](#persistence)

#### web app

- capable of fulfilling our key quality goal of first-class offline support via
  a service worker
- the only pragmatic option, as a bookmark manager that doesn't run in the
  browser can't deliver a great user experience

#### Solid

- high performance [UI framework](https://www.solidjs.com/)
- similar API to React, with which lead developer has experience

#### TailwindCSS

- [CSS utility classes](https://tailwindcss.com/)
- enables beautiful and potentially user-customizable styling
- component libraries similar to Bootstrap, MaterialUI etc. aren't mature for
  Solid, whereas Tailwind is completely framework agnostic

#### indexedDB

- [MDN web docs](https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API)
- better suited for our relatively large amount of structured data than, for
  example, web storage or even cookies.

#### rust

- modern language
  [loved by many programmers](https://survey.stackoverflow.co/2022/#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages)
- zero-cost abstractions and performance rivalling C and C++
- language extensions via hygenic macros enable ergonomic, boilerplate-free
  library APIs
- a modern type system with optional types, algebraic data types, powerful
  generics, exception-free error handling and no inheritance
- a welcoming community and free
  [learning resources](https://doc.rust-lang.org/stable/book/)
- an ecosystem of [high-quality libraries](https://crates.io/)

#### axum

- [web framework for rust](https://docs.rs/axum/latest/axum/)
- integrated in the rust ecosystem - based on tokio, hyper & tower
- more developer friendly than actix
- more modular & extensible than rocket
- likely to be embraced by the community and well-maintained into the future

#### implicit API specification

OpenAPI is a great tool to achieve strongly typed interactions with a
well-defined API. However, the code generators needed to make an openAPI based
workflow productive are usually lagging behind for cutting edge languages and
frameworks. Because we want to use these, openAPI is much less appealing.
Moreover, the developer team is small and informal communication about API
changes are sufficient.

#### persistence

TODO

- ORM: [Diesel](https://diesel.rs/) or [SeaORM](https://www.sea-ql.org/SeaORM/)
  - Diesel: non-async, stronger compile-time checks
  - SeaORM: async, more dynamic - less compile-time checks
  - (SQLite can't use async anyway, diesel looks like better fit)
- DB: PostgreSQL or SQLite

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
