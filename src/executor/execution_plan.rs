use lazy_static::lazy_static;
use std::env;

use crate::executor::executable::{Executable, ExecutablePosition};
use crate::fenced_attributes::code_chunk::CodeChunk;
use crate::fenced_attributes::Attributes;
use crate::minimad::{Line, LineParser};

#[cfg(linux)]
use sys_info;

#[cfg(linux)]
lazy_static! {
    pub static ref LINUX_ID_LIKE: Option<String> = sys_info::linux_os_release()
        .map(|v| -> Option<String> { v.id_like })
        .ok()
        .flatten();
}

#[cfg(not(linux))]
lazy_static! {
    pub static ref LINUX_ID_LIKE: Option<String> = None;
}

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExecutionItem {
    OutputString(String),
    OutputCode(String),
    Execute(Executable),
}

/// a plan that is just a collection of items to be executed
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct ExecutionPlan {
    pub plan: Vec<ExecutionItem>,
}

impl From<&str> for ExecutionPlan {
    /// build a text from a multi-line string interpreted as markdown
    fn from(md: &str) -> ExecutionPlan {
        ExecutionPlan::from_md_lines(md.lines())
    }
}

impl<'a> ExecutionPlan {
    /// parse a text from markdown lines and build the plan.
    ///
    /// ```
    /// use minimad::clean;
    /// use gem::executor::execution_plan::ExecutionPlan;
    ///
    /// let md = clean::lines(r#"
    ///     * some bullet item
    ///     some text
    ///         some_code();
    /// "#);
    /// let text = ExecutionPlan::from_md_lines(md.into_iter());
    /// ```
    pub fn from_md_lines<I>(md_lines: I) -> Self
    where
        I: Iterator<Item = &'a str>,
    {
        let mut plan: Vec<ExecutionItem> = Vec::new();
        let mut between_fences = false;
        let mut code_chunk: Option<CodeChunk> = None;
        let mut current_position: Option<ExecutablePosition> = None;

        // code for to be executed as part of Execute command
        let mut code = String::new();
        for (idx, md_line) in md_lines.enumerate() {
            let parser = LineParser::from(md_line);
            let line = if between_fences {
                parser.as_code()
            } else {
                parser.line()
            };
            match line {
                Line::CodeFence(..) if between_fences == true => {
                    let position = current_position.clone().unwrap();
                    let position = position.end(idx);
                    let executable = Executable::new(position.clone(), code_chunk, code.clone());

                    plan.push(ExecutionItem::Execute(executable));

                    between_fences = !between_fences;
                    code_chunk = None;
                }
                Line::CodeFence(..) if between_fences == false => {
                    between_fences = !between_fences;
                    code = String::new();
                    code_chunk = CodeChunk::parse_fences(String::from(md_line));
                    current_position = Some(ExecutablePosition::start(idx));
                }
                _ => {
                    if between_fences {
                        plan.push(ExecutionItem::OutputCode(String::from(md_line)));
                        code.push_str(format!("{}{}", md_line, LINE_ENDING).as_str());
                    } else {
                        plan.push(ExecutionItem::OutputString(String::from(md_line)));
                    }
                }
            }
        }
        //reverse the plan to use it as a stack
        plan.reverse();
        plan = plan
            .into_iter()
            .filter(|item| match item {
                ExecutionItem::Execute(e) if e.code_chunk.is_some() => {
                    let attrs = &e.code_chunk.as_ref().unwrap().attributes;
                    Self::by_sys(attrs) && Self::by_arch(attrs) && Self::by_linux_distro(attrs)
                }
                _ => true,
            })
            .collect();
        ExecutionPlan { plan }
    }

    fn by_sys(attrs: &Attributes) -> bool {
        attrs.sys.is_none()
            || attrs
                .sys
                .as_ref()
                .unwrap()
                .contains(&env::consts::OS.to_owned())
    }

    fn by_arch(attrs: &Attributes) -> bool {
        attrs.arch.is_none()
            || attrs
                .arch
                .as_ref()
                .unwrap()
                .contains(&env::consts::ARCH.to_owned())
    }

    fn by_linux_distro(attrs: &Attributes) -> bool {
        attrs.linux_distro.is_none()
            || LINUX_ID_LIKE.is_some()
                && attrs
                    .linux_distro
                    .as_ref()
                    .unwrap()
                    .contains(LINUX_ID_LIKE.as_ref().unwrap())
    }

    pub fn next(&mut self) -> Option<ExecutionItem> {
        self.plan.pop()
    }
}

/// Tests of text parsing
#[cfg(test)]
mod tests {
    use termimad::minimad::clean;

    use crate::executor::executable::{Executable, ExecutablePosition};
    use crate::executor::execution_plan::{ExecutionItem, ExecutionPlan, LINE_ENDING};
    use crate::fenced_attributes::{Attributes, CodeChunk};

    #[test]
    fn indented_code_between_fences() {
        let md = clean::lines(
            r#"
            outside 1
            ```code {id=code-1}
            a
                b
            ```
            outside 2
            ```code {id=code-2}
            c
                d
            ```
        "#,
        );
        let chunks = ExecutionPlan::from_md_lines(md.into_iter());

        let expected_attribute_1 = Attributes {
            id: Some("code-1".to_string()),
            as_file: true,
            stdout: true,
            allow_warnings: true,
            ..Attributes::default()
        };

        let expected_attribute_2 = Attributes {
            id: Some("code-2".to_string()),
            as_file: true,
            stdout: true,
            allow_warnings: true,
            ..Attributes::default()
        };

        let mut expected_plan = vec![
            ExecutionItem::OutputString("outside 1".to_string()),
            ExecutionItem::OutputCode("a".to_string()),
            ExecutionItem::OutputCode("    b".to_string()),
            ExecutionItem::Execute(Executable::new(
                ExecutablePosition::new(1, 4),
                Some(CodeChunk {
                    lang: Some(String::from("code")),
                    attributes: expected_attribute_1.clone(),
                }),
                format!("a{}    b{}", LINE_ENDING, LINE_ENDING),
            )),
            ExecutionItem::OutputString("outside 2".to_string()),
            ExecutionItem::OutputCode("c".to_string()),
            ExecutionItem::OutputCode("    d".to_string()),
            ExecutionItem::Execute(Executable::new(
                ExecutablePosition::new(6, 9),
                Some(CodeChunk {
                    lang: Some(String::from("code")),
                    attributes: expected_attribute_2.clone(),
                }),
                format!("c{}    d{}", LINE_ENDING, LINE_ENDING),
            )),
        ];
        expected_plan.reverse();
        assert_eq!(
            chunks,
            ExecutionPlan {
                plan: expected_plan
            },
        );
    }
}
