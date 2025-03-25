use serde_json::{Value, Map};
use std::ops::{Deref, DerefMut};
use quick_xml::Reader;
use quick_xml::events::Event;
pub struct Channel {
    content: Value,
}
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
    pub fn items(&self) -> Option<Vec<Item>> {
        Some(self.content["item"]
            .as_array()?
            .iter()
            .map(|i| Item::new(i))
            .collect::<Vec<_>>())
    }
    pub fn item_iter(&self) -> Option<impl Iterator<Item = Item<'_ >>>{
        Some(self.content["item"]
            .as_array()?
            .iter()
            .map(|i| Item::new(i)))
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
    // println!("{xml}");
    let mut buf = Vec::new();
    let mut stack: Vec<(String, Value)> = Vec::new();
    stack.push(("root".to_string(), Value::Object(Map::new())));
    // let mut value;

    while let Ok(event) = reader.read_event_into(&mut buf) {
        match event {
            Event::Start(e) => {
                let tag_name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                // println!("create: {:?}", tag_name);
                stack.push((tag_name.clone(), Value::Object(Map::new())));
            }
            Event::Text(e) => {
                let text = e.unescape().unwrap().into_owned();
                // println!("content: {:?}", text);
                if let Some((_, last_value)) = stack.last_mut() {
                    // println!("push content: {:?}", text);
                    *last_value = Value::String(text);
                }
            }
            Event::End(e) => {
                let tag_name = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                // println!("end: {:?}", tag_name);
                // println!("{:?}", tag_name);
                // println!("{:?}", stack.last());
                if stack.len() < 2 {
                    continue;
                }
                let value = stack.pop()?.1;
                let parent = &mut stack.last_mut()?.1;
                let map = parent.as_object_mut()?;
                if tag_name == "item" {
                    map.entry("item".to_string())
                        .or_insert_with(|| Value::Array(Vec::new()))
                        .as_array_mut()?
                        .push(value);
                } else {
                    // println!("parent: {:?}", parent);
                    // println!("insert {} and {} into map: {:?}", tag_name, value, map);
                    map.insert(tag_name, value);
                }
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }
    let (_, result) = stack.into_iter().next()?;
    Some(Channel::new(result))
}