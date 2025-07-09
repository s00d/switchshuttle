use crate::config::CommandConfig;
use std::process::Command;

static SCRIPTS_DIR: include_dir::Dir = include_dir::include_dir!("scripts");

fn read_script(script_path: &str) -> Option<String> {
    SCRIPTS_DIR
        .get_file(script_path)
        .map(|file| file.contents_utf8().unwrap().to_string())
}

#[cfg(target_os = "macos")]
fn execute_command_impl(
    commands_to_execute: &[String],
    terminal: &str,
    launch_in: &str,
    theme: &String,
    title: &String,
) {
    println!("Executing on macOS");

    let script_path = match (terminal, launch_in) {
        ("iterm", "current") => "iTerm-Current.scpt",
        ("iterm", "new_tab") => "iTerm-Tab.scpt",
        ("iterm", "new_window") => "iTerm-Window.scpt",
        ("terminal", "current") => "Terminal-Current.scpt",
        ("terminal", "new_tab") => "Terminal-Tab.scpt",
        ("terminal", "new_window") => "Terminal-Window.scpt",
        ("warp", "current") => "Warp-Current.scpt",
        ("warp", "new_tab") => "Warp-Tab.scpt",
        ("warp", "new_window") => "Warp-Window.scpt",
        ("hyper", "current") => "Hyper-Current.scpt",
        ("hyper", "new_tab") => "Hyper-Tab.scpt",
        ("hyper", "new_window") => "Hyper-Window.scpt",
        ("alacritty", "current") => "Alacritty-Current.scpt",
        ("alacritty", "new_tab") => "Alacritty-Tab.scpt",
        ("alacritty", "new_window") => "Alacritty-Window.scpt",
        ("vscode-terminal", "current") => "VSCode-Current.scpt",
        ("vscode-terminal", "new_tab") => "VSCode-Tab.scpt",
        ("vscode-terminal", "new_window") => "VSCode-Window.scpt",
        _ => "",
    };

    if script_path.is_empty() {
        println!("Unsupported terminal or launch_in option");
        return;
    }

    println!("Script path: {}", script_path);

    let script_content = match read_script(script_path) {
        Some(content) => content,
        None => {
            println!("Failed to read script: {}", script_path);
            return;
        }
    };

    for command in commands_to_execute {
        let script = script_content
            .replace("{command}", command)
            .replace("{theme}", theme)
            .replace("{title}", title);

        println!("Executing script: {}", script);

        let output = Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            println!("Command succeeded: {}", command);
            println!(
                "Standard Output: {}",
                String::from_utf8_lossy(&output.stdout)
            );
        } else {
            println!("Command failed: {}", command);
            println!(
                "Standard Output: {}",
                String::from_utf8_lossy(&output.stdout)
            );
            println!(
                "Standard Error: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            break;
        }
    }
}

