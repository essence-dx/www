set shell := ["pwsh.exe", "-c"]

build:
    cargo build --release -p dx-www -j 12
    @New-Item -ItemType Directory -Force -Path G:\Dx\bin | Out-Null
    @Copy-Item target\release\dx-www.exe G:\Dx\bin\dx-www.exe -Force
    @Write-Host "Build complete and deployed to G:\Dx\bin"






