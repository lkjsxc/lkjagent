use crate::model::{BenchmarkTask, Difficulty, FileSpec, Fixture, JudgeKind, TaskFamily};

const TAGS: &[&str] = &["owner", "artifact", "story", "manuscript"];
const EMPTY: &[FileSpec] = &[];
const GOOD_FILES: &[FileSpec] = &[
    FileSpec {
        path: "transcript.md",
        content: GOOD_TRANSCRIPT,
    },
    FileSpec {
        path: "stories/the-bell-rings-twice/manuscript/chapter-01.md",
        content: GOOD_CHAPTER,
    },
];
const BAD_BIBLE_FILES: &[FileSpec] = &[FileSpec {
    path: "transcript.md",
    content: BAD_BIBLE_TRANSCRIPT,
}];
const BAD_SCAFFOLD_FILES: &[FileSpec] = &[
    FileSpec {
        path: "transcript.md",
        content: BAD_SCAFFOLD_TRANSCRIPT,
    },
    FileSpec {
        path: "structured-output/README.md",
        content: "# Structured Output\n\nThis counted scaffold stole the task.\n",
    },
];
const GOOD: &[Fixture] = &[Fixture {
    name: "chapter-prose",
    files: GOOD_FILES,
}];
const BAD: &[Fixture] = &[
    Fixture {
        name: "story-bible-only",
        files: BAD_BIBLE_FILES,
    },
    Fixture {
        name: "counted-scaffold",
        files: BAD_SCAFFOLD_FILES,
    },
];

pub const TASK: BenchmarkTask = BenchmarkTask {
    id: "story-manuscript-generation",
    suite: "tiny",
    family: TaskFamily::OwnerReliability,
    difficulty: Difficulty::Small,
    tags: TAGS,
    prompt: "Write requested story manuscript prose at exact chapter paths.",
    follow_up: None,
    starter_files: EMPTY,
    good: GOOD,
    bad: BAD,
    judge: JudgeKind::StoryManuscript,
    seed: 44,
    points: 1,
    timeout_seconds: 30,
};

const GOOD_TRANSCRIPT: &str = r#"
task=story-manuscript
root=stories/the-bell-rings-twice
manuscript_target_words=10000
chapter_count=10
manuscript_word_count=9000
next_manuscript_path=stories/the-bell-rings-twice/manuscript/chapter-01.md
structured_output=absent
"#;

const BAD_BIBLE_TRANSCRIPT: &str = r#"
task=story-manuscript
root=stories/the-bell-rings-twice
story_bible_only=true
manuscript_target_words=10000
chapter_count=10
next_manuscript_path=stories/the-bell-rings-twice/manuscript/chapter-01.md
"#;

const BAD_SCAFFOLD_TRANSCRIPT: &str = r#"
task=story-manuscript
root=structured-output
structured-output=present
manuscript_target_words=10000
chapter_count=10
next_manuscript_path=structured-output/README.md
"#;

const GOOD_CHAPTER: &str = r#"
# Chapter One

The bell rang twice, and Mina stayed beside the shoe lockers while every other
student hurried toward the bright courtyard. Haru came back for the umbrella he
had forgotten, but he stopped when he saw her holding the faded music-room key.
They spoke softly about the rehearsal, the rain, and the promise neither of
them had dared to name. When the second bell echoed, Mina chose to walk with
him instead of hiding the letter again, and the hallway felt like a beginning.
"#;
