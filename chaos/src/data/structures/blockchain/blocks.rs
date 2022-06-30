/*
    Appellation: block
    Context:
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
 */

pub enum BlockStates {
    Computing,
    Invalid,
    Valid,
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Block {
    pub id: crate::BlockId,
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}