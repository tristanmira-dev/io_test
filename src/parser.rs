use crate::globals;

struct Command {
    keyword: String,
    no_of_arguments: i32
}

struct CommandBundle {
    command_list: Vec<Command>
}

impl CommandBundle {
    fn add_command(self: &mut Self, keyword: String, no_of_arguments: i32) {
        self.command_list.push(Command {
            keyword,
            no_of_arguments
        });
    }

    fn get_arguments(self: &Self, keyword: String) -> globals::Ret<i32> {
        for command in &self.command_list {
            if command.keyword == keyword {
                return globals::Ret::Val(command.no_of_arguments);
            }
        }
        return globals::Ret::Err;
    }
}
