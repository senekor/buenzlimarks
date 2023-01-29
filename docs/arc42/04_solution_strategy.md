# Solution Strategy

## Quality Goals

<table>
  <tr>
    <th>Quality goal</th>
    <th>Solution approch</th>
  </tr>
  <tr>
    <td>The backend server is resource efficient.</td>
    <td>
      <i>written in Rust</i>
    </td>
  </tr>
  <tr>
    <td>The user interface is customizable.</td>
    <td>
      TailwindCSS / CSS-in-JS allow for dynamic, client-side styling.
    </td>
  </tr>
  <tr>
    <td>The backend server is easy to work on.</td>
    <td>
      The database implementation is filesystem based.
      It's written against an interface and may later be swapped out with relational database for increased performance.
    </td>
  </tr>
  <tr>
    <td>The UX must be of the highest quality.</td>
    <td>
      Use a service worker and indexedDB for offline availability.
      Use websockets for live updates when many devices are used simultaneously.
    </td>
  </tr>
</table>

## Technological Decisions

### Web app

- capable of fulfilling our key quality goal of first-class offline support via a service worker
- the only pragmatic option, as a bookmark manager that doesn't run in the browser can't deliver a great user experience

### Rust on the server

- zero-cost abstractions and performance on par with C and C++
- [loved by many programmers](https://survey.stackoverflow.co/2022/#section-most-loved-dreaded-and-wanted-programming-scripting-and-markup-languages)
- a modern type system with null safety, algebraic data types, powerful generics, exception-free error handling and no inheritance
- our developers are experienced with it

## Top-level / Architectural Design Patterns

- frontend: TODO
- backend: traditional layered architecture

## Organizational Decisions

### Implicit API specification

OpenAPI is a great tool to achieve strongly typed interactions with a well-defined API.
However, the code generators needed to make an openAPI based workflow productive are usually lagging behind for cutting edge languages and frameworks.
Because we want to use these, openAPI is much less appealing.
Moreover, the development team is small and informal communication about API changes is sufficient.

### Development process

The team organization is lean and informal, as there are only two developers and a product owner.
Scheduled meetings in the spirit of a sprint review, retrospective and planning combined are held every four weeks.
Requirements are developed in a GitHub project, where individual developers may organize their tasks as well.
Additional communication / meetings may always be initiated by any team member.
The junior backend developer is responsible themself to seek guidance by the lead developer when needed.

---

# TODO move to chapter 9

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

An ORM is used to avoid the tedium of manually writing SQL queries.
[Diesel](https://diesel.rs/) seems to be the "default" choice in the Rust
community and version 2.0 is about to be released.
[SeaORM](https://www.sea-ql.org/SeaORM/) is another, younger alternative worthy
of consideration. It hasn't yet reached 1.0, though. Both ORMs support
PostgreSQL, MySQL and SQLite. SQLite will probably suffice for our purposes and
a later migration to Postgres should be easy enough.

Diesel isn't well suited for async!

Diesel seems appealing, being the more "mature" option as well as having
stronger compile time checks. However, SeaORM has quite lovely documentation,
especially for newcomers to the concept of an ORM, while Diesel is lacking in
that area.

As our developer team has little to no experience with ORMs, SeaORM is chosen
for its superior documentation.

## Top-level / architectural design patterns

These will be documented as the development progresses. "Emergent architecture"
is the mantra for now.

## Quality goals

TODO

## Development process

The team organization is lean and informal, as there are only two developers and
a product owner. Scheduled meetings in the spirit of sprint review,
retrospective and planning are held every four weeks. Requirements are developed
in a GitHub project, where individual developers may organize their tasks as
well. Additional communication / meetings may always be initiated by any team
member. The junior backend developer is responsible themselves to seek guidance
by the lead developer when needed.
