#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    Interrupt,
    Terminate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShutdownState {
    pub stop_requested: bool,
    pub in_flight: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShutdownDecision {
    Continue,
    FinishTurnThenExit,
    ExitNow,
}

pub fn request_shutdown(
    state: ShutdownState,
    _signal: Signal,
) -> (ShutdownState, ShutdownDecision) {
    let next = ShutdownState {
        stop_requested: true,
        in_flight: state.in_flight,
    };
    let decision = if state.in_flight {
        ShutdownDecision::FinishTurnThenExit
    } else {
        ShutdownDecision::ExitNow
    };
    (next, decision)
}
