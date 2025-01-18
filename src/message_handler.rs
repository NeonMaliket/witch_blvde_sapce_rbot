use crate::BotMetadata;
use teloxide::prelude::*;
use teloxide::types::*;

pub async fn attach_message_handlers(metadata: &BotMetadata<'_, '_>) {
    message_kind_handlers(metadata).await;
}

async fn message_kind_handlers(metadata: &BotMetadata<'_, '_>) {
    let message_kind = metadata.message_kind();
    match message_kind {
        MessageKind::Common(message) => {
            media_kind_handlers(&message.media_kind, metadata).await;
        }
        MessageKind::NewChatMembers(_) => {}
        MessageKind::LeftChatMember(_) => {}
        MessageKind::NewChatTitle(_) => {}
        MessageKind::NewChatPhoto(_) => {}
        MessageKind::DeleteChatPhoto(_) => {}
        MessageKind::GroupChatCreated(_) => {}
        MessageKind::SupergroupChatCreated(_) => {}
        MessageKind::ChannelChatCreated(_) => {}
        MessageKind::MessageAutoDeleteTimerChanged(_) => {}
        MessageKind::Pinned(_) => {}
        MessageKind::ChatShared(_) => {}
        MessageKind::UsersShared(_) => {}
        MessageKind::Invoice(_) => {}
        MessageKind::SuccessfulPayment(_) => {}
        MessageKind::ConnectedWebsite(_) => {}
        MessageKind::WriteAccessAllowed(_) => {}
        MessageKind::PassportData(_) => {}
        MessageKind::Dice(_) => {}
        MessageKind::ProximityAlertTriggered(_) => {}
        MessageKind::ForumTopicCreated(_) => {}
        MessageKind::ForumTopicEdited(_) => {}
        MessageKind::ForumTopicClosed(_) => {}
        MessageKind::ForumTopicReopened(_) => {}
        MessageKind::GeneralForumTopicHidden(_) => {}
        MessageKind::GeneralForumTopicUnhidden(_) => {}
        MessageKind::Giveaway(_) => {}
        MessageKind::GiveawayCompleted(_) => {}
        MessageKind::GiveawayCreated(_) => {}
        MessageKind::GiveawayWinners(_) => {}
        MessageKind::VideoChatScheduled(_) => {}
        MessageKind::VideoChatStarted(_) => {}
        MessageKind::VideoChatEnded(_) => {}
        MessageKind::VideoChatParticipantsInvited(_) => {}
        MessageKind::WebAppData(_) => {}
        MessageKind::Empty { .. } => {}
    }
}

async fn media_kind_handlers(media_kind: &MediaKind, metadata: &BotMetadata<'_, '_>) {
    let bot = metadata.bot;
    let chat_id = metadata.chat_id();
    match media_kind {
        MediaKind::Animation(_) => {}
        MediaKind::Audio(_) => {}
        MediaKind::Contact(_) => {}
        MediaKind::Document(_) => {}
        MediaKind::Game(_) => {}
        MediaKind::Venue(_) => {}
        MediaKind::Location(_) => {}
        MediaKind::Photo(photo) => handle_incoming_photo(bot, chat_id, photo).await
        MediaKind::Poll(_) => {}
        MediaKind::Sticker(_) => {}
        MediaKind::Story(_) => {}
        MediaKind::Text(_) => {}
        MediaKind::Video(_) => {}
        MediaKind::VideoNote(_) => {}
        MediaKind::Voice(_) => {}
        MediaKind::Migration(_) => {}
    }
}

async fn handle_incoming_photo(bot: &Bot, chat_id: &ChatId, photo: &MediaPhoto) {
    let id = photo
        .photo
        .iter()
        .map(|file| &file.file.id)
        .last()
        .expect("No photo");
    bot.send_photo(*chat_id, InputFile::file_id(id))
        .await
        .expect("Sending photo exception");
}
