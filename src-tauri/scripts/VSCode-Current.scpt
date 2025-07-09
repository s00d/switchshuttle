on run
    set argsCmd to "{command}"
    scriptRun(argsCmd)
end run

on scriptRun(argsCmd)
    set withCmd to (argsCmd)
    CommandRun(withCmd)
end scriptRun

on CommandRun(withCmd)
    tell application "Visual Studio Code"
        activate
        tell application "System Events"
            keystroke "`" using {command down}
            delay 0.5
            keystroke withCmd
            keystroke return
        end tell
    end tell
end CommandRun 