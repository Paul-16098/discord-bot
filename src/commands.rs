use poise::serenity_prelude::{ Mentionable, Channel };

use crate::{ Context, Error };

/// Show this help menu
#[poise::command(slash_command)]
pub async fn help(
  ctx: Context<'_>,
  #[description = "Specific command to show help about"] #[autocomplete = "poise::builtins::autocomplete_command"] command: Option<String>
) -> Result<(), Error> {
  poise::builtins::help(ctx, command.as_deref(), poise::builtins::HelpConfiguration {
    extra_text_at_bottom: "This is an example bot made to showcase features of my custom Discord bot framework",
    show_context_menu_commands: true,
    show_subcommands: true,
    ..Default::default()
  }).await?;
  Ok(())
}
/// ping bot
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
  let ping_ms = ctx.ping().await.as_millis();
  ctx.say(format!("ping: {}ms", ping_ms)).await?;
  Ok(())
}

/// clear msg
#[poise::command(slash_command, default_member_permissions = "MANAGE_MESSAGES")]
pub async fn clear(
  ctx: Context<'_>,
  #[description = "How many msg to clear"] #[min = 1] count: Option<u8>
) -> Result<(), Error> {
  use poise::serenity_prelude::GetMessages;

  let mut count = count.unwrap_or(100);

  let c = ctx.channel_id();

  let mut msg = c.messages(ctx.serenity_context(), GetMessages::new().limit(count)).await?;
  if count > 100 {
    count -= 100;
    msg.append(
      c
        .messages(ctx.serenity_context(), GetMessages::new().limit(count).before(msg[0].id)).await?
        .as_mut()
    );
  }
  let msg_len = msg.len(); // 先讀取長度，避免下面的 delete_messages 移動 msg 後再使用它
  match c.delete_messages(ctx.serenity_context(), msg).await {
    Ok(_) => {
      // 建立要發送的字串（owned），在 await 前完成
      let reply = format!(
        "{} clear {} {msg_len} message(s) at {}",
        ctx.author().mention(),
        c.mention(),
        ctx.created_at()
      );
      ctx.say("ok").await?;
      c.say(ctx.serenity_context(), reply).await?;
    }
    Err(e) => {
      ctx.say(e.to_string()).await?;
    }
  }

  Ok(())
}
/// nuke Channel
/// 
/// but the
#[poise::command(slash_command, default_member_permissions = "MANAGE_CHANNELS")]
pub async fn nuke(
  ctx: Context<'_>,
  #[description = "why"] why: Option<String>
) -> Result<(), Error> {
  let channel_id = ctx.channel_id();
  let serenity_context = std::sync::Arc::new(ctx.serenity_context().clone());

  // 新增：在呼叫 handle_nuke 前擷取並傳入必要的字串資料
  let author_mention = ctx.author().mention().to_string();
  let created_at = ctx.created_at().to_string();

  ctx.say("This channel has been nuked.").await?;
  handle_nuke(channel_id, serenity_context, author_mention, created_at, why.as_ref()).await
}

async fn handle_nuke(
  channel_id: poise::serenity_prelude::ChannelId,
  serenity_context: std::sync::Arc<poise::serenity_prelude::Context>,
  author_mention: String,
  created_at: String,
  why: Option<&String>
) -> Result<(), Error> {
  match channel_id.to_channel(&serenity_context).await? {
    Channel::Guild(guild_channel) => {
      let guild = guild_channel.guild_id.to_partial_guild(&serenity_context).await?;
      // clone 會在使用前複製必要欄位，避免移動 guild_channel 的內部資料
      let name = guild_channel.name.clone();
      let available_tags = guild_channel.available_tags.clone();
      let permissions = guild_channel.permission_overwrites.clone();
      let nsfw = guild_channel.nsfw;
      let position = guild_channel.position;
      let kind = guild_channel.kind;

      let mut cc = poise::serenity_prelude::CreateChannel
        ::new(name)
        .available_tags(available_tags)
        .permissions(permissions)
        .nsfw(nsfw)
        .position(position)
        .kind(kind);
      // let r = "Nuked Channel".to_string();
      let mut r = "Nuked Channel".to_string();
      if let Some(why) = why {
        // 先加上分隔字串，再附加原因（以 &String 傳入 push_str）
        r.push_str(": ");
        r.push_str(why);
      }
      cc = cc.audit_log_reason(&r);

      // 對於可能是 Option<T> 且非 Copy 的欄位，從參考 clone 值再傳入 builder
      if let Some(topic) = &guild_channel.topic {
        cc = cc.topic(topic.clone());
      }
      if let Some(v) = &guild_channel.default_auto_archive_duration {
        cc = cc.default_auto_archive_duration(*v);
      }
      if let Some(v) = &guild_channel.default_reaction_emoji {
        cc = cc.default_reaction_emoji(v.clone());
      }
      if let Some(v) = &guild_channel.default_sort_order {
        cc = cc.default_sort_order(*v);
      }
      if let Some(v) = &guild_channel.rate_limit_per_user {
        cc = cc.rate_limit_per_user(*v);
      }
      if let Some(v) = &guild_channel.rtc_region {
        cc = cc.rtc_region(v.clone());
      }
      if let Some(v) = &guild_channel.user_limit {
        cc = cc.user_limit(*v);
      }
      if let Some(v) = &guild_channel.video_quality_mode {
        cc = cc.video_quality_mode(*v);
      }
      if let Some(v) = &guild_channel.bitrate {
        cc = cc.bitrate(*v);
      }

      let new = guild.create_channel(&serenity_context, cc).await?;
      // 使用傳入的 author_mention 與 created_at，並補上 await
      let mut r = format!(
        "{} nuke {} to {} at {}",
        author_mention,
        guild_channel.mention(),
        new.mention(),
        created_at
      );
      if let Some(why) = why {
        r.push_str(" because: ");
        r.push_str(why);
      }
      new.say(&*serenity_context, r).await?;
      // 現在可以安全地刪除原頻道並重新建立
      guild_channel.delete(&serenity_context).await?;
    }
    _ => {
      let msg = "This command can only be used in guild channels.";
      channel_id.say(&*serenity_context, msg).await?;
    }
  }

  Ok(())
}
