use postgres_types::{FromSql, ToSql};



// use tokio_postgres::types::ToSql;
#[derive(Debug, ToSql, FromSql,Clone)]
#[postgres(transparent)]
pub struct UniqueId(uuid::Uuid);


#[derive(ToSql,FromSql,Debug)]
pub struct Passkeys {
    id: UniqueId,
    value: String,
}
impl super::Table for Passkeys {
    const NAME: &str = "Passkeys";
}
impl super::PrimaryKey for Passkeys {
    const INDEX: usize = 0;
    const NAME: &str = "id";
    type Table = Self;
    type Type = UniqueId;
    fn primary_key(&self) -> Self::Type {
        self.id.clone()
    }
}