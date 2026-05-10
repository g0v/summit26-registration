use sqlx::{PgPool, Row};

use crate::models::{Attendee, RegistrationTable, RegistrationUpdate, Worker};

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

pub async fn list_workers(db: &PgPool) -> Result<Vec<Worker>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
            SELECT nickname, ticket_id, team, role, registered
            FROM workers
            ORDER BY team
        "#,
    )
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| Worker {
            nickname: row.get("nickname"),
            ticket_id: row.get("ticket_id"),
            team: row.get("team"),
            role: row.get("role"),
            registered: row.get("registered"),
        })
        .collect())
}

pub async fn update_registration(
    table: RegistrationTable,
    db: &PgPool,
    update: &RegistrationUpdate,
) -> Result<Option<RegistrationUpdate>, sqlx::Error> {
    let sql = match table {
        RegistrationTable::Attendees => {
            r#"
            UPDATE attendees
            SET registered = $1, updated_at = NOW()
            WHERE ticket_id = $2
            RETURNING ticket_id, registered
        "#
        }
        RegistrationTable::Workers => {
            r#"
            UPDATE workers
            SET registered = $1, updated_at = NOW()
            WHERE ticket_id = $2
            RETURNING ticket_id, registered
        "#
        }
    };

    sqlx::query(sql)
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
