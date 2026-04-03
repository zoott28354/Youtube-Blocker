cask "youtube-blocker" do
  version "1.2.1"
  sha256 "1cabb4644ce224dd562042101be43b72ca4764e54c7326089a7cb7c7dc3c95d7"

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

  caveats <<~EOS
    YouTube Blocker requires administrator privileges to modify the hosts file.
    You will be prompted for your password when launching the app.
  EOS
end
