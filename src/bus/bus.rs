use std::{
  error::Error,
  fmt::Error as OtherError,
  sync::Mutex,
  time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Default)]
pub struct Bus {
  topics: Mutex<Vec<Topic>>,
}

#[derive(Debug, Default)]
struct Topic {
  name: String,
  messages: Mutex<Vec<Message>>,
}

#[derive(Debug, Default)]
struct Message {
  payload: String,
  timestamp: String,
}

impl Bus {
  pub fn new() -> Bus {
    Bus {
      topics: Mutex::new(Vec::new()),
    }
  }

  pub fn add_topic(&mut self, name: String) -> Result<String, Box<dyn Error>> {
    let topics: &mut Vec<Topic> = &mut self.topics.lock().unwrap();

    topics.push(Topic {
      name: name.clone().to_string(),
      messages: Mutex::new(Vec::new()),
    });

    Ok(name)
  }

  pub fn add_message(&mut self, topic: String, payload: String) -> Result<String, Box<dyn Error>> {
    let topics: &Vec<Topic> = &self.topics.lock().unwrap();

    let top: &Topic = match topics.iter().find(|t| t.name == topic) {
      Some(t) => t,
      None => return Err("topic not found".into()),
    };

    let time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    let messages: &mut Vec<Message> = &mut top.messages.lock().unwrap();

    messages.push(Message {
      payload: payload.clone().to_string(),
      timestamp: time.to_string(),
    });

    Ok(payload)
  }
}
