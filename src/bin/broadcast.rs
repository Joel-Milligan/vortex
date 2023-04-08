use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use vortex::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    name: String,
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
            name: init.node_id,
            messages: Vec::new(),
            topology,
        }
    }

    fn step(&mut self, input: Message<Payload>, output: &mut std::io::StdoutLock) {
        let mut reply = input.clone().into_reply(Some(&mut self.id));

        match reply.body.payload {
            Payload::Broadcast { message } => {
                if self.messages.contains(&message) {
                    reply.body.payload = Payload::BroadcastOk;
                    reply.send(output);
                    self.id += 1;
                    return;
                }

                self.messages.push(message);

                for neighbour in self
                    .topology
                    .get(&self.name)
                    .unwrap()
                    .iter()
                    .filter(|n| n.to_string() != input.src)
                {
                    let msg = Message {
                        src: input.clone().dest,
                        dest: neighbour.to_string(),
                        body: Body {
                            id: Some(self.id),
                            in_reply_to: None,
                            payload: Payload::Broadcast { message },
                        },
                    };

                    msg.send(output);
                    self.id += 1;
                }

                reply.body.payload = Payload::BroadcastOk;
                reply.send(output);
                self.id += 1;
            }
            Payload::Read => {
                let messages = self.messages.clone();
                reply.body.payload = Payload::ReadOk { messages };
                reply.send(output);
                self.id += 1;
            }
            Payload::Topology { ref topology } => {
                self.topology = topology.clone();
                reply.body.payload = Payload::TopologyOk;
                reply.send(output);
                self.id += 1;
            }
            Payload::ReadOk { .. } | Payload::BroadcastOk | Payload::TopologyOk => {}
        }
    }
}

fn main() {
    main_loop::<_, BroadcastNode, _>(());
}
