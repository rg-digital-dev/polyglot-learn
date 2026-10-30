# Install the `learn` CLI and the polyglot-learn course (Windows, PowerShell):
#
#   irm https://raw.githubusercontent.com/rg-digital-dev/polyglot-learn/main/install.ps1 | iex
#
# It downloads the latest release's binary, checks it against SHA256SUMS,
# installs it, adds it to your user PATH, and fetches the matching version of
# the course into .\polyglot-learn in the folder you run it from. Run it again
# from that same folder, or from inside the course, to update both; your work
# in the course's workspace\ stays.
#
# Settings (environment variables):
#   LEARN_BIN_DIR          where learn.exe goes   (default: %LOCALAPPDATA%\Programs\learn)
#   LEARN_COURSE_DIR       where the course goes  (default: .\polyglot-learn)
#   LEARN_VERSION          a release like 0.1.0   (default: the latest)
#   LEARN_NO_MODIFY_PATH   set to 1 to leave your PATH alone

& {
    $ErrorActionPreference = 'Stop'
    $ProgressPreference = 'SilentlyContinue'   # Invoke-WebRequest is much faster without it

    $repo = 'rg-digital-dev/polyglot-learn'
    $binDir = if ($env:LEARN_BIN_DIR) { $env:LEARN_BIN_DIR } else { Join-Path $env:LOCALAPPDATA 'Programs\learn' }
    $here = (Get-Location).ProviderPath
    $courseDir = if ($env:LEARN_COURSE_DIR) {
        $env:LEARN_COURSE_DIR
    } elseif (Test-Path (Join-Path $here 'course.toml')) {
        $here   # run from inside the course: update it in place
    } else {
        # An administrator PowerShell starts in C:\Windows\System32.
        if ($here.StartsWith($env:windir, [StringComparison]::OrdinalIgnoreCase)) {
            Write-Host "Run this from the folder you want the course in (for example: cd ~), not from $here." -ForegroundColor Red
            return
        }
        Join-Path $here 'polyglot-learn'
    }

    # x86-64 build; Windows on ARM runs it through emulation.
    $target = 'x86_64-pc-windows-msvc'

    if ($env:LEARN_VERSION) {
        $tag = 'v' + $env:LEARN_VERSION.TrimStart('v')
    } else {
        $tag = (Invoke-RestMethod "https://api.github.com/repos/$repo/releases/latest").tag_name
    }
    if ($tag -notmatch '^v\d') { throw "couldn't find the latest release" }
    $version = $tag.Substring(1)
    $name = "learn-$version-$target"
    $base = "https://github.com/$repo/releases/download/$tag"

    $tmp = Join-Path ([IO.Path]::GetTempPath()) ("learn-install-" + [guid]::NewGuid())
    New-Item -ItemType Directory $tmp | Out-Null
    try {
        Write-Host "Downloading learn $version for $target..."
        Invoke-WebRequest -UseBasicParsing "$base/$name.zip" -OutFile "$tmp\$name.zip"
        Invoke-WebRequest -UseBasicParsing "$base/SHA256SUMS" -OutFile "$tmp\SHA256SUMS"
        $line = Get-Content "$tmp\SHA256SUMS" | Where-Object { $_ -match " $([regex]::Escape("$name.zip"))$" }
        $expected = if ($line) { ($line -split ' ')[0] } else { '' }
        $actual = (Get-FileHash "$tmp\$name.zip" -Algorithm SHA256).Hash.ToLower()
        if (-not $expected -or $expected -ne $actual) { throw "checksum mismatch for $name.zip; nothing was installed" }

        Expand-Archive "$tmp\$name.zip" -DestinationPath $tmp
        New-Item -ItemType Directory -Force $binDir | Out-Null
        Copy-Item "$tmp\$name\learn.exe" (Join-Path $binDir 'learn.exe') -Force
        # The binary isn't signed: drop the "downloaded from the internet" mark.
        Unblock-File (Join-Path $binDir 'learn.exe')
        Write-Host "Installed $binDir\learn.exe"

        # The course, at the same version as the binary.
        $git = Get-Command git -ErrorAction SilentlyContinue
        if (Test-Path (Join-Path $courseDir 'course.toml')) {
            if ($git -and (Test-Path (Join-Path $courseDir '.git'))) {
                Write-Host "Updating the course in $courseDir to $tag..."
                git -C $courseDir fetch -q --depth 1 origin tag $tag
                if ($LASTEXITCODE -eq 0) { git -C $courseDir -c advice.detachedHead=false checkout -q $tag }
                if ($LASTEXITCODE -ne 0) { Write-Host "  (couldn't update it; check for local changes with: git -C `"$courseDir`" status)" }
            } else {
                Write-Host "Keeping the course in $courseDir as it is."
            }
        } elseif ((Test-Path $courseDir) -and (Get-ChildItem $courseDir -Force | Select-Object -First 1)) {
            throw "$courseDir exists but isn't the course. Set LEARN_COURSE_DIR to another folder."
        } elseif ($git) {
            Write-Host "Fetching the course into $courseDir..."
            # init + fetch the tag (rather than clone --branch): quiet for annotated
            # tags, and the same steps a later update uses.
            git init -q $courseDir
            git -C $courseDir remote add origin "https://github.com/$repo"
            git -C $courseDir fetch -q --depth 1 origin tag $tag
            if ($LASTEXITCODE -ne 0) { throw "couldn't fetch the course" }
            git -C $courseDir -c advice.detachedHead=false checkout -q $tag
        } else {
            Write-Host "Fetching the course into $courseDir (no git found, so as a plain folder)..."
            Invoke-WebRequest -UseBasicParsing "https://github.com/$repo/archive/refs/tags/$tag.zip" -OutFile "$tmp\course.zip"
            Expand-Archive "$tmp\course.zip" -DestinationPath "$tmp\course"
            $inner = Get-ChildItem "$tmp\course" -Directory | Select-Object -First 1
            New-Item -ItemType Directory -Force $courseDir | Out-Null
            Get-ChildItem $inner.FullName -Force | Move-Item -Destination $courseDir
        }
    } finally {
        Remove-Item -Recurse -Force $tmp -ErrorAction SilentlyContinue
    }

    $userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
    $onPath = ($userPath -split ';') -contains $binDir
    if (-not $onPath -and $env:LEARN_NO_MODIFY_PATH -ne '1') {
        [Environment]::SetEnvironmentVariable('Path', ($(if ($userPath) { "$userPath;" } else { '' }) + $binDir), 'User')
        $onPath = $true
        Write-Host "Added $binDir to your user PATH."
    }
    # Make `learn` work in this window too.
    if (($env:Path -split ';') -notcontains $binDir) { $env:Path = "$env:Path;$binDir" }

    Write-Host ''
    Write-Host 'Checking which language toolchains you have:'
    Push-Location $courseDir
    try { & (Join-Path $binDir 'learn.exe') doctor } finally { Pop-Location }

    Write-Host ''
    if (-not $onPath) { Write-Host "Add $binDir to your PATH to run learn from any terminal." }
    Write-Host 'Then start learning (new terminals pick up the PATH change):'
    Write-Host "  cd $courseDir"
    Write-Host '  learn init           # which language do you already know?'
    Write-Host '  learn start rust     # or typescript, go, php, lua, qml'
    Write-Host '  learn watch          # or learn serve for the web UI'
}
