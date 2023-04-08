use message::Message;
use node::Node;

pub mod message;
pub mod node;

fn main() {
    let stdin = std::io::stdin().lock();
    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

    let mut stdout = std::io::stdout().lock();

    let mut state = Node { id: 0 };

    for input in inputs {
        let input = input.unwrap();
        state.step(input, &mut stdout);
    }
}
