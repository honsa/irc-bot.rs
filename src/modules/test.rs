use core::*;
use irc::client::prelude::*;
use std::rc::Rc;

pub fn mk() -> Module {
    Module::new("test".into(),
                vec![ModuleFeature::Command {
                         name: "test-line-wrap".into(),
                         usage: "".into(),
                         handler: Rc::new(test_line_wrap),
                     }])
}

const LOREM_IPSUM_TEXT: &'static str =
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer et tincidunt nibh. Nullam \
     aliquet imperdiet cursus. Duis at turpis mollis, iaculis quam sed, efficitur arcu. Sed vel \
     massa sit amet magna efficitur hendrerit. Donec auctor auctor ligula nec semper. Nulla a \
     odio suscipit, suscipit velit in, ullamcorper velit. In bibendum pulvinar ipsum. Fusce \
     elementum maximus mattis. Donec sed mauris nec ante eleifend dapibus non faucibus massa. \
     Vivamus a auctor ligula. Cras hendrerit, velit sit amet sagittis placerat, elit elit feugiat \
     quam, vel aliquet ligula elit sit amet nibh. Fusce dignissim, orci vitae sodales ornare, \
     lacus risus facilisis sem, a imperdiet lectus massa at velit. Etiam sed magna congue, \
     pulvinar diam quis, facilisis risus. Sed semper, lectus vulputate luctus fermentum, quam \
     lacus consectetur arcu, ac mollis ipsum metus vel nunc. Ut posuere arcu enim, id dictum arcu \
     sagittis in. Mauris a lectus nec ligula eleifend rutrum. Class aptent taciti sociosqu ad \
     litora torquent per conubia massa nunc.";

fn test_line_wrap(state: &State,
                  &MsgMetadata { prefix, .. }: &MsgMetadata,
                  arg: &str)
                  -> BotCmdResult {
    match state.have_owner(prefix) {
        Ok(true) => BotCmdResult::Ok(Reaction::Reply(LOREM_IPSUM_TEXT.into())),
        Ok(false) => BotCmdResult::Unauthorized,
        Err(e) => BotCmdResult::Err(e),
    }
}
