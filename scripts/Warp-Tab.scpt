on run
	set argsCmd to "{command}"
	set argsTheme to "{theme}"
	set argsTitle to "{title}"
	scriptRun(argsCmd, argsTheme, argsTitle)
end run

on scriptRun(argsCmd, argsTheme, argsTitle)
	set withCmd to (argsCmd)
	set withTheme to (argsTheme)
	set theTitle to (argsTitle)
	CommandRun(withCmd, withTheme, theTitle)
end scriptRun

on CommandRun(withCmd, withTheme, theTitle)
	tell application "Warp"
			activate
            tell application "System Events"
                tell process "Warp"
                    delay 0.5
                    keystroke "t" using command down
                end tell
	        end tell
            tell application "System Events"
                tell process "Warp"
                    delay 0.5
                    keystroke withCmd
                    delay 0.5
                    key code 36
                end tell
	        end tell
	end tell
end CommandRun
