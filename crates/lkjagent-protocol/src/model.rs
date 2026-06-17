#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Action {
    pub tool: String,
    pub params: Vec<Param>,
}

impl Action {
    pub fn new(tool: impl Into<String>, params: Vec<Param>) -> Self {
        Self {
            tool: tool.into(),
            params,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Param {
    pub name: String,
    pub value: String,
}

impl Param {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseFault {
    MissingAct,
    MultipleAct,
    MissingTool,
    UnknownTool {
        tool: String,
    },
    UnclosedTag {
        tag: String,
    },
    DuplicateParam {
        name: String,
    },
    BadParams {
        missing: Vec<String>,
        unknown: Vec<String>,
    },
}
