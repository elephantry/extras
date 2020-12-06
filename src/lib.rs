pub mod html;
#[cfg(feature = "tera")]
pub mod tera;

mod pager;
mod pagination;

pub use pagination::Pagination;

pub use pager::Pager;
