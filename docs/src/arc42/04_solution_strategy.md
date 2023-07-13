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
