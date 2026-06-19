set shell := ["pwsh.exe", "-c"]

build:
    cargo build --release -p dx-www -j 12
    Copy-Item target\release\dx-www.exe G:\Dx\bin\www.exe -Force
    Write-Host "Build complete and deployed to G:\Dx\bin"
