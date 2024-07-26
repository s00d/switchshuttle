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
    tell application "Warp"
        activate
        tell application "System Events"
            keystroke "t" using {command down}
        end tell
        do script withCmd in selected tab of front window
        set custom title of front window to theTitle
    end tell
end CommandRun
