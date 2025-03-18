# Set Working Directory
Split-Path $MyInvocation.MyCommand.Path | Push-Location
[Environment]::CurrentDirectory = $PWD

Remove-Item "$env:RELOADEDIIMODS/riri.criadx/*" -Force -Recurse
dotnet publish "./riri.criadx.csproj" -c Release -o "$env:RELOADEDIIMODS/riri.criadx" /p:OutputPath="./bin/Release" /p:ReloadedILLink="true"

# Restore Working Directory
Pop-Location