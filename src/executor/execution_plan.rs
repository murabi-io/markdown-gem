use crate::executor::executable::{Executable, ExecutablePosition};
use crate::fenced_attributes::code_chunk::CodeChunk;
use crate::minimad::{Line, LineParser};

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExecutionItem<'a> {
    Output(Line<'a>),
    Execute(Executable),
}

/// a plan that is just a collection of items to be executed
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct ExecutionPlan<'a> {
    pub plan: Vec<ExecutionItem<'a>>,
}

impl<'s> From<&'s str> for ExecutionPlan<'s> {
    /// build a text from a multi-line string interpreted as markdown
    fn from(md: &str) -> ExecutionPlan<'_> {
        ExecutionPlan::from_md_lines(md.lines())
    }
}

impl<'s> ExecutionPlan<'s> {
    /// parse a text from markdown lines and build the plan.
    ///
    /// ```
    /// use minimad::clean;
    /// use murabi::executor::execution_plan::ExecutionPlan;
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
        I: Iterator<Item = &'s str>,
    {
        let mut plan: Vec<ExecutionItem> = Vec::new();
        let mut between_fences = false;
        let mut code_chunk: Option<CodeChunk> = None;
        let mut current_position: Option<ExecutablePosition> = None;
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
                    plan.push(ExecutionItem::Output(line));
                    if between_fences {
                        code.push_str(format!("{}{}", md_line, LINE_ENDING).as_str());
                    }
                }
            }
        }
        //reverse the plan to use it as a stack from the end
        plan.reverse();
        ExecutionPlan { plan }
    }

    pub fn next(&mut self) -> Option<ExecutionItem> {
        self.plan.pop()
    }
}

/// Tests of text parsing
#[cfg(test)]
mod tests {
    use crate::executor::executable::{Executable, ExecutablePosition};
    use crate::executor::execution_plan::{ExecutionItem, ExecutionPlan, LINE_ENDING};
    use crate::fenced_attributes::{Attributes, CodeChunk};
    use crate::minimad::Compound;
    use crate::*;
    use std::fmt::format;

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
            ..Attributes::default()
        };

        let expected_attribute_2 = Attributes {
            id: Some("code-2".to_string()),
            ..Attributes::default()
        };

        assert_eq!(
            chunks,
            ExecutionPlan {
                plan: vec![
                    ExecutionItem::Output(Line::new_paragraph(vec![Compound::raw_str(
                        "outside 1"
                    )])),
                    ExecutionItem::Output(Line::new_code(Compound::raw_str("a").code())),
                    ExecutionItem::Output(Line::new_code(Compound::raw_str("    b").code())),
                    ExecutionItem::Execute(Executable::new(
                        ExecutablePosition::new(1, 4),
                        Some(CodeChunk {
                            lang: Some(String::from("code")),
                            attributes: expected_attribute_1.clone()
                        }),
                        format!("a{}    b{}", LINE_ENDING, LINE_ENDING)
                    )),
                    ExecutionItem::Output(Line::new_paragraph(vec![Compound::raw_str(
                        "outside 2"
                    )])),
                    ExecutionItem::Output(Line::new_code(Compound::raw_str("a").code())),
                    ExecutionItem::Output(Line::new_code(Compound::raw_str("    b").code())),
                    ExecutionItem::Execute(Executable::new(
                        ExecutablePosition::new(6, 9),
                        Some(CodeChunk {
                            lang: Some(String::from("code")),
                            attributes: expected_attribute_2.clone()
                        }),
                        format!("c{}    d{}", LINE_ENDING, LINE_ENDING)
                    ))
                ]
            },
        );
    }
}
