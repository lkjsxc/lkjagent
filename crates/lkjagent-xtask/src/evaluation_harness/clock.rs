use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fault {
    pub injection_id: String,
    pub boundary: String,
    pub outcome: String,
    pub advance_ms: u64,
}

#[derive(Default)]
pub struct FakeClock {
    now_ms: u64,
}

impl FakeClock {
    pub fn now_ms(&self) -> u64 {
        self.now_ms
    }

    pub fn advance_to(&mut self, target_ms: u64) -> Result<(), String> {
        if target_ms < self.now_ms {
            return Err("fake clock monotonic regression".into());
        }
        self.now_ms = target_ms;
        Ok(())
    }

    fn advance_by(&mut self, amount_ms: u64) -> Result<(), String> {
        let target = self
            .now_ms
            .checked_add(amount_ms)
            .ok_or_else(|| "fake clock overflow".to_string())?;
        self.advance_to(target)
    }
}

#[derive(Clone)]
pub struct FaultInjector {
    faults: Vec<Fault>,
    cursor: usize,
}

impl FaultInjector {
    pub fn from_path(path: &Path) -> Result<Self, String> {
        let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
        let mut faults = Vec::new();
        for (index, line) in text.lines().enumerate().skip(1) {
            let fields = line.split('\t').collect::<Vec<_>>();
            if fields.len() != 4 || fields.iter().any(|field| field.is_empty()) {
                return Err(format!("fault row {} is malformed", index + 1));
            }
            let advance_ms = fields[3]
                .parse::<u64>()
                .map_err(|_| format!("fault row {} advance is invalid", index + 1))?;
            faults.push(Fault {
                injection_id: fields[0].into(),
                boundary: fields[1].into(),
                outcome: fields[2].into(),
                advance_ms,
            });
        }
        if faults.len() < 10 {
            return Err("fault schedule has fewer than ten injections".into());
        }
        let ids = faults
            .iter()
            .map(|fault| fault.injection_id.as_str())
            .collect::<BTreeSet<_>>();
        if ids.len() != faults.len() {
            return Err("fault injection IDs are not unique".into());
        }
        Ok(Self { faults, cursor: 0 })
    }

    pub fn faults(&self) -> &[Fault] {
        &self.faults
    }

    pub fn consume(
        &mut self,
        injection_id: &str,
        boundary: &str,
        clock: &mut FakeClock,
    ) -> Result<String, String> {
        let Some(expected) = self.faults.get(self.cursor) else {
            return Err("fault schedule was consumed more than once".into());
        };
        if expected.injection_id != injection_id || expected.boundary != boundary {
            return Err(format!(
                "fault order mismatch: expected {} at {}",
                expected.injection_id, expected.boundary
            ));
        }
        clock.advance_by(expected.advance_ms)?;
        self.cursor += 1;
        Ok(expected.outcome.clone())
    }

    pub fn finish(&self) -> Result<(), String> {
        if self.cursor == self.faults.len() {
            Ok(())
        } else {
            Err(format!(
                "{} declared faults were not consumed",
                self.faults.len() - self.cursor
            ))
        }
    }
}

pub fn exercise(path: &Path) -> Result<BTreeSet<String>, Vec<String>> {
    let schedule = FaultInjector::from_path(path).map_err(|error| vec![error])?;
    let mut replay = schedule.clone();
    let mut clock = FakeClock::default();
    for fault in schedule.faults() {
        replay
            .consume(&fault.injection_id, &fault.boundary, &mut clock)
            .map_err(|error| vec![error])?;
    }
    replay.finish().map_err(|error| vec![error])?;
    if clock.now_ms() == 0 {
        return Err(vec!["fault replay did not advance fake time".into()]);
    }
    let mut out_of_order = schedule.clone();
    let second = schedule
        .faults()
        .get(1)
        .ok_or_else(|| vec!["fault schedule lacks an order fixture".into()])?;
    if out_of_order
        .consume(
            &second.injection_id,
            &second.boundary,
            &mut FakeClock::default(),
        )
        .is_ok()
    {
        return Err(vec!["out-of-order fault was accepted".into()]);
    }
    Ok(schedule
        .faults()
        .iter()
        .map(|fault| fault.injection_id.clone())
        .collect())
}
