use lkjagent_context::budget::ContextPressure;

pub(super) fn pressure_name(pressure: ContextPressure) -> &'static str {
    match pressure {
        ContextPressure::Green => "green",
        ContextPressure::Yellow => "yellow",
        ContextPressure::Orange => "orange",
        ContextPressure::Red => "red",
        ContextPressure::BlackInvalid => "black-invalid",
    }
}
