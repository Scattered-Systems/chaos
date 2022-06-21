/*
    Appellation: controllers
    Context: module
    Creator: FL03 <jo3mccain@icloud.com> (https://pzzld.eth.link/)
    Description:
        ... Summary ...
 */

pub trait ControllerSpec<T> {
    type Actor;
    type Client;
    type Context;
    type Data;

    fn constructor(configuration: T) -> Self::Context;
}