#[cfg(target_os = "windows")]
fn execute_command_impl(
    commands_to_execute: &[String],
    terminal: &str,
    launch_in: &str,
    _theme: &String,
    _title: &String,
) {
    println!("Executing on Windows");

    for command in commands_to_execute {
        println!("Executing command: {}", command);

        let status = match (terminal, launch_in) {
            ("hyper", "current") => Command::new("cmd")
                .args(&["/C", &format!("start hyper -e \"{}\"", command)])
                .status(),
            ("hyper", "new_tab") => Command::new("cmd")
                .args(&["/C", &format!("start hyper --new-tab -e \"{}\"", command)])
                .status(),
            ("hyper", "new_window") => Command::new("cmd")
                .args(&[
                    "/C",
                    &format!("start hyper --new-window -e \"{}\"", command),
                ])
                .status(),

            ("wsl", "current") => Command::new("wsl")
                .arg("-e")
                .arg("bash")
                .arg("-c")
                .arg(command)
                .status(),
            ("wsl", "new_tab") => Command::new("cmd")
                .args(&["/C", &format!("start wsl -e bash -c \"{}\"", command)])
                .status(),
            ("wsl", "new_window") => Command::new("cmd")
                .args(&["/C", &format!("start wsl -e bash -c \"{}\"", command)])
                .status(),

            ("powershell", "current") => Command::new("powershell")
                .arg("-Command")
                .arg(command)
                .status(),
            ("powershell", "new_tab") => Command::new("cmd")
                .args(&["/C", &format!("start powershell -Command \"{}\"", command)])
                .status(),
            ("powershell", "new_window") => Command::new("cmd")
                .args(&["/C", &format!("start powershell -Command \"{}\"", command)])
                .status(),

            ("windows-terminal", "current") => Command::new("cmd")
                .args(&["/C", &format!("wt -d . \"{}\"", command)])
                .status(),
            ("windows-terminal", "new_tab") => Command::new("cmd")
                .args(&["/C", &format!("start wt -d . new-tab \"{}\"", command)])
                .status(),
            ("windows-terminal", "new_window") => Command::new("cmd")
                .args(&["/C", &format!("start wt -d . new-window \"{}\"", command)])
                .status(),

            ("conemu", "current") => Command::new("cmd")
                .args(&["/C", &format!("conemu /cmd \"{}\"", command)])
                .status(),
            ("conemu", "new_tab") => Command::new("cmd")
                .args(&["/C", &format!("start conemu /new-tab /cmd \"{}\"", command)])
                .status(),
            ("conemu", "new_window") => Command::new("cmd")
                .args(&[
                    "/C",
                    &format!("start conemu /new-window /cmd \"{}\"", command),
                ])
                .status(),

            ("cmder", "current") => Command::new("cmd")
                .args(&["/C", &format!("cmder /cmd \"{}\"", command)])
                .status(),
            ("cmder", "new_tab") => Command::new("cmd")
                .args(&["/C", &format!("start cmder /new-tab /cmd \"{}\"", command)])
                .status(),
            ("cmder", "new_window") => Command::new("cmd")
                .args(&[
                    "/C",
                    &format!("start cmder /new-window /cmd \"{}\"", command),
                ])
                .status(),

            ("git-bash", "current") => Command::new("cmd")
                .args(&[
                    "/C",
                    &format!(
                        "\"C:\\Program Files\\Git\\bin\\bash.exe\" -c \"{}\"",
                        command
                    ),
                ])
                .status(),
            ("git-bash", "new_tab") => Command::new("cmd")
                .args(&[
                    "/C",
                    &format!(
                        "start \"C:\\Program Files\\Git\\bin\\bash.exe\" -c \"{}\"",
                        command
                    ),
                ])
                .status(),
            ("git-bash", "new_window") => Command::new("cmd")
                .args(&[
                    "/C",
                    &format!(
                        "start \"C:\\Program Files\\Git\\bin\\bash.exe\" -c \"{}\"",
                        command
                    ),
                ])
                .status(),

            ("alacritty", "current") => Command::new("cmd")
                .args(&["/C", &format!("alacritty -e \"{}\"", command)])
                .status(),
            ("alacritty", "new_tab") => Command::new("cmd")
                .args(&["/C", &format!("start alacritty -e \"{}\"", command)])
                .status(),
            ("alacritty", "new_window") => Command::new("cmd")
                .args(&["/C", &format!("start alacritty -e \"{}\"", command)])
                .status(),

            ("wezterm", "current") => Command::new("cmd")
                .args(&["/C", &format!("wezterm cli spawn -- \"{}\"", command)])
                .status(),
            ("wezterm", "new_tab") => Command::new("cmd")
                .args(&[
                    "/C",
                    &format!("start wezterm cli spawn --new-tab -- \"{}\"", command),
                ])
                .status(),
            ("wezterm", "new_window") => Command::new("cmd")
                .args(&[
                    "/C",
                    &format!("start wezterm cli spawn --new-window -- \"{}\"", command),
                ])
                .status(),

            ("tabby", "current") => Command::new("cmd")
                .args(&["/C", &format!("tabby -e \"{}\"", command)])
                .status(),
            ("tabby", "new_tab") => Command::new("cmd")
                .args(&["/C", &format!("start tabby --new-tab -e \"{}\"", command)])
                .status(),
            ("tabby", "new_window") => Command::new("cmd")
                .args(&[
                    "/C",
                    &format!("start tabby --new-window -e \"{}\"", command),
                ])
                .status(),

            ("terminus", "current") => Command::new("cmd")
                .args(&["/C", &format!("terminus -e \"{}\"", command)])
                .status(),
            ("terminus", "new_tab") => Command::new("cmd")
                .args(&[
                    "/C",
                    &format!("start terminus --new-tab -e \"{}\"", command),
                ])
                .status(),
            ("terminus", "new_window") => Command::new("cmd")
                .args(&[
                    "/C",
                    &format!("start terminus --new-window -e \"{}\"", command),
                ])
                .status(),

            ("mintty", "current") => Command::new("cmd")
                .args(&["/C", &format!("mintty -e \"{}\"", command)])
                .status(),
            ("mintty", "new_tab") => Command::new("cmd")
                .args(&["/C", &format!("start mintty --new-tab -e \"{}\"", command)])
                .status(),
            ("mintty", "new_window") => Command::new("cmd")
                .args(&[
                    "/C",
                    &format!("start mintty --new-window -e \"{}\"", command),
                ])
                .status(),

            ("putty", "current") => Command::new("cmd")
                .args(&["/C", &format!("putty -load \"{}\"", command)])
                .status(),
            ("putty", "new_tab") => Command::new("cmd")
                .args(&["/C", &format!("start putty -new-tab -load \"{}\"", command)])
                .status(),
            ("putty", "new_window") => Command::new("cmd")
                .args(&[
                    "/C",
                    &format!("start putty -new-window -load \"{}\"", command),
                ])
                .status(),

            ("securecrt", "current") => Command::new("cmd")
                .args(&["/C", &format!("securecrt /argfile \"{}\"", command)])
                .status(),
            ("securecrt", "new_tab") => Command::new("cmd")
                .args(&[
                    "/C",
                    &format!("start securecrt /new-tab /argfile \"{}\"", command),
                ])
                .status(),
            ("securecrt", "new_window") => Command::new("cmd")
                .args(&[
                    "/C",
                    &format!("start securecrt /new-window /argfile \"{}\"", command),
                ])
                .status(),

            ("mobaxterm", "current") => Command::new("cmd")
                .args(&["/C", &format!("mobaxterm -newtab \"{}\"", command)])
                .status(),
            ("mobaxterm", "new_tab") => Command::new("cmd")
                .args(&["/C", &format!("start mobaxterm -newtab \"{}\"", command)])
                .status(),
            ("mobaxterm", "new_window") => Command::new("cmd")
                .args(&["/C", &format!("start mobaxterm -newwindow \"{}\"", command)])
                .status(),

            ("royal-tsx", "current") => Command::new("cmd")
                .args(&["/C", &format!("royaltsx -newtab \"{}\"", command)])
                .status(),
            ("royal-tsx", "new_tab") => Command::new("cmd")
                .args(&["/C", &format!("start royaltsx -newtab \"{}\"", command)])
                .status(),
            ("royal-tsx", "new_window") => Command::new("cmd")
                .args(&["/C", &format!("start royaltsx -newwindow \"{}\"", command)])
                .status(),

            ("vscode-terminal", "current") => Command::new("cmd")
                .args(&["/C", &format!("code --new-terminal \"{}\"", command)])
                .status(),
            ("vscode-terminal", "new_tab") => Command::new("cmd")
                .args(&["/C", &format!("start code --new-terminal \"{}\"", command)])
                .status(),
            ("vscode-terminal", "new_window") => Command::new("cmd")
                .args(&[
                    "/C",
                    &format!("start code --new-window --new-terminal \"{}\"", command),
                ])
                .status(),

            ("sublime-terminal", "current") => Command::new("cmd")
                .args(&["/C", &format!("subl --new-window \"{}\"", command)])
                .status(),
            ("sublime-terminal", "new_tab") => Command::new("cmd")
                .args(&["/C", &format!("start subl --new-tab \"{}\"", command)])
                .status(),
            ("sublime-terminal", "new_window") => Command::new("cmd")
                .args(&["/C", &format!("start subl --new-window \"{}\"", command)])
                .status(),

            ("atom-terminal", "current") => Command::new("cmd")
                .args(&["/C", &format!("atom --new-window \"{}\"", command)])
                .status(),
            ("atom-terminal", "new_tab") => Command::new("cmd")
                .args(&["/C", &format!("start atom --new-tab \"{}\"", command)])
                .status(),
            ("atom-terminal", "new_window") => Command::new("cmd")
                .args(&["/C", &format!("start atom --new-window \"{}\"", command)])
                .status(),

            ("notepad++", "current") => Command::new("cmd")
                .args(&["/C", &format!("notepad++ \"{}\"", command)])
                .status(),
            ("notepad++", "new_tab") => Command::new("cmd")
                .args(&["/C", &format!("start notepad++ -multiInst \"{}\"", command)])
                .status(),
            ("notepad++", "new_window") => Command::new("cmd")
                .args(&["/C", &format!("start notepad++ -multiInst \"{}\"", command)])
                .status(),

            ("cygwin", "current") => Command::new("cmd")
                .args(&[
                    "/C",
                    &format!("C:\\cygwin64\\bin\\bash.exe -c \"{}\"", command),
                ])
                .status(),
            ("cygwin", "new_tab") => Command::new("cmd")
                .args(&[
                    "/C",
                    &format!("start C:\\cygwin64\\bin\\bash.exe -c \"{}\"", command),
                ])
                .status(),
            ("cygwin", "new_window") => Command::new("cmd")
                .args(&[
                    "/C",
                    &format!("start C:\\cygwin64\\bin\\bash.exe -c \"{}\"", command),
                ])
                .status(),

            ("msys2", "current") => Command::new("cmd")
                .args(&[
                    "/C",
                    &format!("C:\\msys64\\usr\\bin\\bash.exe -c \"{}\"", command),
                ])
                .status(),
            ("msys2", "new_tab") => Command::new("cmd")
                .args(&[
                    "/C",
                    &format!("start C:\\msys64\\usr\\bin\\bash.exe -c \"{}\"", command),
                ])
                .status(),
            ("msys2", "new_window") => Command::new("cmd")
                .args(&[
                    "/C",
                    &format!("start C:\\msys64\\usr\\bin\\bash.exe -c \"{}\"", command),
                ])
                .status(),

            // Fallback для неизвестных комбинаций
            (_, _) => Command::new("cmd").args(&["/C", command]).status(),
        }
        .expect("Failed to execute command");

        if status.success() {
            println!("Command succeeded: {}", command);
        } else {
            println!("Command failed: {}", command);
            break;
        }
    }
}

