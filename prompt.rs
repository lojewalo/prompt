use std::env;

// Basic prompts

struct Prompt {
    parts: Vec<String>
}

impl Prompt {

    fn new() -> Prompt {
        Prompt {
            parts: Vec::new()
        }
    }

    fn raw<S: Into<String>>(&mut self, string: S) -> &mut Prompt {
        self.parts.push(string.into());
        self
    }

    fn foreground(&mut self, color: i32) -> &mut Prompt {
        self.raw(format!("%{{$FG[{:03}]%}}", color));
        self
    }

    fn background(&mut self, color: i32) -> &mut Prompt {
        self.raw(format!("%{{$BG[{:03}]%}}", color));
        self
    }

    fn reset_color(&mut self) -> &mut Prompt {
        self.raw("%{$reset_color%}");
        self
    }

    fn current_working_directory(&mut self, home: bool, components: i32) -> &mut Prompt {
        self.raw("%".to_string() + &components.to_string() + if home { "~" } else { "/" });
        self
    }

    fn hostname(&mut self, full: bool) -> &mut Prompt {
        self.raw("%".to_string() + if full { "M" } else { "m" });
        self
    }

    fn username(&mut self) -> &mut Prompt {
        self.raw("%n");
        self
    }

    fn prompt(&mut self) -> &mut Prompt {
        self.raw("%#");
        self
    }

    fn return_status(&mut self) -> &mut Prompt {
        self.raw("%?");
        self
    }

    fn to_prompt_string(&mut self) -> String {
        self.parts.join("")
    }
}

// Extensions to prompt

/*
trait GitPrompt {
    fn git(&self);
}

impl GitPrompt for Prompt {
    fn git(&self) {

    }
}
*/

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    if args.len() < 1 {
        return;
    }
    let mut prompt = Prompt::new();
    match args[0].to_lowercase().as_ref() {
         "prompt" => {
            prompt
                .foreground(33)
                .current_working_directory(true, 3)
                .reset_color()
                .raw("$(git_prompt_info) ")
                .foreground(240)
                .raw("(")
                .hostname(false)
                .raw(":")
                .username()
                .raw(")")
                .reset_color()
                .raw("\n")
                .foreground(255)
                .prompt()
                .reset_color()
                .raw(" ");
         },
         "rprompt" => {
            prompt
                .raw("ret: ")
                .return_status();
         },
         _ => ()
     };
    print!("{}", prompt.to_prompt_string());
}
