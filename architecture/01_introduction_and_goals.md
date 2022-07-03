# Introduction and Goals

This document describes BuenzliMarks, short BM, which is a bookmark
management application. It is used to store and organise browser bookmarks
and related things, such that they can be presented for fast and convenient
access later.

The following goals have been established for this system:

- The system shall have all features of current competing products
  which are _actually used_ by the target user base (family & friends of the developers).
- The system shall be _highly available_. Offline use is of equal priority
  to online use for UX considerations.
- Development of the system shall provide an opportunity to gain experience
  with cutting edge technologies to the developers.

## Requirements

Requirements are developed and documented as GitHub issues.
They can be found [here](https://github.com/remlse/buenzlimarks/issues?q=is%3Aissue+label%3AEpic%2C%22User+Story%22+).

## Quality Goals

The architecture of the system is designed to fulfill the following goals:

- Availability: The UX during offline use is of the highest possible quality.
- Efficiency: The self-hostable synchronisation service is resource efficient
  to enable deployment on cheap hardware like a raspberry pi.
- Extensiblilty: The system is easily extensible to store different
  information as well as provide more UI customization capabilities.

## Stakeholders

This is an overview of the stakeholders who

- should know the architecture
- have to be convinced of the architecture
- have to work with the architecture or with code
- need the documentation of the architecture for their work
- have to come up with decisions about the system or its development

| Role             | Contact | Expectation                                                                    |
| ---------------- | ------- | ------------------------------------------------------------------------------ |
| Product Owner    | Dad     | Architecture is flexible enough to enable iteratively evolving requirements    |
| Lead developer   | me      | Architecture is highly modular such as not to restrict technological choices   |
| Junior developer | Mom     | Architecture is clean and simple such as not to inhibit their learning process |
