$url = "https://github.com/protocolbuffers/protobuf/releases/download/v25.3/protoc-25.3-win64.zip";
$location = Get-Location;

$outputPath = "$($location.Path)\protoc";
mkdir $outputPath;

Invoke-WebRequest -Uri $url -OutFile "$outputPath\file.zip";
Expand-Archive -Path "$outputPath\file.zip" -DestinationPath $outputPath;

$binPath = "$outputPath\bin";
[Environment]::SetEnvironmentVariable("Path", "$env:Path;$binPath", [System.EnvironmentVariableTarget]::Machine);
[Environment]::SetEnvironmentVariable("PROTOC", "$binPath", [System.EnvironmentVariableTarget]::Machine);
