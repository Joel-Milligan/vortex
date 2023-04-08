use std::io::Write;

use serde::{Deserialize, Serialize};
use vortex::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Generate,
    GenerateOk {
        #[serde(rename = "id")]
        uuid: uuid::Uuid,
    },
}

struct GenerateNode {
    id: usize,
}

impl Node<(), Payload> for GenerateNode {
    fn from_init(_state: (), _init: Init) -> Self {
        GenerateNode { id: 1 }
    }

    fn step(&mut self, input: Message<Payload>, output: &mut std::io::StdoutLock) {
        match input.body.payload {
            Payload::Generate => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::GenerateOk {
                            uuid: uuid::Uuid::new_v4(),
                        },
                    },
                };
                serde_json::to_writer(&mut *output, &reply).unwrap();
                output.write_all(b"\n").unwrap();
                self.id += 1;
            }
            Payload::GenerateOk { uuid: _ } => panic!("received generate_ok"),
        }
    }
}

fn main() {
    main_loop::<_, GenerateNode, _>(());
}
