use diesel::{
    backend::Backend,
    deserialize::{self, FromSql, FromSqlRow},
    expression::AsExpression,
    mysql::Mysql,
    serialize::{self, IsNull, Output, ToSql},
    sql_types::{self, Binary},
};

// Generic uuid conversion
#[derive(Debug, Clone, AsExpression, FromSqlRow)]
#[diesel(sql_type = sql_types::Binary)]
pub struct UUID(pub uuid::Uuid);

impl From<UUID> for uuid::Uuid {
    fn from(s: UUID) -> Self {
        s.0
    }
}

impl Into<UUID> for uuid::Uuid {
    fn into(self) -> UUID {
        UUID(self)
    }
}

impl ToSql<Binary, Mysql> for UUID {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> serialize::Result {
        let by = *self.0.as_bytes();
        std::io::Write::write_all(out, by.as_slice())?;

        Ok(IsNull::No)
    }
}

impl FromSql<Binary, Mysql> for UUID {
    fn from_sql(bytes: <Mysql as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        Ok(UUID(uuid::Uuid::from_slice(bytes.as_bytes())?))
    }
}
