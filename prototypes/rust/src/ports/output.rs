use crate::domain::Yak;

pub enum OutputFormat {
    Markdown,
    Plain,
}

pub enum YakFilter {
    All,
    NotDone,
    Done,
}

pub trait OutputFormatter {
    fn format_yak_list(&self, yaks: &[Yak], format: OutputFormat, filter: YakFilter) -> String;
    fn format_yak_with_context(&self, yak: &Yak) -> String;
    fn format_empty_list(&self, format: OutputFormat) -> String;
}
