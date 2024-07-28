on run
    set argsCmd to "{command}"
    set argsTitle to "{title}"
    CommandRun(argsCmd, argsTitle)
end run

on scriptRun(argsCmd, argsTitle)
    set withCmd to (argsCmd)
    set theTitle to (argsTitle)
    CommandRun(withCmd, theTitle)
end scriptRun

on CommandRun(withCmd, theTitle)
    tell application "Terminal"
        if it is not running then
            --if this is the first time Terminal is running you have specify window 1
            --if you dont do this you will get two windows and the title wont be set
            set newTerm to do script withCmd in window 0
        else
            set newTerm to do script withCmd
        end if
        activate
        set custom title of front window to theTitle
    end tell
end CommandRun

