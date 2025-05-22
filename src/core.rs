use quick_xml::Reader;
use quick_xml::events::Event;
use serde_json::{Map, Value};
use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq)]
pub struct Channel {
    content: Value,
}

#[derive(Debug, PartialEq)]
pub struct Item<'a> {
    content: &'a Value,
}
impl<'a> Item<'a> {
    pub fn new(content: &'a Value) -> Self {
        Item { content }
    }
    pub fn title(&self) -> Option<&str> {
        self.content["title"].as_str()
    }
    pub fn link(&self) -> Option<&str> {
        self.content["link"].as_str()
    }
    pub fn pub_date(&self) -> Option<&str> {
        self.content["pubDate"].as_str()
    }
    pub fn torrent(&self) -> Option<&Map<String, Value>> {
        self.content["torrent"].as_object()
    }
    pub fn description(&self) -> Option<&str> {
        self.content["description"].as_str()
    }
}
impl Channel {
    pub fn new(mut content: Value) -> Self {
        Channel {
            content: content["rss"]["channel"].take(),
        }
    }
    pub fn link(&self) -> Option<&str> {
        self.content["link"].as_str()
    }
    pub fn title(&self) -> Option<&str> {
        self.content["title"].as_str()
    }
    pub fn items(&self) -> Option<Vec<Item>> {
        Some(
            self.content["item"]
                .as_array()?
                .iter()
                .map(|i| Item::new(i))
                .collect::<Vec<_>>(),
        )
    }
    pub fn item_iter(&self) -> Option<impl Iterator<Item = Item<'_>>> {
        Some(
            self.content["item"]
                .as_array()?
                .iter()
                .map(|i| Item::new(i)),
        )
    }
    pub fn description(&self) -> Option<&str> {
        self.content["description"].as_str()
    }
}
impl Deref for Channel {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl DerefMut for Channel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}
impl<'a> Deref for Item<'a> {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}
pub fn from_str(xml: &str) -> Option<Channel> {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    let mut stack: Vec<(String, Value)> = Vec::new();
    stack.push(("root".to_string(), Value::Object(Map::new())));

    while let Ok(event) = reader.read_event_into(&mut buf) {
        match event {
            Event::Start(e) => {
                let tag_name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                stack.push((tag_name.clone(), Value::Object(Map::new())));
            }
            Event::Text(e) => {
                let text = e.unescape().ok()?.into_owned();
                if let Some((_, last_value)) = stack.last_mut() {
                    *last_value = Value::String(text);
                }
            }
            Event::End(e) => {
                if stack.len() < 2 {
                    continue;
                }
                let tag_name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                let value = stack.pop()?.1;
                let parent = &mut stack.last_mut()?.1;
                let map = parent.as_object_mut()?;
                if tag_name == "item" {
                    map.entry("item".to_string())
                        .or_insert_with(|| Value::Array(Vec::new()))
                        .as_array_mut()?
                        .push(value);
                } else {
                    map.insert(tag_name, value);
                }
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }
    let (_, result) = stack.into_iter().next()?;
    if result.as_object()?.is_empty() {
        return None;
    }
    Some(Channel::new(result))
}
