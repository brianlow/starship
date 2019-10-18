use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GitStatusConfig<'a> {
    pub ahead: SegmentConfig<'a>,
    pub behind: SegmentConfig<'a>,
    pub diverged: SegmentConfig<'a>,
    pub show_sync_count: bool,
    pub conflicted: SegmentConfig<'a>,
    pub conflicted_count_disabled: bool,
    pub deleted: SegmentConfig<'a>,
    pub deleted_count_disabled: bool,
    pub renamed: SegmentConfig<'a>,
    pub renamed_count_disabled: bool,
    pub modified: SegmentConfig<'a>,
    pub modified_count_disabled: bool,
    pub staged: SegmentConfig<'a>,
    pub staged_count_disabled: bool,
    pub untracked: SegmentConfig<'a>,
    pub untracked_count_disabled: bool,
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for GitStatusConfig<'a> {
    fn new() -> Self {
        GitStatusConfig {
            ahead: SegmentConfig::new("⇡"),
            behind: SegmentConfig::new("⇣"),
            diverged: SegmentConfig::new("⇕"),
            conflicted: SegmentConfig::new("="),
            show_sync_count: false,
            conflicted_count_disabled: true,
            deleted: SegmentConfig::new("✘"),
            deleted_count_disabled: true,
            renamed: SegmentConfig::new("»"),
            renamed_count_disabled: true,
            modified: SegmentConfig::new("!"),
            modified_count_disabled: true,
            staged: SegmentConfig::new("+"),
            staged_count_disabled: true,
            untracked: SegmentConfig::new("?"),
            untracked_count_disabled: true,
            prefix: "[",
            suffix: "] ",
            style: Color::Red.bold(),
            disabled: false,
        }
    }
}
