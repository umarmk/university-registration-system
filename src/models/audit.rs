use crate::schema::audit_logs;
use chrono::{DateTime, Utc};
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Jsonb;
use diesel::{AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::io::Write;

// Create a wrapper around JsonValue to implement diesel SQL traits
#[derive(Debug, Clone, Serialize, Deserialize, FromSqlRow, AsExpression)]
#[diesel(sql_type = Jsonb)]
pub struct Json(pub JsonValue);

impl From<JsonValue> for Json {
    fn from(value: JsonValue) -> Self {
        Json(value)
    }
}

impl From<Json> for JsonValue {
    fn from(json: Json) -> Self {
        json.0
    }
}

impl ToSql<Jsonb, Pg> for Json {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        out.write_all(&[1])?; // JSONB version
        serde_json::to_writer(out, &self.0)?;
        Ok(IsNull::No)
    }
}

impl FromSql<Jsonb, Pg> for Json {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> deserialize::Result<Self> {
        let value = serde_json::from_slice(bytes.as_bytes())?;
        Ok(Json(value))
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = audit_logs)]
pub struct AuditLog {
    pub id: i32,
    pub user_id: Option<i32>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<i32>,
    pub details: Option<Json>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = audit_logs)]
pub struct NewAuditLog {
    pub user_id: Option<i32>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<i32>,
    pub details: Option<Json>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

impl AuditLog {
    pub fn new_activity(
        user_id: Option<i32>,
        action: &str,
        entity_type: &str,
        entity_id: Option<i32>,
        details: Option<JsonValue>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> NewAuditLog {
        NewAuditLog {
            user_id,
            action: action.to_string(),
            entity_type: entity_type.to_string(),
            entity_id,
            details: details.map(Json),
            ip_address,
            user_agent,
        }
    }
}
