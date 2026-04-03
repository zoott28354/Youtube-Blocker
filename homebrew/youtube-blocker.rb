cask "youtube-blocker" do
  version "1.2.0"
  sha256 "83bb80e1c8e9fe51515066e60285efe1744166a88f836682c29bd3f612c91382"

  url "https://github.com/zoott28354/Youtube-Blocker/releases/download/v#{version}/YouTubeBlocker_#{version}_aarch64.dmg"
  name "YouTube Blocker"
  desc "Desktop app to block websites at system level, built for parents"
  homepage "https://github.com/zoott28354/Youtube-Blocker"

  depends_on macos: ">= :catalina"

  app "YouTubeBlocker.app"

  postflight do
    system_command "/usr/bin/xattr",
                   args: ["-cr", "#{appdir}/YouTubeBlocker.app"],
                   sudo: false
  end

  zap trash: [
    "/Library/Managed Preferences/com.google.Chrome.plist",
    "/Library/Managed Preferences/com.microsoft.Edge.plist",
    "/Library/Managed Preferences/com.brave.Browser.plist",
    "/Library/Managed Preferences/com.vivaldi.Vivaldi.plist",
    "/Library/Managed Preferences/com.operasoftware.Opera.plist",
    "/Library/Managed Preferences/org.chromium.Chromium.plist",
    "/etc/pf.anchors/com.youtubeblocker",
  ]

  caveats <<~EOS
    YouTube Blocker requires administrator privileges to modify the hosts file,
    firewall rules, and browser policies. You will be prompted for your password
    when launching the app.
  EOS
end
