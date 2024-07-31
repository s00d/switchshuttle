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
	set currentClipboard to do shell script "pbpaste"
	set the clipboard to withCmd
	tell application "Warp"
			activate
			tell application "System Events"
					tell process "Warp"
						key code 9 using {command down} -- cmd + V
						delay 0.1
						key code 36 -- entr
					end tell
		end tell
	end tell
	do shell script "echo " & quoted form of currentClipboard & " | pbcopy"
end CommandRun