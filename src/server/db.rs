use sqlx::{PgPool, Row};

use crate::models::{Attendee, RegistrationUpdate};

pub async fn list_attendees(db: &PgPool) -> Result<Vec<Attendee>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
            SELECT name, ticket_id, ticket_type, registered
            FROM attendees
            ORDER BY name
        "#,
    )
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| Attendee {
            name: row.get("name"),
            ticket_id: row.get("ticket_id"),
            ticket_type: row.get("ticket_type"),
            registered: row.get("registered"),
        })
        .collect())
}

pub async fn update_registration(
    db: &PgPool,
    update: &RegistrationUpdate,
) -> Result<Option<RegistrationUpdate>, sqlx::Error> {
    sqlx::query(
        r#"
            UPDATE attendees
            SET registered = $1, updated_at = NOW()
            WHERE ticket_id = $2
            RETURNING ticket_id, registered
        "#,
    )
    .bind(update.registered)
    .bind(&update.ticket_id)
    .fetch_optional(db)
    .await
    .map(|row| {
        row.map(|row| RegistrationUpdate {
            ticket_id: row.get("ticket_id"),
            registered: row.get("registered"),
        })
    })
}
