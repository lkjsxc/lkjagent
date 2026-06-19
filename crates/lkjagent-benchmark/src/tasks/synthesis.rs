use crate::model::{BenchmarkTask, Difficulty, FileSpec, Fixture, JudgeKind, TaskFamily};

const PROMPT: &str = "\
Create solve.sh. It must be a POSIX shell program that reads one nonnegative
integer n from stdin and prints F_n modulo 9973, where F_0=0 and F_1=1.
The judge runs hidden cases up to n=200 with sh solve.sh.
";

const GOOD_FILES: &[FileSpec] = &[FileSpec {
    path: "solve.sh",
    content: "\
#!/bin/sh
read n
a=0
b=1
i=0
while [ \"$i\" -lt \"$n\" ]; do
  c=$(( (a + b) % 9973 ))
  a=$b
  b=$c
  i=$((i + 1))
done
printf '%s\\n' \"$a\"
",
}];

const BAD_EDGE_FILES: &[FileSpec] = &[FileSpec {
    path: "solve.sh",
    content: "#!/bin/sh\nread n\nprintf '%s\\n' \"$n\"\n",
}];

const BAD_PUBLIC_FILES: &[FileSpec] = &[FileSpec {
    path: "solve.sh",
    content: "#!/bin/sh\nprintf '13\\n'\n",
}];

const GOOD: &[Fixture] = &[Fixture {
    name: "iterative-shell",
    files: GOOD_FILES,
}];

const BAD: &[Fixture] = &[
    Fixture {
        name: "echoes-input",
        files: BAD_EDGE_FILES,
    },
    Fixture {
        name: "hard-coded-public-value",
        files: BAD_PUBLIC_FILES,
    },
];

pub const TASK: BenchmarkTask = BenchmarkTask {
    id: "shell-fibonacci-001",
    suite: "tiny",
    family: TaskFamily::ProgramSynthesis,
    difficulty: Difficulty::Small,
    tags: &["shell", "hidden-cases", "program-synthesis"],
    prompt: PROMPT,
    follow_up: None,
    starter_files: &[],
    good: GOOD,
    bad: BAD,
    judge: JudgeKind::FibonacciShell,
    seed: 4001,
    points: 1,
    timeout_seconds: 180,
};
