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
        reopen
        activate
        set custom title of front window to theTitle
        do script withCmd in front window
    end tell
end CommandRun
