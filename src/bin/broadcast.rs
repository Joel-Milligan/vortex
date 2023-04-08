use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
        let mut reply = input.into_reply(Some(&mut self.id));

        match reply.body.payload {
            Payload::Broadcast { message } => {
                self.messages.push(message);
                reply.send(output, Payload::BroadcastOk);
                self.id += 1;
            }
            Payload::Read => {
                reply.send(
                    output,
                    Payload::ReadOk {
                        messages: self.messages.clone(),
                    },
                );
                self.id += 1;
            }
            Payload::Topology { ref topology } => {
                self.topology = topology.clone();
                reply.send(output, Payload::TopologyOk);
                self.id += 1;
            }
            Payload::ReadOk { .. } | Payload::BroadcastOk | Payload::TopologyOk => {}
        }
    }
}

fn main() {
    main_loop::<_, BroadcastNode, _>(());
}
