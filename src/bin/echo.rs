use std::io::Write;

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
        match input.body.payload {
            Payload::Echo { echo } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::EchoOk { echo },
                    },
                };
                serde_json::to_writer(&mut *output, &reply).unwrap();
                output.write_all(b"\n").unwrap();
                self.id += 1;
            }
            Payload::EchoOk { echo: _ } => {}
        }
    }
}

fn main() {
    main_loop::<_, EchoNode, _>(());
}
