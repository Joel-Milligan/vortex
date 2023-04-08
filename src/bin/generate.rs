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
        let mut reply = input.into_reply(Some(&mut self.id));

        match reply.body.payload {
            Payload::Generate => {
                let uuid = uuid::Uuid::new_v4();
                reply.body.payload = Payload::GenerateOk { uuid };
                reply.send(output);
                self.id += 1;
            }
            Payload::GenerateOk { .. } => {}
        }
    }
}

fn main() {
    main_loop::<_, GenerateNode, _>(());
}
