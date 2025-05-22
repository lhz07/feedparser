#[test]
fn test_normal() {
    let xml = r#"<?xml version="1.0" encoding="utf-8"?><rss version="2.0"><channel><title>Mikan Project - 莉可丽丝：友谊是时间的窃贼</title><link>http://mikanime.tv/RSS/Bangumi?bangumiId=3644&amp;subgroupid=1204</link><description>Mikan Project - 莉可丽丝：友谊是时间的窃贼</description><item><guid isPermaLink="false">[AnimeRep]莉可丽丝：友谊是时间的窃贼 / Lycoris Recoil - Friends are thieves of time. [02][1080p][简中内嵌]</guid><link>https://mikanime.tv/Home/Episode/fe6522db1f28a80944e50af3da3ae164a9fbebcf</link><title>[AnimeRep]莉可丽丝：友谊是时间的窃贼 / Lycoris Recoil - Friends are thieves of time. [02][1080p][简中内嵌]</title><description>[AnimeRep]莉可丽丝：友谊是时间的窃贼 / Lycoris Recoil - Friends are thieves of time. [02][1080p][简中内嵌][32.5 MB]</description><torrent xmlns="https://mikanime.tv/0.1/"><link>https://mikanime.tv/Home/Episode/fe6522db1f28a80944e50af3da3ae164a9fbebcf</link><contentLength>34078720</contentLength><pubDate>2025-04-23T20:22:48.014686</pubDate></torrent><enclosure type="application/x-bittorrent" length="34078720" url="https://mikanime.tv/Download/20250423/fe6522db1f28a80944e50af3da3ae164a9fbebcf.torrent" /></item><item><guid isPermaLink="false">[AnimeRep]莉可丽丝 朋友是时间小偷。/Lycoris Recoil - Friends are thieves of time. [简中][H264]</guid><link>https://mikanime.tv/Home/Episode/70d42351deed217c56c734e1fbb6e88064290b2f</link><title>[AnimeRep]莉可丽丝 朋友是时间小偷。/Lycoris Recoil - Friends are thieves of time. [简中][H264]</title><description>[AnimeRep]莉可丽丝 朋友是时间小偷。/Lycoris Recoil - Friends are thieves of time. [简中][H264][21.9 MB]</description><torrent xmlns="https://mikanime.tv/0.1/"><link>https://mikanime.tv/Home/Episode/70d42351deed217c56c734e1fbb6e88064290b2f</link><contentLength>22963814</contentLength><pubDate>2025-04-23T04:13:05.923325</pubDate></torrent><enclosure type="application/x-bittorrent" length="22963814" url="https://mikanime.tv/Download/20250423/70d42351deed217c56c734e1fbb6e88064290b2f.torrent" /></item></channel></rss>"#;
    let channel = crate::from_str(xml).unwrap();
    assert_eq!(
        Some("http://mikanime.tv/RSS/Bangumi?bangumiId=3644&subgroupid=1204"),
        channel.link()
    );
    assert_eq!(
        Some("Mikan Project - 莉可丽丝：友谊是时间的窃贼"),
        channel.description()
    );
    assert_eq!(
        Some("Mikan Project - 莉可丽丝：友谊是时间的窃贼"),
        channel.title()
    );
    let items = channel.items().unwrap();
    let item_iter = channel.item_iter().unwrap();
    assert_eq!(items, item_iter.collect::<Vec<_>>());
    assert_eq!(2, items.len());
    assert_eq!(
        Some("2025-04-23T20:22:48.014686"),
        items[0]["torrent"]["pubDate"].as_str()
    );
    assert_eq!(
        Some("https://mikanime.tv/Home/Episode/70d42351deed217c56c734e1fbb6e88064290b2f"),
        items[1].link()
    );
    assert_eq!(
        Some(
            "[AnimeRep]莉可丽丝：友谊是时间的窃贼 / Lycoris Recoil - Friends are thieves of time. [02][1080p][简中内嵌]"
        ),
        items[0].title()
    );
    assert_eq!(
        Some(
            "[AnimeRep]莉可丽丝：友谊是时间的窃贼 / Lycoris Recoil - Friends are thieves of time. [02][1080p][简中内嵌][32.5 MB]"
        ),
        items[0].description()
    );
    assert_eq!(
        "https://mikanime.tv/Home/Episode/fe6522db1f28a80944e50af3da3ae164a9fbebcf",
        items[0].torrent().unwrap()["link"]
    );
    assert_eq!("34078720", items[0].torrent().unwrap()["contentLength"]);
}