#[cfg(target_os = "linux")]
fn execute_command_impl(
    commands_to_execute: &[String],
    terminal: &str,
    launch_in: &str,
    _theme: &String,
    _title: &String,
) {
    println!("Executing on Linux");

    for command in commands_to_execute {
        println!("Executing command: {}", command);

        let status = match (terminal, launch_in) {
            ("hyper", "current") => Command::new("hyper").arg("-e").arg(command).status(),
            ("hyper", "new_tab") => Command::new("hyper")
                .args(&["--new-tab", "-e", command])
                .status(),
            ("hyper", "new_window") => Command::new("hyper")
                .args(&["--new-window", "-e", command])
                .status(),

            ("gnome-terminal", "current") => Command::new("gnome-terminal")
                .arg("--")
                .arg("bash")
                .arg("-c")
                .arg(command)
                .status(),
            ("gnome-terminal", "new_tab") => Command::new("gnome-terminal")
                .args(&["--tab", "--", "bash", "-c", command])
                .status(),
            ("gnome-terminal", "new_window") => Command::new("gnome-terminal")
                .args(&["--new-window", "--", "bash", "-c", command])
                .status(),

            ("konsole", "current") => Command::new("konsole")
                .args(&["-e", "bash", "-c", command])
                .status(),
            ("konsole", "new_tab") => Command::new("konsole")
                .args(&["--new-tab", "-e", "bash", "-c", command])
                .status(),
            ("konsole", "new_window") => Command::new("konsole")
                .args(&["--new-window", "-e", "bash", "-c", command])
                .status(),

            ("xfce4-terminal", "current") => Command::new("xfce4-terminal")
                .args(&["-e", "bash", "-c", command])
                .status(),
            ("xfce4-terminal", "new_tab") => Command::new("xfce4-terminal")
                .args(&["--tab", "-e", "bash", "-c", command])
                .status(),
            ("xfce4-terminal", "new_window") => Command::new("xfce4-terminal")
                .args(&["--new-window", "-e", "bash", "-c", command])
                .status(),

            ("lxterminal", "current") => Command::new("lxterminal")
                .args(&["-e", "bash", "-c", command])
                .status(),
            ("lxterminal", "new_tab") => Command::new("lxterminal")
                .args(&["--new-tab", "-e", "bash", "-c", command])
                .status(),
            ("lxterminal", "new_window") => Command::new("lxterminal")
                .args(&["--new-window", "-e", "bash", "-c", command])
                .status(),

            ("mate-terminal", "current") => Command::new("mate-terminal")
                .args(&["-e", "bash", "-c", command])
                .status(),
            ("mate-terminal", "new_tab") => Command::new("mate-terminal")
                .args(&["--new-tab", "-e", "bash", "-c", command])
                .status(),
            ("mate-terminal", "new_window") => Command::new("mate-terminal")
                .args(&["--new-window", "-e", "bash", "-c", command])
                .status(),

            ("tilix", "current") => Command::new("tilix")
                .args(&["-e", "bash", "-c", command])
                .status(),
            ("tilix", "new_tab") => Command::new("tilix")
                .args(&["--new-tab", "-e", "bash", "-c", command])
                .status(),
            ("tilix", "new_window") => Command::new("tilix")
                .args(&["--new-window", "-e", "bash", "-c", command])
                .status(),

            ("terminator", "current") => Command::new("terminator")
                .args(&["-e", "bash", "-c", command])
                .status(),
            ("terminator", "new_tab") => Command::new("terminator")
                .args(&["--new-tab", "-e", "bash", "-c", command])
                .status(),
            ("terminator", "new_window") => Command::new("terminator")
                .args(&["--new-window", "-e", "bash", "-c", command])
                .status(),

            ("alacritty", "current") => Command::new("alacritty")
                .args(&["-e", "bash", "-c", command])
                .status(),
            ("alacritty", "new_tab") => Command::new("alacritty")
                .args(&["--new-tab", "-e", "bash", "-c", command])
                .status(),
            ("alacritty", "new_window") => Command::new("alacritty")
                .args(&["--new-window", "-e", "bash", "-c", command])
                .status(),

            ("wezterm", "current") => Command::new("wezterm")
                .args(&["cli", "spawn", "--", "bash", "-c", command])
                .status(),
            ("wezterm", "new_tab") => Command::new("wezterm")
                .args(&["cli", "spawn", "--new-tab", "--", "bash", "-c", command])
                .status(),
            ("wezterm", "new_window") => Command::new("wezterm")
                .args(&["cli", "spawn", "--new-window", "--", "bash", "-c", command])
                .status(),

            ("kitty", "current") => Command::new("kitty")
                .args(&["@", "launch", "--type=tab", "bash", "-c", command])
                .status(),
            ("kitty", "new_tab") => Command::new("kitty")
                .args(&["@", "launch", "--type=tab", "bash", "-c", command])
                .status(),
            ("kitty", "new_window") => Command::new("kitty")
                .args(&["@", "launch", "--type=window", "bash", "-c", command])
                .status(),

            ("tabby", "current") => Command::new("tabby")
                .args(&["-e", "bash", "-c", command])
                .status(),
            ("tabby", "new_tab") => Command::new("tabby")
                .args(&["--new-tab", "-e", "bash", "-c", command])
                .status(),
            ("tabby", "new_window") => Command::new("tabby")
                .args(&["--new-window", "-e", "bash", "-c", command])
                .status(),

            ("terminology", "current") => Command::new("terminology")
                .args(&["-e", "bash", "-c", command])
                .status(),
            ("terminology", "new_tab") => Command::new("terminology")
                .args(&["--new-tab", "-e", "bash", "-c", command])
                .status(),
            ("terminology", "new_window") => Command::new("terminology")
                .args(&["--new-window", "-e", "bash", "-c", command])
                .status(),

            ("deepin-terminal", "current") => Command::new("deepin-terminal")
                .args(&["-e", "bash", "-c", command])
                .status(),
            ("deepin-terminal", "new_tab") => Command::new("deepin-terminal")
                .args(&["--new-tab", "-e", "bash", "-c", command])
                .status(),
            ("deepin-terminal", "new_window") => Command::new("deepin-terminal")
                .args(&["--new-window", "-e", "bash", "-c", command])
                .status(),

            ("cool-retro-term", "current") => Command::new("cool-retro-term")
                .args(&["-e", "bash", "-c", command])
                .status(),
            ("cool-retro-term", "new_tab") => Command::new("cool-retro-term")
                .args(&["--new-tab", "-e", "bash", "-c", command])
                .status(),
            ("cool-retro-term", "new_window") => Command::new("cool-retro-term")
                .args(&["--new-window", "-e", "bash", "-c", command])
                .status(),

            ("guake", "current") => Command::new("guake")
                .args(&["-e", "bash", "-c", command])
                .status(),
            ("guake", "new_tab") => Command::new("guake")
                .args(&["--new-tab", "-e", "bash", "-c", command])
                .status(),
            ("guake", "new_window") => Command::new("guake")
                .args(&["--new-window", "-e", "bash", "-c", command])
                .status(),

            ("yakuake", "current") => Command::new("yakuake")
                .args(&["-e", "bash", "-c", command])
                .status(),
            ("yakuake", "new_tab") => Command::new("yakuake")
                .args(&["--new-tab", "-e", "bash", "-c", command])
                .status(),
            ("yakuake", "new_window") => Command::new("yakuake")
                .args(&["--new-window", "-e", "bash", "-c", command])
                .status(),

            ("tilda", "current") => Command::new("tilda")
                .args(&["-e", "bash", "-c", command])
                .status(),
            ("tilda", "new_tab") => Command::new("tilda")
                .args(&["--new-tab", "-e", "bash", "-c", command])
                .status(),
            ("tilda", "new_window") => Command::new("tilda")
                .args(&["--new-window", "-e", "bash", "-c", command])
                .status(),

            ("sakura", "current") => Command::new("sakura")
                .args(&["-e", "bash", "-c", command])
                .status(),
            ("sakura", "new_tab") => Command::new("sakura")
                .args(&["--new-tab", "-e", "bash", "-c", command])
                .status(),
            ("sakura", "new_window") => Command::new("sakura")
                .args(&["--new-window", "-e", "bash", "-c", command])
                .status(),

            ("roxterm", "current") => Command::new("roxterm")
                .args(&["-e", "bash", "-c", command])
                .status(),
            ("roxterm", "new_tab") => Command::new("roxterm")
                .args(&["--new-tab", "-e", "bash", "-c", command])
                .status(),
            ("roxterm", "new_window") => Command::new("roxterm")
                .args(&["--new-window", "-e", "bash", "-c", command])
                .status(),

            ("pantheon-terminal", "current") => Command::new("pantheon-terminal")
                .args(&["-e", "bash", "-c", command])
                .status(),
            ("pantheon-terminal", "new_tab") => Command::new("pantheon-terminal")
                .args(&["--new-tab", "-e", "bash", "-c", command])
                .status(),
            ("pantheon-terminal", "new_window") => Command::new("pantheon-terminal")
                .args(&["--new-window", "-e", "bash", "-c", command])
                .status(),

            ("vscode-terminal", "current") => Command::new("code")
                .args(&["--new-terminal", "--", "bash", "-c", command])
                .status(),
            ("vscode-terminal", "new_tab") => Command::new("code")
                .args(&["--new-terminal", "--", "bash", "-c", command])
                .status(),
            ("vscode-terminal", "new_window") => Command::new("code")
                .args(&[
                    "--new-window",
                    "--new-terminal",
                    "--",
                    "bash",
                    "-c",
                    command,
                ])
                .status(),

            ("sublime-terminal", "current") => Command::new("subl")
                .args(&["--new-window", "--", "bash", "-c", command])
                .status(),
            ("sublime-terminal", "new_tab") => Command::new("subl")
                .args(&["--new-tab", "--", "bash", "-c", command])
                .status(),
            ("sublime-terminal", "new_window") => Command::new("subl")
                .args(&["--new-window", "--", "bash", "-c", command])
                .status(),

            ("atom-terminal", "current") => Command::new("atom")
                .args(&["--new-window", "--", "bash", "-c", command])
                .status(),
            ("atom-terminal", "new_tab") => Command::new("atom")
                .args(&["--new-tab", "--", "bash", "-c", command])
                .status(),
            ("atom-terminal", "new_window") => Command::new("atom")
                .args(&["--new-window", "--", "bash", "-c", command])
                .status(),

            // Fallback для неизвестных комбинаций
            (_, _) => Command::new("sh").arg("-c").arg(command).status(),
        }
        .expect("Failed to execute command");

        if status.success() {
            println!("Command succeeded: {}", command);
        } else {
            println!("Command failed: {}", command);
            break;
        }
    }
}

pub fn execute_command(
    command_config: &CommandConfig,
    terminal: &str,
    launch_in: &str,
    theme: &String,
    title: &String,
) {
    let mut commands_to_execute = Vec::new();

    // Дебаг: Инициализация commands_to_execute
    println!("Initializing commands_to_execute...");

    if let Some(command) = &command_config.command {
        if !command.trim().is_empty() {
            println!("Adding single command: {}", command);
            commands_to_execute.push(command.clone());
        } else {
            println!("Skipping empty single command");
        }
    }

    if let Some(commands) = &command_config.commands {
        println!("Adding multiple commands: {:?}", commands);
        for cmd in commands {
            if !cmd.trim().is_empty() {
                commands_to_execute.push(cmd.clone());
            } else {
                println!("Skipping empty command in multiple commands");
            }
        }
    }

    // Дебаг: Печать списка команд
    println!("Commands to execute: {:?}", commands_to_execute);

    // Проверяем, что есть команды для выполнения
    if commands_to_execute.is_empty() {
        println!("No commands to execute, skipping");
        return;
    }

    let terminal = terminal.to_lowercase();
    let launch_in = launch_in.to_lowercase();

    execute_command_impl(&commands_to_execute, &terminal, &launch_in, theme, title);
}
