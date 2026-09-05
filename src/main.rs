use nostr_sdk::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // throwaway key — we'll swap this for a saved one later
    let keys = Keys::generate();
    println!("kizami npub: {}", keys.public_key().to_bech32()?);

    let client = Client::new();
    client.add_relay("wss://hasky.chat").await?;
    client.connect().await;

    // fake timecode until real hardware shows up
    let timecode = "11:12:00:16";
    let fps = "25";

    let event = EventBuilder::new(
        Kind::TextNote,
        format!("刻み kizami test — tc {timecode} @ {fps} fps"),
    )
    .tag(Tag::parse(["timecode", timecode])?)
    .tag(Tag::parse(["fps", fps])?)
    .tag(Tag::hashtag("kizami"))
    .finalize(&keys)?;

    let out = client.send_event(&event).await?;
    println!("published: {}", out.id().to_bech32()?);

    Ok(())
}


