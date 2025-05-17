use opf_models::event::{send_event, Event};
use opf_models::{self, error::ErrorKind, suggestion, Command, CommandAction, Suggestion};

use crate::store::DB;

impl DB {
    pub async fn on_suggestion_command(&mut self, command: Command) -> Result<(), ErrorKind> {
        match command.action {
            CommandAction::List => self.list_suggestions().await,
            CommandAction::Accept => self.approve_suggestion(command).await,
            CommandAction::Del => self.deny_suggestion(command).await,
            _ => Err(ErrorKind::ActionNotAvailable),
        }
    }

    async fn list_suggestions(&self) -> Result<(), ErrorKind> {
        let suggestions = self.suggestions.read().await;
        let mut headers = vec![
            suggestion::ID.to_string(),
            suggestion::DESCRIPTION.to_string(),
            suggestion::COMMAND.to_string(),
        ];
        let mut rows = vec![];
        for (_, sugg) in suggestions.iter() {
            rows.push(vec![
                sugg.suggestion_id.to_string(),
                sugg.description.clone(),
                sugg.command.clone(),
            ]);
        }
        send_event(&self.db_tx, Event::ResponseTable((headers, rows))).await
    }

    async fn approve_suggestion(&mut self, command: Command) -> Result<(), ErrorKind> {
        let suggestion_id = command
            .params
            .get(suggestion::ID)
            .ok_or(ErrorKind::InvalidFormatArgument)?
            .parse::<i32>()
            .map_err(|_| ErrorKind::InvalidFormatArgument)?;
        let mut suggestions = self.suggestions.write().await;
        let sugg = suggestions
            .remove(&suggestion_id)
            .ok_or(ErrorKind::GenericError(format!("suggestion {} not found", suggestion_id)))?;
        send_event(&self.db_tx, Event::ResponseSimple(format!("suggestion {} executed", suggestion_id))).await?;
        send_event(&self.db_tx, Event::NewCommand(sugg.command)).await
    }

    async fn deny_suggestion(&mut self, command: Command) -> Result<(), ErrorKind> {
        let suggestion_id = command
            .params
            .get(suggestion::ID)
            .ok_or(ErrorKind::InvalidFormatArgument)?
            .parse::<i32>()
            .map_err(|_| ErrorKind::InvalidFormatArgument)?;
        let mut suggestions = self.suggestions.write().await;
        suggestions.remove(&suggestion_id);
        send_event(&self.db_tx, Event::ResponseSimple(format!("suggestion {} denied", suggestion_id))).await
    }
}
