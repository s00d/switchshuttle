on run
    set argsCmd to "{command}"
    set argsTitle to "{title}"
    scriptRun(argsCmd, argsTitle)
end run

on scriptRun(argsCmd, argsTitle)
    set withCmd to (argsCmd)
    set theTitle to (argsTitle)
    CommandRun(withCmd, theTitle)
end scriptRun

on CommandRun(withCmd, theTitle)
    tell application "Visual Studio Code"
        activate
        tell application "System Events"
            keystroke "n" using {command down, shift down}
            delay 0.5
            keystroke "`" using {command down}
            delay 0.5
            keystroke withCmd
            keystroke return
        end tell
    end tell
end CommandRun 