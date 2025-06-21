cask "switchshuttle" do
  version "1.5"

  if Hardware::CPU.intel?
    url "https://github.com/s00d/switchshuttle/releases/download/app-v#{version}/switch-shuttle_#{version}_x64.dmg"
    sha256 "95bf3af6e156007dfafbfb0fa4fdf5da04c0e4320ff0e307cdbad344cf6f5253"
  else
    url "https://github.com/s00d/switchshuttle/releases/download/app-v#{version}/switch-shuttle_#{version}_aarch64.dmg"
    sha256 "95bf3af6e156007dfafbfb0fa4fdf5da04c0e4320ff0e307cdbad344cf6f5253"
  end

  name "SwitchShuttle"
  desc "Cross-platform terminal command manager with global hotkeys - organize, customize, and quickly access your most-used terminal operations with a sleek interface"
  homepage "https://github.com/s00d/switchshuttle"

  app "switch-shuttle.app"

  postflight do
    system_command "chmod", args: ["+x", "/Applications/switch-shuttle.app"]
    system_command "xattr", args: ["-cr", "/Applications/switch-shuttle.app"]
    system_command "codesign", args: ["--force", "--deep", "--sign", "-", "/Applications/switch-shuttle.app"]
  end

  # Uncomment the following lines if you want to remove configuration on uninstall
  # zap trash: [
  #   "~/.config/switch-shuttle",
  #   "~/Library/Application Support/switch-shuttle",
  #   "~/Library/Preferences/com.SwitchShuttle.app.plist",
  #   "~/Library/Saved Application State/com.SwitchShuttle.app.savedState"
  # ]
end 