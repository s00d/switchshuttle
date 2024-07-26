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
    tell application "Terminal"
        if it is not running then
            do script withCmd
        else
            do script withCmd in window 1
        end if
        activate
        set custom title of front window to theTitle
    end tell
end CommandRun
