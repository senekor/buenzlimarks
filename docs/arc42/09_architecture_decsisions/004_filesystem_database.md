# Simple Filesystem Database

## Context

Relational databases and ORMs are difficult concepts for absolute newcomers, whereas the filesystem is familiar to any developer.

Reading and writing plain files is much less efficient than a relational database.
This performance drop likely won't be noticeable for users, as our traffic will be very low.

To make a future transistion easier, a filesystem database can be written against an opaque interface.
This would enable an implementation of the persistance layer based on a relational database to be swapped in, while barely affecting the code using the database.

## Decision

We will switch our persistence layer from SeaORM to the filesystem.

## Status

Accepted.

## Consequences

Onboarding of inexperienced junior developers will be much easier.
Performance of the backend will be much lower, though still not noticeably bad.

## Experience report
