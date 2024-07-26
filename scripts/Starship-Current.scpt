on run
    set argsCmd to "{command}"
    scriptRun(argsCmd)
end run

on scriptRun(argsCmd)
    set withCmd to (argsCmd)
    CommandRun(withCmd)
end scriptRun

on CommandRun(withCmd)
    tell application "Terminal"
        reopen
        activate
        do script withCmd in front window
    end tell
end CommandRun
