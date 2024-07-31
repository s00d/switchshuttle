on run
  set argsCmd to "{command}"
  scriptRun(argsCmd)
end run

on scriptRun(argsCmd)
  set withCmd to (argsCmd)
  CommandRun(withCmd)
end scriptRun

on CommandRun(withCmd)
  set the clipboard to withCmd
  tell application "Alacritty"
    activate
    tell application "System Events"
      key code 45 using {command down} -- Cmd + N
      delay 0.1
      key code 9 using {command down} -- cmd + V
      delay 0.1
      key code 36 -- entr
    end tell
  end tell
end CommandRun
