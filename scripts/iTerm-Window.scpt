on run
    set argsCmd to "{command}"
    set argsTitle to "Custom title"
    scriptRun(argsCmd, argsTitle)
end run

on scriptRun(argsCmd, argsTitle)
    set withCmd to (argsCmd)
    set theTitle to (argsTitle)
    CommandRun(withCmd, theTitle)
end scriptRun

on CommandRun(withCmd, theTitle)
    tell application "iTerm"
        if it is not running then
            activate
            if (count windows) is 0 then
                my NewWin()
            end if
        else
            my NewWin()
        end if
        tell the current window
            tell the current session
                set name to theTitle
                write text withCmd
            end tell
        end tell
    end tell
end CommandRun

on NewWin()
    tell application "iTerm"
        create window with profile "Default"
    end tell
end NewWin
