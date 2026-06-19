use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
struct Dfa {
    start: String,
    accept: BTreeSet<String>,
    transitions: BTreeMap<(String, char), String>,
    states: BTreeSet<String>,
}

pub fn judge(workspace: &Path) -> Result<(), String> {
    let text = fs::read_to_string(workspace.join("dfa.txt"))
        .map_err(|error| format!("dfa.txt missing or unreadable: {error}"))?;
    let dfa = parse(&text)?;
    assert_total(&dfa)?;
    equivalent_to_even_ones(&dfa)
}

fn parse(text: &str) -> Result<Dfa, String> {
    let mut start = None;
    let mut accept = BTreeSet::new();
    let mut transitions = BTreeMap::new();
    let mut states = BTreeSet::new();
    for raw in text.lines().map(str::trim) {
        if raw.is_empty() || raw.starts_with('#') {
            continue;
        }
        if let Some(value) = raw.strip_prefix("start:") {
            start = Some(value.trim().to_string());
            states.insert(value.trim().to_string());
        } else if let Some(value) = raw.strip_prefix("accept:") {
            for state in value.split(|ch: char| ch == ',' || ch.is_whitespace()) {
                if !state.trim().is_empty() {
                    accept.insert(state.trim().to_string());
                    states.insert(state.trim().to_string());
                }
            }
        } else {
            parse_transition(raw, &mut transitions, &mut states)?;
        }
    }
    let Some(start) = start else {
        return Err("missing start line".to_string());
    };
    Ok(Dfa {
        start,
        accept,
        transitions,
        states,
    })
}

fn parse_transition(
    line: &str,
    transitions: &mut BTreeMap<(String, char), String>,
    states: &mut BTreeSet<String>,
) -> Result<(), String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 3 {
        return Err(format!("bad transition line: {line}"));
    }
    let symbol = match parts[1] {
        "0" => '0',
        "1" => '1',
        other => return Err(format!("symbol must be 0 or 1, got {other}")),
    };
    let key = (parts[0].to_string(), symbol);
    if transitions.contains_key(&key) {
        return Err(format!("duplicate transition {} {symbol}", parts[0]));
    }
    states.insert(parts[0].to_string());
    states.insert(parts[2].to_string());
    transitions.insert(key, parts[2].to_string());
    Ok(())
}

fn assert_total(dfa: &Dfa) -> Result<(), String> {
    for state in &dfa.states {
        for symbol in ['0', '1'] {
            if !dfa.transitions.contains_key(&(state.clone(), symbol)) {
                return Err(format!("missing transition {state} {symbol}"));
            }
        }
    }
    Ok(())
}

fn equivalent_to_even_ones(dfa: &Dfa) -> Result<(), String> {
    let mut queue = VecDeque::from([(dfa.start.clone(), true)]);
    let mut seen = BTreeSet::new();
    while let Some((state, reference_even)) = queue.pop_front() {
        if !seen.insert((state.clone(), reference_even)) {
            continue;
        }
        if dfa.accept.contains(&state) != reference_even {
            return Err(format!("acceptance mismatch at state {state}"));
        }
        for symbol in ['0', '1'] {
            let next = dfa
                .transitions
                .get(&(state.clone(), symbol))
                .ok_or_else(|| format!("missing transition {state} {symbol}"))?;
            queue.push_back((next.clone(), ref_next(reference_even, symbol)));
        }
    }
    Ok(())
}

fn ref_next(even: bool, symbol: char) -> bool {
    match symbol {
        '1' => !even,
        _ => even,
    }
}
