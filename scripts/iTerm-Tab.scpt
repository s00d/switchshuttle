on run
    set argsCmd to "{command}"
    set argsTheme to "{theme}"
    set argsTitle to "{title}"
    scriptRun(argsCmd, argsTheme, argsTitle)
end run

on scriptRun(argsCmd, argsTitle)
    set withCmd to (argsCmd)
    -- set withTheme to (argsTheme)
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
            my SetWinParam(theTitle, withCmd)
        else if (count windows) is 0 then
            my NewWin()
            my SetWinParam(theTitle, withCmd)
        else
            my NewTab()
            my SetTabParam(theTitle, withCmd)
        end if
    end tell
end CommandRun

on NewWin()
    tell application "iTerm"
        create window with profile "Default"
    end tell
end NewWin

on SetWinParam(argsTitle, argsCmd)
    tell application "iTerm"
        tell the current window
            tell the current session
                set name to argsTitle
                write text argsCmd
            end tell
        end tell
    end tell
end SetWinParam

on NewTab()
    tell application "iTerm"
        activate
        tell the current window
            create tab with profile "Default"
        end tell
    end tell
end NewTab

on SetTabParam(argsTitle, argsCmd)
    tell application "iTerm"
        tell the current window
            tell the current tab
                tell the current session
                    set name to argsTitle
                    write text argsCmd
                end tell
            end tell
        end tell
    end tell
end SetTabParam
