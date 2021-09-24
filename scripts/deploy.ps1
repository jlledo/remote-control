if ($args.Count -ne 1) {
    throw "at least 1 argument is required for the deploy dir"
}

$dir = Get-Item(Split-Path $script:MyInvocation.MyCommand.Path)

$binPath = Join-Path $dir.Parent.FullName target\debug\remote-control.exe
Write-Output "Bin path: $binPath"
Write-Output "Deploy dir: $($Args[0])"

Get-Process "remote-control" -ErrorAction SilentlyContinue | Stop-Process

# Hack to wait till process has exited
# Sometimes exe is still in use even though process has exited
Start-Sleep 0.5

Copy-Item $binPath $Args[0]
