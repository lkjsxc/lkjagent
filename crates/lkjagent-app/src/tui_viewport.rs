#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Anchor {
    pub message_id: String,
    pub wrapped_row: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Viewport {
    #[default]
    Follow,
    Manual(Anchor),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ViewRow {
    pub message_id: String,
    pub wrapped_row: usize,
    pub role: String,
    pub text: String,
}

pub fn visible(viewport: &Viewport, rows: &[ViewRow], height: usize) -> Vec<ViewRow> {
    if rows.is_empty() || height == 0 {
        return Vec::new();
    }
    rows.iter()
        .skip(start(viewport, rows, height))
        .take(height)
        .cloned()
        .collect()
}

pub fn scroll(viewport: &mut Viewport, rows: &[ViewRow], height: usize, delta: isize) {
    if rows.is_empty() || height == 0 {
        return;
    }
    let maximum = rows.len().saturating_sub(height);
    let current = start(viewport, rows, height);
    let target = if delta.is_negative() {
        current.saturating_sub(delta.unsigned_abs())
    } else {
        current.saturating_add(delta as usize).min(maximum)
    };
    if target == maximum {
        *viewport = Viewport::Follow;
    } else if let Some(row) = rows.get(target) {
        *viewport = Viewport::Manual(anchor(row));
    }
}

pub fn reconcile(viewport: &mut Viewport, rows: &[ViewRow], height: usize) {
    let Viewport::Manual(_) = viewport else {
        return;
    };
    if rows.is_empty() {
        *viewport = Viewport::Follow;
        return;
    }
    let index = start(viewport, rows, height.max(1));
    if let Some(row) = rows.get(index) {
        *viewport = Viewport::Manual(anchor(row));
    }
}

fn start(viewport: &Viewport, rows: &[ViewRow], height: usize) -> usize {
    let maximum = rows.len().saturating_sub(height);
    match viewport {
        Viewport::Follow => maximum,
        Viewport::Manual(anchor) => anchor_index(anchor, rows).min(maximum),
    }
}

fn anchor_index(anchor: &Anchor, rows: &[ViewRow]) -> usize {
    if let Some(index) = rows.iter().position(|row| {
        row.message_id == anchor.message_id && row.wrapped_row == anchor.wrapped_row
    }) {
        return index;
    }
    rows.iter()
        .enumerate()
        .filter(|(_, row)| row.message_id == anchor.message_id)
        .map(|(index, _)| index)
        .next_back()
        .unwrap_or_else(|| rows.len().saturating_sub(1))
}

fn anchor(row: &ViewRow) -> Anchor {
    Anchor {
        message_id: row.message_id.clone(),
        wrapped_row: row.wrapped_row,
    }
}
