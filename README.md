# ari

A process runner for busy bees

This tool tries to help with remembering commands. Specific to a folder. It does not try to replace makefile or justfile. It stores all the commands in one config file on your system. The idea is that you put in commands that only you use. Or only make sense on your system.

To put it into concrete terms, I mostly use it in combination with https://github.com/micmine/logana and/or https://github.com/watchexec/watchexec

For example this is one of my saved commands:
``` command
watchexec -r -e go 'logana -c "go run ." -p go'
```
This command will build and run a go program. And if there are build errors they will be analyzed by logana and the errors of that build will be saved to a file.

## Shell integrations
Nushell
``` nushell
# Return available actions for the current directory
def local-actions [] {
  ari --print-actions | lines | split column -r '\t' | rename value description
}
# Run ari action. In current directory
export def a [ action: string@local-actions ] {
  let command = (ari -a $action)
  nu -c $command
}
# Add ari action for the current directory. Provide a action name as a parameter and it will save this action with that last entered command
export def "aset" [action] {
  let cmd = (history | get command | last)
  ari -a $action --set $cmd
}
# Runns a command from README.md code blocks. (Each line of the code block is recomended separately)
export def "ar" [] {
  let command = (ari --find-actions | fzf)
  nu -c $command
  echo $command
}
```
