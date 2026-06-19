use crate::count_profile::DeliverableKind;
use crate::count_profile_variation::Variation;

pub(crate) fn english_variation(kind: DeliverableKind, index: usize) -> Variation {
    let values = match kind {
        DeliverableKind::Narrative => EN_NARRATIVE,
        DeliverableKind::Guide => EN_GUIDE,
        DeliverableKind::Report => EN_REPORT,
        DeliverableKind::General => EN_GENERAL,
    };
    values[index.saturating_sub(1) % values.len()]
}

const EN_NARRATIVE: &[Variation] = &[
    Variation {
        focus: "the recorder",
        context: "an archive stair",
        action: "mark the false timestamp",
        result: "trust shifts toward the unfinished map",
    },
    Variation {
        focus: "the courier",
        context: "a flooded platform",
        action: "hide a cracked token",
        result: "the route becomes a public risk",
    },
    Variation {
        focus: "the steward",
        context: "a quiet engine room",
        action: "refuse the easy repair",
        result: "safety and loyalty split",
    },
    Variation {
        focus: "the apprentice",
        context: "a locked observatory",
        action: "read the missing margin note",
        result: "the promise gains a cost",
    },
    Variation {
        focus: "the witness",
        context: "a market bridge",
        action: "name the wrong beneficiary",
        result: "the crowd changes sides",
    },
    Variation {
        focus: "the keeper",
        context: "a cold signal tower",
        action: "send the delayed warning",
        result: "help arrives with a debt",
    },
];

const EN_GUIDE: &[Variation] = &[
    Variation {
        focus: "workspace boundary",
        context: "raw request",
        action: "normalize the input into one work unit",
        result: "the next step receives a stable premise",
    },
    Variation {
        focus: "handoff packet",
        context: "partial output",
        action: "name the required state before editing",
        result: "later work can resume without guessing",
    },
    Variation {
        focus: "validation probe",
        context: "candidate result",
        action: "run the smallest check that proves the claim",
        result: "failure has a return point",
    },
    Variation {
        focus: "repair branch",
        context: "observed mismatch",
        action: "change one boundary at a time",
        result: "the fix stays auditable",
    },
    Variation {
        focus: "operator note",
        context: "ambiguous instruction",
        action: "record the chosen interpretation",
        result: "future steps inherit the same contract",
    },
    Variation {
        focus: "closure record",
        context: "finished unit",
        action: "save evidence before moving on",
        result: "completion is recoverable",
    },
];

const EN_REPORT: &[Variation] = &[
    Variation {
        focus: "usage evidence",
        context: "current behavior",
        action: "separate observed facts from inference",
        result: "the claim has a visible basis",
    },
    Variation {
        focus: "risk evidence",
        context: "failure mode",
        action: "name the trigger and blast radius",
        result: "the decision carries a guardrail",
    },
    Variation {
        focus: "comparison evidence",
        context: "alternative path",
        action: "state the rejected tradeoff",
        result: "the reader sees why the route holds",
    },
    Variation {
        focus: "cost evidence",
        context: "operational load",
        action: "tie the cost to one measurable pressure",
        result: "follow-up work can be prioritized",
    },
    Variation {
        focus: "quality evidence",
        context: "acceptance condition",
        action: "link the proof to a concrete artifact",
        result: "the conclusion is testable",
    },
    Variation {
        focus: "unknown evidence",
        context: "open question",
        action: "bound what is not yet proven",
        result: "the next review has a target",
    },
];

const EN_GENERAL: &[Variation] = &[
    Variation {
        focus: "scope slice",
        context: "owner objective",
        action: "define the smallest complete unit",
        result: "neighboring files avoid overlap",
    },
    Variation {
        focus: "example slice",
        context: "abstract requirement",
        action: "ground it in one concrete case",
        result: "the reader can inspect the claim",
    },
    Variation {
        focus: "constraint slice",
        context: "available limits",
        action: "state the rule that shapes the output",
        result: "later work respects the boundary",
    },
    Variation {
        focus: "decision slice",
        context: "branching option",
        action: "choose and explain one path",
        result: "the sequence keeps momentum",
    },
    Variation {
        focus: "verification slice",
        context: "completion claim",
        action: "name the proof artifact",
        result: "the unit can be checked",
    },
    Variation {
        focus: "handoff slice",
        context: "remaining work",
        action: "leave the next term and open point",
        result: "continuation stays structured",
    },
];
