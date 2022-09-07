/*
    Appellation: primitives <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/

/// Type alias
pub type Ipfs<T = IpfsTestTypes> = ipfs::Ipfs<T>;

pub type IpfsTestTypes = ipfs::TestTypes;

/// Type alias
pub type StorjBucket = uplink::Bucket;
/// Type alias
pub type StorjGrant = uplink::access::Grant;
/// Type alias
pub type StorjProject = uplink::Project;
