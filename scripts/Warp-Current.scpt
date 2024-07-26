on run
    set argsCmd to "{command}"
    scriptRun(argsCmd)
end run

on scriptRun(argsCmd)
    set withCmd to (argsCmd)
    CommandRun(withCmd)
end scriptRun

on CommandRun(withCmd)
    tell application "Warp"
        reopen
        activate
        tell the current window
            tell the current session
                write text withCmd
            end tell
        end tell
    end tell
end CommandRun
