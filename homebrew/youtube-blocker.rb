cask "youtube-blocker" do
  version "1.2.3"

  on_arm do
    sha256 "39b2651500865fa5159cd95e6aa8f1fef7e3b9e7d5a94cb79d9f6fa6dd53d845"
    url "https://github.com/zoott28354/Youtube-Blocker/releases/download/v#{version}/YouTubeBlocker_#{version}_aarch64.dmg"
  end

  on_intel do
    sha256 "24bbe10f5dc4a2883793797e155bc31cb7d0220053b41e104a28a86447773fc5"
    url "https://github.com/zoott28354/Youtube-Blocker/releases/download/v#{version}/YouTubeBlocker_#{version}_x64.dmg"
  end

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
