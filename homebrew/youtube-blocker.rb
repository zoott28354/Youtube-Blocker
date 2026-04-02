cask "youtube-blocker" do
  version "1.1.4"
  sha256 :no_check # Aggiornare con sha256 reale dopo il primo build macOS

  url "https://github.com/zoott28354/Youtube-Blocker/releases/download/v#{version}/YouTubeBlocker_#{version}_aarch64.dmg"
  name "YouTube Blocker"
  desc "Desktop app to block websites at system level, built for parents"
  homepage "https://github.com/zoott28354/Youtube-Blocker"

  depends_on macos: ">= :catalina"

  app "YouTubeBlocker.app"

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