#[test]
fn test_invalid() {
    let xml = r#"<?xml version="1.0" encoding="utf-8"?><rss version="2.0"><channel><title><link>http://mikanime.tv/RSS/Bangumi?bangumiId=3644&amp;subgroupid=1204</link><description>Mikan Project - 莉可丽丝：友谊是时间的窃贼</description><item><guid isPermaLink="false">[AnimeRep]莉可丽丝：友谊是时间的窃贼 / Lycoris Recoil - Friends are thieves of time. [02][1080p][简中内嵌]</guid><link>https://mikanime.tv/Home/Episode/fe6522db1f28a80944e50af3da3ae164a9fbebcf</link><title>[AnimeRep]莉可丽丝：友谊是时间的窃贼 / Lycoris Recoil - Friends are thieves of time. [02][1080p][简中内嵌]</title><description>[AnimeRep]莉可丽丝：友谊是时间的窃贼 / Lycoris Recoil - Friends are thieves of time. [02][1080p][简中内嵌][32.5 MB]</description><torrent xmlns="https://mikanime.tv/0.1/"><link>https://mikanime.tv/Home/Episode/fe6522db1f28a80944e50af3da3ae164a9fbebcf</link><contentLength>34078720</contentLength><pubDate>2025-04-23T20:22:48.014686</pubDate></torrent><enclosure type="application/x-bittorrent" length="34078720" url="https://mikanime.tv/Download/20250423/fe6522db1f28a80944e50af3da3ae164a9fbebcf.torrent" /></item><item><guid isPermaLink="false">[AnimeRep]莉可丽丝 朋友是时间小偷。/Lycoris Recoil - Friends are thieves of time. [简中][H264]</guid><link>https://mikanime.tv/Home/Episode/70d42351deed217c56c734e1fbb6e88064290b2f</link><title>[AnimeRep]莉可丽丝 朋友是时间小偷。/Lycoris Recoil - Friends are thieves of time. [简中][H264]</title><description>[AnimeRep]莉可丽丝 朋友是时间小偷。/Lycoris Recoil - Friends are thieves of time. [简中][H264][21.9 MB]</description><torrent xmlns="https://mikanime.tv/0.1/"><link>https://mikanime.tv/Home/Episode/70d42351deed217c56c734e1fbb6e88064290b2f</link><contentLength>22963814</contentLength><pubDate>2025-04-23T04:13:05.923325</pubDate></torrent><enclosure type="application/x-bittorrent" length="22963814" url="https://mikanime.tv/Download/20250423/70d42351deed217c56c734e1fbb6e88064290b2f.torrent" /></item></channel></rss>"#;
    let channel = crate::from_str(xml);
    assert_eq!(channel, None);
    let xml = r#"<?xml version="1.0" encoding="utf-8"?><rss version="2.0"><channel><link>http://mikanime.tv/RSS/Bangumi?bangumiId=3644&amp;subgroupid=1204</link><descriMikan Project - 莉可丽丝：友谊是时间的窃贼</description><item><guid isPermaLink="false">[AnimeRep]莉可丽丝：友谊是时间的窃贼 / Lycoris Recoil - Friends are thieves of time. [02][1080p][简中内嵌]</guid><link>https://mikanime.tv/Home/Episode/fe6522db1f28a80944e50af3da3ae164a9fbebcf</link><title>[AnimeRep]莉可丽丝：友谊是时间的窃贼 / Lycoris Recoil - Friends are thieves of time. [02][1080p][简中内嵌]</title><description>[AnimeRep]莉可丽丝：友谊是时间的窃贼 / Lycoris Recoil - Friends are thieves of time. [02][1080p][简中内嵌][32.5 MB]</description><torrent xmlns="https://mikanime.tv/0.1/"><link>https://mikanime.tv/Home/Episode/fe6522db1f28a80944e50af3da3ae164a9fbebcf</link><contentLength>34078720</contentLength><pubDate>2025-04-23T20:22:48.014686</pubDate></torrent><enclosure type="application/x-bittorrent" length="34078720" url="https://mikanime.tv/Download/20250423/fe6522db1f28a80944e50af3da3ae164a9fbebcf.torrent" /></item><item><guid isPermaLink="false">[AnimeRep]莉可丽丝 朋友是时间小偷。/Lycoris Recoil - Friends are thieves of time. [简中][H264]</guid><link>https://mikanime.tv/Home/Episode/70d42351deed217c56c734e1fbb6e88064290b2f</link><title>[AnimeRep]莉可丽丝 朋友是时间小偷。/Lycoris Recoil - Friends are thieves of time. [简中][H264]</title><description>[AnimeRep]莉可丽丝 朋友是时间小偷。/Lycoris Recoil - Friends are thieves of time. [简中][H264][21.9 MB]</description><torrent xmlns="https://mikanime.tv/0.1/"><link>https://mikanime.tv/Home/Episode/70d42351deed217c56c734e1fbb6e88064290b2f</link><contentLength>22963814</contentLength><pubDate>2025-04-23T04:13:05.923325</pubDate></torrent><enclosure type="application/x-bittorrent" length="22963814" url="https://mikanime.tv/Download/20250423/70d42351deed217c56c734e1fbb6e88064290b2f.torrent" /></item></channel></rss>"#;
    let channel = crate::from_str(xml);
    assert_eq!(channel, None);
}
