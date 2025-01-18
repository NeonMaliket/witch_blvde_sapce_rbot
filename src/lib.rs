use teloxide::prelude::*;
use teloxide::types::MessageKind;

pub mod message_handler;

pub struct BotMetadata<'m, 'b> {
    bot: &'b Bot,
    message: &'m Message,
}

impl<'m, 'b> BotMetadata<'m, 'b> {
    pub fn new(bot: &'b Bot, message: &'m Message) -> Self {
        Self { bot, message }
    }

    pub(crate) fn chat_id(&self) -> &'m ChatId {
        &self.message.chat.id
    }

    pub(crate) fn message_kind(&self) -> &'m MessageKind {
        &self.message.kind
    }
}