param(
    [int]$ProcessId = 0,
    [ValidateRange(10, 3600)]
    [int]$Seconds = 300
)

$ErrorActionPreference = "Stop"

if ($ProcessId -gt 0) {
    $target = Get-Process -Id $ProcessId
} else {
    $target = Get-Process -Name "flowrail" | Select-Object -First 1
}

if (-not $target) {
    throw "FlowRail is not running. Launch target\release\flowrail.exe first."
}

$target.Refresh()
$startCpu = $target.TotalProcessorTime.TotalSeconds
$start = Get-Date
$peakWorkingSet = [double]$target.WorkingSet64
$peakPrivateBytes = [double]$target.PrivateMemorySize64

Write-Host "Sampling $($target.ProcessName) (PID $($target.Id)) for $Seconds seconds..."
for ($i = 0; $i -lt $Seconds; $i++) {
    Start-Sleep -Seconds 1
    $target.Refresh()
    if ($target.HasExited) {
        throw "FlowRail exited during the measurement."
    }
    $peakWorkingSet = [Math]::Max($peakWorkingSet, [double]$target.WorkingSet64)
    $peakPrivateBytes = [Math]::Max($peakPrivateBytes, [double]$target.PrivateMemorySize64)
}

$elapsed = ((Get-Date) - $start).TotalSeconds
$cpuSeconds = $target.TotalProcessorTime.TotalSeconds - $startCpu
$logicalProcessors = [Environment]::ProcessorCount
$averageCpu = 100.0 * $cpuSeconds / ($elapsed * $logicalProcessors)

[pscustomobject]@{
    ProcessId = $target.Id
    SampleSeconds = [Math]::Round($elapsed, 1)
    AverageTotalSystemCpuPercent = [Math]::Round($averageCpu, 3)
    PeakWorkingSetMiB = [Math]::Round($peakWorkingSet / 1MB, 1)
    PeakPrivateBytesMiB = [Math]::Round($peakPrivateBytes / 1MB, 1)
} | Format-List
