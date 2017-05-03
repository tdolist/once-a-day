use clap::{App, Arg, AppSettings, SubCommand, Shell};

pub fn cli() -> App<'static, 'static> {
    App::new("once-a-day")
        .version(crate_version!())
        .author("Felix Wittwer <dev@felixwittwer.de>, Felix Döring <development@felixdoering.com>")
        .about("Make a compliment once a day and become a better person")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::DeriveDisplayOrder)
        .subcommand(SubCommand::with_name("run")
            .about("Run with notifications localy")
            .arg(Arg::with_name("time")
                .help("Schedule time")
                .takes_value(true)
                .required(true)))
        .subcommand(SubCommand::with_name("server")
            .about("Run as a server to send mails")
            .arg(Arg::with_name("mail")
                .help("Your mail address")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("host")
                .help("SMTP server")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("user")
                .help("Username for host")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("password")
                .help("Password for host")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("port")
                .help("SMTP port (default will be 587)")
                .takes_value(true))
            .arg(Arg::with_name("time")
                .help("Schedule time")
                .takes_value(true)
                .required(true)))
        .subcommand(SubCommand::with_name("completions")
            .about("Generate completion scripts for your shell.")
            .after_help(COMPLETION_HELP)
            .setting(AppSettings::ArgRequiredElseHelp)
            .arg(Arg::with_name("shell").possible_values(&Shell::variants())))
}

static COMPLETION_HELP: &'static str =
r"NOTICE:
    One can generate a completion script for once-a-day that is
    compatible with a given shell. The script is output on `stdout`
    allowing one to re-direct the output to the file of their
    choosing. Where you place the file will depend on which shell, and
    which operating system you are using. Your particular
    configuration may also determine where these scripts need to be
    placed.
    Here are some common set ups for the three supported shells under
    Unix and similar operating systems (such as GNU/Linux).
    BASH:
    Completion files are commonly stored in `/etc/bash_completion.d/`
    Run the command:
    `oad completions bash > /etc/bash_completion.d/oad.bash-completion`
    This installs the completion script. You may have to log out and
    log back in to your shell session for the changes to take affect.
    BASH (macOS/Homebrew):
    Homebrew stores bash completion files within the Homebrew directory.
    With the `bash-completion` brew formula installed, run the command:
    `oad completions bash > $(brew --prefix)/etc/bash_completion.d/oad.bash-completion`
    FISH:
    Fish completion files are commonly stored in
    `$HOME/.config/fish/completions`
    Run the command:
    `oad completions fish > ~/.config/fish/completions/oad.fish`
    This installs the completion script. You may have to log out and
    log back in to your shell session for the changes to take affect.
    ZSH:
    ZSH completions are commonly stored in any directory listed in
    your `$fpath` variable. To use these completions, you must either
    add the generated script to one of those directories, or add your
    own to this list.
    Adding a custom directory is often the safest best if you're
    unsure of which directory to use. First create the directory, for
    this example we'll create a hidden directory inside our `$HOME`
    directory
    `mkdir ~/.zfunc`
    Then add the following lines to your `.zshrc` just before
    `compinit`
    `fpath+=~/.zfunc`
    Now you can install the completions script using the following
    command
    `oad completions zsh > ~/.zfunc/_oad`
    You must then either log out and log back in, or simply run
    `exec zsh`
    For the new completions to take affect.
    CUSTOM LOCATIONS:
    Alternatively, you could save these files to the place of your
    choosing, such as a custom directory inside your $HOME. Doing so
    will require you to add the proper directives, such as `source`ing
    inside your login script. Consult your shells documentation for
    how to add such directives.
    POWERSHELL:
    The powershell completion scripts require PowerShell v5.0+ (which
    comes Windows 10, but can be downloaded separately for windows 7
    or 8.1).
    First, check if a profile has already been set
    `PS C:\> Test-Path $profile`
    If the above command returns `False` run the following
    `PS C:\> New-Item -path $profile -type file -force`
    Now open the file provided by `$profile` (if you used the
    `New-Item` command it will be
    `%USERPROFILE%\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1`
    Next, we either save the completions file into our profile, or
    into a separate file and source it inside our profile. To save the
    completions into our profile simply use
    `PS C:\> oad completions powershell >> %USERPROFILE%\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1`";