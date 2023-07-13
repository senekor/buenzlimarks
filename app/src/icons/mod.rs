//! A module containing leptos component-wrappers around heroicons.
//! You can search the full catalogue of icons here:
//! https://heroicons.com/
//!
//! There is also [leptos-icons](https://github.com/Carlosted/leptos-icons)
//! but it seems a little weird. Every icon has its own feature flag.

mod arrow_right_rectangle;
mod pencil_square;
mod plus;
mod question_mark_circle;
mod x_mark;

pub use arrow_right_rectangle::ArrowRightOnRectangleIcon;
pub use pencil_square::PencilSquareIcon;
pub use plus::PlusIcon;
pub use question_mark_circle::QuestionMarkCircleIcon;
pub use x_mark::XMarkIcon;
