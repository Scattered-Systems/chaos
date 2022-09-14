/*
    Appellation: endpoints <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::{crud::CrudRouter, index::Homepage, storj::StorjRouter};

mod crud;
mod index;
mod storj;

pub trait PageRouter {
    fn path(&self) -> String;
    fn router(&self) -> axum::Router;
}
