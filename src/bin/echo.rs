use serde::{Deserialize, Serialize};
use vortex::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Echo { echo: String },
    EchoOk { echo: String },
}

struct EchoNode {
    id: usize,
}

impl Node<(), Payload> for EchoNode {
    fn from_init(_state: (), _init: Init) -> Self {
        EchoNode { id: 1 }
    }

    fn step(&mut self, input: Message<Payload>, output: &mut std::io::StdoutLock) {
        let mut reply = input.into_reply(Some(&mut self.id));
        match reply.body.payload {
            Payload::Echo { ref echo } => {
                let echo = echo.to_string();
                reply.body.payload = Payload::EchoOk { echo };
                reply.send(output);
                self.id += 1;
            }
            Payload::EchoOk { .. } => {}
        }
    }
}

fn main() {
    main_loop::<_, EchoNode, _>(());
}
