use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message<Payload> {
    pub src: String,
    pub dest: String,
    pub body: Body<Payload>,
}

impl<Payload: Serialize> Message<Payload> {
    pub fn into_reply(self, id: Option<&mut usize>) -> Self {
        Self {
            src: self.dest,
            dest: self.src,
            body: Body {
                id: id.map(|id| {
                    let mid = *id;
                    *id += 1;
                    mid
                }),
                in_reply_to: self.body.id,
                payload: self.body.payload,
            },
        }
    }

    pub fn send(&mut self, output: &mut std::io::StdoutLock, payload: Payload) {
        self.body.payload = payload;
        serde_json::to_writer(&mut *output, &self).unwrap();
        output.write_all(b"\n").unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body<Payload> {
    #[serde(rename = "msg_id")]
    pub id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum InitPayload {
    Init(Init),
    InitOk,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Init {
    pub node_id: String,
    pub node_ids: Vec<String>,
}

pub trait Node<S, Payload> {
    fn from_init(state: S, init: Init) -> Self;
    fn step(&mut self, input: Message<Payload>, output: &mut std::io::StdoutLock);
}

pub fn main_loop<S, N, P>(init_state: S)
where
    P: DeserializeOwned,
    N: Node<S, P>,
{
    let mut stdin = std::io::stdin().lock().lines();
    let mut stdout = std::io::stdout().lock();

    let init_msg: Message<InitPayload> =
        serde_json::from_str(&stdin.next().expect("No init message received").unwrap()).unwrap();

    let InitPayload::Init(init) = init_msg.body.payload else {
        panic!("first message should be init");
    };

    let mut node: N = Node::from_init(init_state, init);

    let reply = Message {
        src: init_msg.dest,
        dest: init_msg.src,
        body: Body {
            id: Some(0),
            in_reply_to: init_msg.body.id,
            payload: InitPayload::InitOk,
        },
    };

    serde_json::to_writer(&mut stdout, &reply).unwrap();
    stdout.write_all(b"\n").unwrap();

    for line in stdin {
        let line = line.unwrap();
        let input: Message<P> =
            serde_json::from_str(&line).expect("Could not deserialise input from STDIN");
        node.step(input, &mut stdout);
    }
}
