use lkjagent_core::owner_turn::{route_turn, RouteContext};
use rusqlite::{params, Connection};

use crate::error::StoreResult;
use crate::plan_rows::QueueRow;

pub fn enqueue(conn: &Connection, content: &str, now: &str) -> StoreResult<i64> {
    enqueue_with_force(conn, content, false, now)
}

pub fn enqueue_with_force(
    conn: &Connection,
    content: &str,
    force_new: bool,
    now: &str,
) -> StoreResult<i64> {
    let route = route_fields(
        content,
        RouteContext {
            force_new,
            ..RouteContext::default()
        },
    );
    conn.execute(
        "INSERT INTO queue (content, state, force_new, created_at, route_lane,
         route_durability, route_title_seed, route_transform_allowed)
         VALUES (?1, 'pending', ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            content,
            i64::from(force_new),
            now,
            route.lane,
            route.durability,
            route.title,
            route.allowed
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn deliver_next(conn: &Connection, task_id: i64, now: &str) -> StoreResult<Option<QueueRow>> {
    deliver_matching(conn, task_id, None, now)
}

pub fn mark_recorded(conn: &Connection, queue_id: i64, now: &str) -> StoreResult<()> {
    conn.execute(
        "UPDATE queue SET state = 'recorded', delivered_at = ?1 WHERE id = ?2",
        params![now, queue_id],
    )?;
    Ok(())
}

pub fn deliver_answer(conn: &Connection, task_id: i64, now: &str) -> StoreResult<Option<QueueRow>> {
    deliver_matching(conn, task_id, Some(false), now)
}

pub fn deliver_forced_new(
    conn: &Connection,
    task_id: i64,
    now: &str,
) -> StoreResult<Option<QueueRow>> {
    deliver_matching(conn, task_id, Some(true), now)
}

pub fn deliver_matter_update(
    conn: &Connection,
    task_id: i64,
    now: &str,
) -> StoreResult<Option<QueueRow>> {
    let row = next_pending_matching(conn, Some(false))?;
    let Some(row) = row else { return Ok(None) };
    let context = RouteContext {
        open_matter: true,
        ..RouteContext::default()
    };
    let route = route_fields(&row.content, context);
    if route.lane.as_deref() != Some("existing_matter")
        || route.durability.as_deref() != Some("matter_update")
    {
        return Ok(None);
    }
    update_delivered(conn, row, task_id, now, route).map(Some)
}

fn deliver_matching(
    conn: &Connection,
    task_id: i64,
    force_new: Option<bool>,
    now: &str,
) -> StoreResult<Option<QueueRow>> {
    let row = next_pending_matching(conn, force_new)?;
    let Some(row) = row else { return Ok(None) };
    let context = delivery_context(force_new, row.force_new);
    let route = route_fields(&row.content, context);
    update_delivered(conn, row, task_id, now, route).map(Some)
}

fn update_delivered(
    conn: &Connection,
    row: QueueRow,
    task_id: i64,
    now: &str,
    route: RouteFields,
) -> StoreResult<QueueRow> {
    conn.execute(
        "UPDATE queue SET state = 'delivered', delivered_at = ?1, task_id = ?2,
         route_lane = ?4, route_durability = ?5, route_title_seed = ?6,
         route_transform_allowed = ?7 WHERE id = ?3",
        params![
            now,
            task_id,
            row.id,
            route.lane,
            route.durability,
            route.title,
            route.allowed
        ],
    )?;
    Ok(QueueRow {
        state: "delivered".to_string(),
        task_id: Some(task_id),
        route_lane: route.lane,
        route_durability: route.durability,
        route_title_seed: route.title,
        route_transform_allowed: route.allowed.map(|value| value != 0),
        ..row
    })
}

pub fn next_pending(conn: &Connection) -> StoreResult<Option<QueueRow>> {
    next_pending_matching(conn, None)
}

fn next_pending_matching(
    conn: &Connection,
    force_new: Option<bool>,
) -> StoreResult<Option<QueueRow>> {
    let clause = match force_new {
        Some(true) => " AND force_new = 1",
        Some(false) => " AND force_new = 0",
        None => "",
    };
    let sql = format!(
        "SELECT id, content, state, task_id, force_new, route_lane, route_durability,
         route_title_seed, route_transform_allowed FROM queue
         WHERE state = 'pending'{clause} ORDER BY id LIMIT 1",
    );
    let mut statement = conn.prepare(&sql)?;
    let mut rows = statement.query([])?;
    let Some(row) = rows.next()? else {
        return Ok(None);
    };
    Ok(Some(QueueRow {
        id: row.get(0)?,
        content: row.get(1)?,
        state: row.get(2)?,
        task_id: row.get(3)?,
        force_new: row.get::<_, i64>(4)? != 0,
        route_lane: row.get(5)?,
        route_durability: row.get(6)?,
        route_title_seed: row.get(7)?,
        route_transform_allowed: row.get::<_, Option<i64>>(8)?.map(|value| value != 0),
    }))
}

fn delivery_context(force_new: Option<bool>, row_force_new: bool) -> RouteContext {
    RouteContext {
        waiting_matter: force_new == Some(false),
        force_new: row_force_new || force_new == Some(true),
        ..RouteContext::default()
    }
}

struct RouteFields {
    lane: Option<String>,
    durability: Option<String>,
    title: Option<String>,
    allowed: Option<i64>,
}

fn route_fields(content: &str, context: RouteContext) -> RouteFields {
    route_turn(content, context).map_or(
        RouteFields {
            lane: None,
            durability: None,
            title: None,
            allowed: None,
        },
        |route| RouteFields {
            lane: Some(route.lane),
            durability: Some(route.desired_durability),
            title: Some(route.title_seed),
            allowed: Some(i64::from(route.transformation_allowed)),
        },
    )
}
