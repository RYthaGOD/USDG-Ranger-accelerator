$path = "d:\bear strategy\ranger-accelerator\ranger-video\public"
Get-ChildItem -Path $path -Filter *.wav | ForEach-Object {
    $bytes = [System.IO.File]::ReadAllBytes($_.FullName)
    $byteRate = [System.BitConverter]::ToUInt32($bytes, 28)
    $dataSize = [System.BitConverter]::ToUInt32($bytes, 40)
    $duration = $dataSize / $byteRate
    Write-Host "$($_.Name): $duration seconds"
}
