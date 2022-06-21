pub mod flood;
pub mod ipfs;
pub mod storage;


pub trait Contract {
    type Actor;
}