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
  tell application "Hyper"
    activate
    tell application "System Events"
      key code 17 using {command down} -- Cmd + T
      delay 0.1
      key code 9 using {command down} -- cmd + V
      delay 0.1
      key code 36 -- entr
    end tell
  end tell
end CommandRun
