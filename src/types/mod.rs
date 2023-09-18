use std::io::Write;

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
    fn from(value: UUID) -> Self {
        value.0
    }
}

impl Into<UUID> for uuid::Uuid {
    fn into(self) -> UUID {
        UUID(self)
    }
}

impl ToSql<Binary, Mysql> for UUID {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> serialize::Result {
        Write::write_all(out, self.0.as_bytes().as_slice())?;

        Ok(IsNull::No)
    }
}

impl FromSql<Binary, Mysql> for UUID {
    fn from_sql(bytes: <Mysql as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        Ok(UUID(uuid::Uuid::from_slice(bytes.as_bytes())?))
    }
}
