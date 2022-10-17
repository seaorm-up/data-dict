//! A crate to ReDef Type for schema
pub(crate) mod account {
    // pub use super::*;
    pub(crate) mod auth {
        pub type AccountId = u64;
        pub type UserId = AccountId;
        pub type TeamId = AccountId;
    }
    pub use auth::*;
    pub(crate) mod trading {
        use crate::AccountId;

        pub type PaymentId = u64;
        pub type CustomerId = AccountId;
        pub type ProducerId = AccountId;
        pub type RetailerId = AccountId; // 零售商
        pub type StoreId = AccountId;
    }
    pub use trading::*;

    pub type Name = String;
    // pub type
}
// pub use account::{auth::*, trading::*, AccountId, Name};
pub use account::*;
pub(crate) mod thing {
    use crate::*;

    pub type ObjectId = u64;
    pub type LanguageId = u16;
    // pub type ObjectKind = u32;
    pub type OwnerId = AccountId; // alias for user team
    pub type Picture = Vec<u8>;
}
pub use thing::*;
pub(crate) mod place {
    pub type Address = String;
    pub type AddressId = u32;
    pub type District = String;
    pub type DistrictId = u32;
    pub type City = String;
    pub type CityId = u32;
    pub type State = String;
    pub type StateId = u32;
    pub type PostalCode = String;
}
pub use place::*;
pub(crate) mod status {

    pub type TimeAlias = chrono::DateTime<chrono::Utc>; // use sea_orm::prelude::DateTimeUtc;
    pub type KeepDay = u16;
}
pub use status::*;