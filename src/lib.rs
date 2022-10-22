//! A crate to ReDef Type for schema
mod account {
    // pub use super::*;
    mod auth {
        pub type AccountId = u64;
        pub type OpenId = u64;
        pub type UnionId = u64;
        pub type AppId = u64;
        pub type TenantFlowId = u64;
        pub type TenantKey = u64;
    }
    pub use auth::*;
    mod token {
        pub type UserAccessToken = String;
        pub type TenantAccessToken = String;
        pub type AppAccessToken = String;

        pub type TicketValue = u64;
    }
    pub use token::*;
    mod trading {
        use crate::*;

        pub type PaymentId = u64;
        pub type CustomerId = AccountId;
        pub type ProducerId = AccountId;
        pub type RetailerId = AccountId; // 零售商
        pub type StoreId = AccountId;
    }
    pub use trading::*;

    pub type Name = String;
    pub type AppPermissions = u64;
    // pub type
}
// pub use account::{auth::*, trading::*, AccountId, Name};
pub use account::*;
mod thing {
    use crate::*;

    pub type ObjectId = u64;
    pub type LanguageId = u16;
    // pub type ObjectKind = u32;
    pub type OwnerId = AccountId; // alias for user team
    pub type UserId = OwnerId;
    pub type TeamId = OwnerId;
    pub type Picture = Vec<u8>;
    mod chat {
        pub type ChatId = u64;
        pub type OpenMessageId = u64;
    }
    pub use chat::*;
    mod department {
        pub type OpenDepartmentId = u64;
        pub type DepartmentId = u64;
    }
    pub use department::*;
}
pub use thing::*;
mod place {
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
mod status {

    pub type TimeAlias = chrono::DateTime<chrono::Utc>; // use sea_orm::prelude::DateTimeUtc;
    pub type KeepDay = u16;
}
pub use status::*;
