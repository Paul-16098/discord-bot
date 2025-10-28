use poise::serenity_prelude::{ Channel, Mentionable };

use crate::{ Context, Error };

/// Show this help menu
#[poise::command(slash_command)]
pub async fn help(
  ctx: Context<'_>,
  #[description = "Specific command to show help about"] #[autocomplete = "poise::builtins::autocomplete_command"] command: Option<String>
) -> Result<(), Error> {
  poise::builtins::help(ctx, command.as_deref(), poise::builtins::HelpConfiguration {
    extra_text_at_bottom: "by Paul-16098 [repo](https://github.com/Paul-16098/discord-bot/)",
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

  let c = ctx.channel_id();
  let mut count = count.unwrap_or(100);

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
      ctx.send(
        poise::CreateReply
          ::default()
          .embed(
            poise::serenity_prelude::CreateEmbed
              ::new()
              .author(ctx.author().into())
              .title("clear message")
              .description(
                format!(
                  "{} clear {} {msg_len} message(s) at {}",
                  ctx.author().mention(),
                  c.mention(),
                  ctx.created_at()
                )
              )
          )
      ).await?;
    }
    Err(e) => {
      ctx.say(e.to_string()).await?;
    }
  }

  Ok(())
}

/// nuke Channel
#[poise::command(slash_command, default_member_permissions = "MANAGE_CHANNELS")]
pub async fn nuke(
  ctx: Context<'_>,
  #[description = "why"] why: Option<String>,
  #[description = "skip confirm"] skip_confirm: Option<bool>
) -> Result<(), Error> {
  let skip_confirm = skip_confirm.unwrap_or(false);
  let channel_id = ctx.channel_id();
  let serenity_context = std::sync::Arc::new(ctx.serenity_context().clone());

  if !skip_confirm {
    let b = ctx.send(
      poise::CreateReply
        ::default()
        .embed(
          poise::serenity_prelude::CreateEmbed
            ::default()
            .title("Are you sure?")
            .description(
              "This will completely delete the channel with all the content inside, and create a new one with the same settings."
            )
        )
        .components(
          vec![
            poise::serenity_prelude::CreateActionRow::Buttons(
              vec![
                poise::serenity_prelude::CreateButton
                  ::new("sure")
                  .label("sure")
                  .style(poise::serenity_prelude::ButtonStyle::Danger),
                poise::serenity_prelude::CreateButton::new("cancel").label("cancel")
              ]
            )
          ]
        )
        .ephemeral(true)
    ).await?;
    while
      let Some(mut mci) = poise::serenity_prelude::ComponentInteractionCollector
        ::new(ctx.serenity_context())
        .timeout(std::time::Duration::from_secs(30))
        .filter(move |mci| (mci.data.custom_id == "sure" || mci.data.custom_id == "cancel"))
        .author_id(ctx.author().id)
        .channel_id(ctx.channel_id()).await
    {
      if mci.data.custom_id == "sure" {
        println!("nuke: sure");
        // 不刪除訊息，改為編輯互動訊息並清除按鈕，然後結束等待
        if
          let Err(e) = mci.message.edit(
            ctx.serenity_context(),
            poise::serenity_prelude::EditMessage
              ::default()
              .embed(poise::serenity_prelude::CreateEmbed::default().title("Ok"))
              .components(vec![])
          ).await
        {
          // 記錄但不回傳錯誤，避免 Unknown Message 導致整個指令失敗
          println!("Failed to edit interaction message after sure: {:?}", e);
        }
        ctx.say("This channel has been nuked.").await?;
        return handle_nuke(
          channel_id,
          serenity_context,
          ctx.author(),
          ctx.created_at(),
          why.as_ref()
        ).await;
      } else if mci.data.custom_id == "cancel" {
        println!("nuke: cancel");
        if
          let Err(e) = mci.message.edit(
            ctx.serenity_context(),
            poise::serenity_prelude::EditMessage
              ::default()
              .embed(
                poise::serenity_prelude::CreateEmbed
                  ::default()
                  .title("Cancel")
                  .description("nuke cancel.")
              )
              .components(vec![])
          ).await
        {
          println!("Failed to edit interaction message after cancel: {:?}", e);
        }
        // 使用者取消，直接返回
        return Ok(());
      }
    }

    if
      let Err(e) = b.edit(
        ctx,
        poise::CreateReply
          ::default()
          .embed(
            poise::serenity_prelude::CreateEmbed
              ::default()
              .title("timeout 30")
              .description("nuke cancel.")
          )
          .components(vec![])
      ).await
    {
      println!("Failed to edit original confirmation message on timeout: {:?}", e);
    }
    Ok(())
  } else {
    ctx.say("This channel has been nuked.").await?;
    handle_nuke(channel_id, serenity_context, ctx.author(), ctx.created_at(), why.as_ref()).await
  }
}

// Extracted common logic for creating a new channel into a helper function
// 修改 create_channel_builder 的 reason 型別為 Option<&'a str'>
fn create_channel_builder<'a>(
  guild_channel: &'a poise::serenity_prelude::GuildChannel,
  name: String,
  reason: Option<&'a str>
) -> poise::serenity_prelude::CreateChannel<'a> {
  let mut cc = poise::serenity_prelude::CreateChannel
    ::new(name)
    .available_tags(guild_channel.available_tags.clone())
    .permissions(guild_channel.permission_overwrites.clone())
    .nsfw(guild_channel.nsfw)
    .position(guild_channel.position)
    .kind(guild_channel.kind);

  // 使用傳入的字串參考（若有）
  if let Some(reason) = reason {
    cc = cc.audit_log_reason(reason);
  }

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

  cc
}

async fn handle_nuke(
  channel_id: poise::serenity_prelude::ChannelId,
  serenity_context: std::sync::Arc<poise::serenity_prelude::Context>,
  author: &poise::serenity_prelude::User,
  created_at: poise::serenity_prelude::Timestamp,
  why: Option<&String>
) -> Result<(), Error> {
  match channel_id.to_channel(&serenity_context).await? {
    Channel::Guild(guild_channel) => {
      let guild = guild_channel.guild_id.to_partial_guild(&serenity_context).await?;
      let name = guild_channel.name.clone();

      // 傳入字串切片，避免引用本地建立的 String
      let cc = create_channel_builder(
        &guild_channel,
        name,
        why.map(|s| s.as_str())
      );

      let new = guild.create_channel(&serenity_context, cc).await?;
      let mut r = format!("{} nuke {} to {}", author, guild_channel.mention(), new.mention());
      if let Some(v) = why {
        r += "because ";
        r += v;
      }

      new.send_message(
        &*serenity_context,
        poise::serenity_prelude::CreateMessage
          ::default()
          .embed(
            poise::serenity_prelude::CreateEmbed
              ::new()
              .author(author.into())
              .title("nuke channel")
              .description(r)
              .timestamp(created_at)
          )
      ).await?;

      guild_channel.delete(&serenity_context).await?;
    }
    _ => {
      let msg = "This command can only be used in guild channels.";
      channel_id.say(&*serenity_context, msg).await?;
    }
  }

  Ok(())
}
