use crate::model::{GraphNodeId, TaskPhase};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateTrackId(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatePosture {
    Exploring,
    Structuring,
    Implementing,
    Verifying,
    Recovering,
    Waiting,
    Maintaining,
    Closing,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateTrack {
    pub id: StateTrackId,
    pub label: String,
    pub posture: StatePosture,
    pub intensity: u8,
    pub confidence: u8,
    pub phase: TaskPhase,
    pub active_node: GraphNodeId,
    pub evidence_gap: Vec<String>,
    pub next_affordances: Vec<String>,
    pub risk: Vec<String>,
    pub last_update_turn: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RankedStateTrack {
    pub rank_score: u8,
    pub track: StateTrack,
}

impl StatePosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exploring => "Exploring",
            Self::Structuring => "Structuring",
            Self::Implementing => "Implementing",
            Self::Verifying => "Verifying",
            Self::Recovering => "Recovering",
            Self::Waiting => "Waiting",
            Self::Maintaining => "Maintaining",
            Self::Closing => "Closing",
        }
    }
}

impl StateTrack {
    pub fn new(
        id: &str,
        label: &str,
        posture: StatePosture,
        intensity: u8,
        confidence: u8,
        phase: TaskPhase,
        active_node: GraphNodeId,
        gaps: &[&str],
    ) -> Self {
        Self {
            id: StateTrackId(id.to_string()),
            label: label.to_string(),
            posture,
            intensity,
            confidence,
            phase,
            active_node,
            evidence_gap: gaps.iter().map(|gap| (*gap).to_string()).collect(),
            next_affordances: Vec::new(),
            risk: Vec::new(),
            last_update_turn: None,
        }
    }

    pub fn rank_score(&self, current_turn: u64, user_priority: u8) -> u8 {
        let recency = recency_score(self.last_update_turn, current_turn);
        let gap = gap_urgency(&self.evidence_gap);
        let mut score = (40 * u16::from(self.intensity)
            + 25 * u16::from(recency)
            + 20 * u16::from(gap)
            + 10 * u16::from(user_priority)
            + 5 * u16::from(self.confidence))
            / 100;
        if matches!(self.phase, TaskPhase::Closed) {
            score = score.saturating_sub(30);
        }
        score.min(100) as u8
    }
}

pub fn ranked_state_tracks(
    tracks: &[StateTrack],
    current_turn: u64,
    user_priority: u8,
) -> Vec<RankedStateTrack> {
    let mut ranked = tracks
        .iter()
        .cloned()
        .map(|track| RankedStateTrack {
            rank_score: track.rank_score(current_turn, user_priority),
            track,
        })
        .collect::<Vec<_>>();
    ranked.sort_by(|left, right| {
        right
            .rank_score
            .cmp(&left.rank_score)
            .then_with(|| left.track.id.cmp(&right.track.id))
    });
    ranked
}

pub fn promote_recovery_track(
    tracks: &mut Vec<StateTrack>,
    label: &str,
    active_node: GraphNodeId,
    phase: TaskPhase,
) {
    if let Some(track) = tracks.iter_mut().find(|track| track.id.0 == "recovery") {
        track.label = label.to_string();
        track.posture = StatePosture::Recovering;
        track.intensity = track.intensity.max(90);
        track.confidence = track.confidence.max(70);
        track.phase = phase;
        track.active_node = active_node;
        track.evidence_gap = vec!["recovery evidence".to_string()];
        track.next_affordances = vec![
            "inspect-state".to_string(),
            "repair-action-shape".to_string(),
        ];
        return;
    }
    tracks.push(StateTrack::new(
        "recovery",
        label,
        StatePosture::Recovering,
        90,
        70,
        phase,
        active_node,
        &["recovery evidence"],
    ));
}

pub fn render_ranked_tracks(tracks: &[StateTrack], limit: usize) -> String {
    let ranked = ranked_state_tracks(tracks, 0, 80);
    if ranked.is_empty() {
        return "none".to_string();
    }
    ranked
        .into_iter()
        .take(limit)
        .enumerate()
        .map(|(index, item)| {
            let gap = item
                .track
                .evidence_gap
                .first()
                .map_or("none", String::as_str);
            format!(
                "{}. {} {} {} phase={} gap={}",
                index + 1,
                item.track.posture.as_str(),
                score_decimal(item.rank_score),
                item.track.label,
                item.track.phase.as_str(),
                gap
            )
        })
        .collect::<Vec<_>>()
        .join("; ")
}

pub fn score_decimal(score: u8) -> String {
    format!("{}.{:02}", score / 100, score % 100)
}

fn recency_score(last_update_turn: Option<u64>, current_turn: u64) -> u8 {
    last_update_turn.map_or(40, |turn| {
        100_u8.saturating_sub(current_turn.saturating_sub(turn).min(100) as u8)
    })
}

fn gap_urgency(gaps: &[String]) -> u8 {
    (gaps.len().saturating_mul(25)).min(100) as u8
}
