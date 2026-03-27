use std::{marker::PhantomData, ops::Deref};

// use sqlx::prelude::FromRow;
use uuid::Uuid;

pub mod models;
pub mod connect;
pub mod types;

// where T: ParialEq
/// the primary ID type for the entire library.
#[derive(PartialEq, Eq, Hash, Debug,Clone)]
pub struct Id {
    pub value: uuid::Uuid,
    // #[sqlx(skip)]
    // _marker: std::marker::PhantomData<T>,
}
impl Id {
    pub fn new_v7() -> Self {
        Self {
            value: uuid::Uuid::now_v7(),
            // _marker: PhantomData,
        }
    }
}

// enables us to use the id type in multiple databases



impl std::ops::Deref for Id {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl std::convert::From<Vec<u8>> for Id {
    fn from(value: Vec<u8>) -> Self {
        let value = value.as_array().unwrap();
        Self {
            value: uuid::Uuid::from_bytes(*value),
        }
    }
}

impl Default for Id {
    fn default() -> Self {
        Self {
            value: uuid::Uuid::now_v7(),
            
        }
    }
}



pub trait GetId {
    fn id(&self) -> &Id
    where
        Self: Sized;
}
pub trait Table {
    const NAME: &str;
    // type Columns: [impl Column<Table=Self>];
    // type PrimaryKey;
}

pub trait PrimaryKey {
    type Table: Table;
    type Type;
    const NAME: &str;
    const INDEX: usize = 0;
    fn primary_key(&self) -> Self::Type;
}


pub trait Column {
    const NAME: &str = "id";
    type Table: self::Table;
    type Type;
}

pub struct Passkeys;

/// unless you want a headache. NEVER change this value
impl Table for Passkeys {
    const NAME: &str = "Passkeys";
}

pub struct PasskeysPrimaryID(uuid::Uuid);
impl PrimaryKey for PasskeysPrimaryID {
    // const NAME: &str = ;
    type Type = uuid::Uuid;
    type Table = self::Passkeys;
    const NAME: &str = "id";
    fn primary_key(&self) -> Self::Type {
        self.0.clone()
    }   
}