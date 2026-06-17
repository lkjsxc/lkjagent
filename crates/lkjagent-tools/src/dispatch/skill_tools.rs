use std::collections::BTreeMap;

use lkjagent_context::budget::{LOG_LOADED_SKILLS, LOG_SKILL_BODY};

use crate::dispatch::params::param;
use crate::dispatch::{finish, observe_error, observe_result};
use crate::dispatch::{DispatchOutput, DispatchState, LoadedSkillRecord, ToolRuntime};
use crate::observe;
use crate::skill;

pub fn dispatch_skill_use(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    let name = param(params, "name");
    if let Some(record) = state
        .loaded_skills
        .iter()
        .find(|record| record.name == name)
    {
        return finish(
            state,
            action_text,
            observe::notice(
                "error",
                format!("skill already loaded in frame {}", record.frame_ref),
            ),
        );
    }
    match skill::use_skill(&runtime.skill_library, &name) {
        Err(error) => observe_error(error, action_text, runtime, state),
        Ok(loaded) => finish_loaded_skill(loaded, action_text, state),
    }
}

pub fn dispatch_skill_save(
    params: &BTreeMap<String, String>,
    action_text: &str,
    runtime: &ToolRuntime,
    state: &mut DispatchState,
) -> DispatchOutput {
    observe_result(
        skill::save_skill(
            &runtime.skill_library,
            &param(params, "name"),
            &param(params, "content"),
        ),
        action_text,
        runtime,
        state,
    )
}

fn finish_loaded_skill(
    loaded: skill::LoadedSkill,
    action_text: &str,
    state: &mut DispatchState,
) -> DispatchOutput {
    let tokens = observe::estimate_tokens(&loaded.frame);
    if tokens > LOG_SKILL_BODY {
        return finish(
            state,
            action_text,
            observe::notice("error", "skill body exceeds 2,048 tokens"),
        );
    }
    if state.loaded_skill_tokens.saturating_add(tokens) > LOG_LOADED_SKILLS {
        return finish(
            state,
            action_text,
            observe::notice("error", "loaded skills exceed 6,144 tokens"),
        );
    }
    let output = finish(
        state,
        action_text,
        observe::skill(&loaded.name, loaded.frame),
    );
    state.loaded_skill_tokens = state.loaded_skill_tokens.saturating_add(tokens);
    state.loaded_skills.push(LoadedSkillRecord {
        name: loaded.name,
        frame_ref: output.frame_ref,
        tokens,
    });
    output
}
