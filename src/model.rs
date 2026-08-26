// use crate::message::Message;

// #[derive(Debug, Default)]
// pub struct CounterModel {
//     pub counter: i64,
//     running_state: RunningState,
// }

// impl CounterModel {
//     pub fn state(&self) -> RunningState {
//         self.running_state
//     }
// }

// #[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
// #[repr(u8)]
// pub enum RunningState {
//     #[default]
//     Running,
//     Done,
// }

// pub fn update(model: &mut CounterModel, msg: Message) -> Option<Message> {
//     match msg {
//         Message::Increment => {
//             model.counter += 1;
//             if model.counter > 50 {
//                 return Some(Message::Reset);
//             }
//         }
//         Message::Decrement => {
//             model.counter -= 1;
//             if model.counter < -50 {
//                 return Some(Message::Reset);
//             }
//         }
//         Message::Reset => model.counter = 0,
//         Message::Quit => {
//             // You can handle cleanup and exit here
//             model.running_state = RunningState::Done;
//         }
//     };
//     None
// }
