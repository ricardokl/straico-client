use ::serde::Deserialize;

// Add model in the struct. This way, we can use the model to pick the right template
#[derive(Deserialize)]
pub struct Chat(Vec<Message>);

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Message {
    role: Role,
    content: String,
}

#[derive(Deserialize)]
pub enum Role {
    User,
    Assistant,
}

// Convert a Chat object into a prompt, using jinja2 templating
// Needs to know the target model to pick the right jinja template
// Might not be able to use 'form', since it needs the target model
// Idea: Have a prompt type with a model field that is skipped when serializing,
// this way, the correct template can be picked via the output
impl<'a> From<Chat> for &'a str {
    fn from(_value: Chat) -> Self {
        todo!()
    }
}

impl Chat {
    pub fn new() -> Chat {
        Chat(Vec::new())
    }

    // Add a message to the chat
    pub fn add_message(&mut self, role: Role, content: &str) {
        self.0.push(Message {
            role,
            content: content.to_string(),
        });
    }
}
