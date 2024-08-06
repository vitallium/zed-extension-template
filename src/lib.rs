use zed_extension_api::{self as zed, Command, LanguageServerId, Result, Worktree};

struct {{ project-name | upper_camel_case }}Extension {}

impl {{ project-name | upper_camel_case }}Extension {}

impl zed::Extension for {{ project-name | upper_camel_case }}Extension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &Worktree,
    ) -> Result<Command> {
        panic!()
    }
}

zed::register_extension!({{ project-name | upper_camel_case }}Extension);
