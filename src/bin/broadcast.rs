use std::{collections::HashMap, io::Write};

use serde::{Deserialize, Serialize};
use vortex::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Broadcast {
        message: usize,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: Vec<usize>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
}

struct BroadcastNode {
    id: usize,
    messages: Vec<usize>,
    topology: HashMap<String, Vec<String>>,
}

impl Node<(), Payload> for BroadcastNode {
    fn from_init(_state: (), init: Init) -> Self {
        let topology = init
            .node_ids
            .iter()
            .map(|n| (n.to_string(), vec![]))
            .collect();

        BroadcastNode {
            id: 1,
            messages: vec![],
            topology,
        }
    }

    fn step(&mut self, input: Message<Payload>, output: &mut std::io::StdoutLock) {
        match input.body.payload {
            Payload::Broadcast { message } => {
                self.messages.push(message);

                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::BroadcastOk,
                    },
                };
                serde_json::to_writer(&mut *output, &reply).unwrap();
                output.write_all(b"\n").unwrap();
                self.id += 1;
            }
            Payload::BroadcastOk => {}
            Payload::Read => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::ReadOk {
                            messages: self.messages.clone(),
                        },
                    },
                };
                serde_json::to_writer(&mut *output, &reply).unwrap();
                output.write_all(b"\n").unwrap();
                self.id += 1;
            }
            Payload::ReadOk { messages: _ } => {}
            Payload::Topology { topology } => {
                self.topology = topology;

                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::TopologyOk,
                    },
                };
                serde_json::to_writer(&mut *output, &reply).unwrap();
                output.write_all(b"\n").unwrap();
                self.id += 1;
            }
            Payload::TopologyOk => {}
        }
    }
}

fn main() {
    main_loop::<_, BroadcastNode, _>(());
}
