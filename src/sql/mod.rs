#![feature(type_alias_impl_trait)]
use std::{marker::PhantomData, ops::Deref};


pub mod connect;
pub mod migrator;
pub mod generated;
// macro_rules! entity {
//     ($Ty:T) => {
//         impl $crate::Entity for $Ty {

//         }
//     };
// }

macro_rules! sqlite_impl {
    ($T:ty,$Table_literal:literal) => {
        impl $T
        where
            $T: $crate::Table + $crate::GetId + $crate::PrimaryKey + Self::Sized,
        {
            async fn async_get_one<'a, E>(executor: E, id: &Id<$T>) -> $T {
                sqlx::query_as!($T, concat!("src/scripts/sqlite/", $table_literal,))
                    .fetch_one(executor)
                    .await
            }
        }
    };
}

