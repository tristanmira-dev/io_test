use crate::globals;

struct Command {
    keyword: String,
    no_of_arguments: i32,
    action: fn(Vec<String>, i32) -> ()
}

struct CommandBundle {
    command_list: Vec<Command>
}

impl CommandBundle {
    fn add_command(self: &mut Self, keyword: String, no_of_arguments: i32, action: fn(Vec<String>, i32)->()) {
        self.command_list.push(Command {
            keyword,
            no_of_arguments,
            action
        });
    }

    fn run_command(self: &Self, keyword: String, args: Vec<String>) {
        for command in &self.command_list {
            if command.keyword == keyword {
                (command.action)(args, command.no_of_arguments);   
                return;
            }
        }
    }
}
