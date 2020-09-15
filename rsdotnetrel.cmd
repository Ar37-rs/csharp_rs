@ECHO off
set rsbuildmodedir=release
set csbuildmodedir=Release
rem if net5.0
rem set framework=net5.0
set framework=netcoreapp3.1

cargo build --release
IF EXIST "bin\%csbuildmodedir%\%framework%" (
   copy "target\%rsbuildmodedir%\*.dll" "bin\%csbuildmodedir%\%framework%\*.dll"
   ECHO    ok.
   dotnet build -c %csbuildmodedir%
) else (
    mkdir "bin\%csbuildmodedir%\%framework%"
    copy "target\%rsbuildmodedir%\*.dll" "bin\%csbuildmodedir%\%framework%\*.dll"
    ECHO    ok.
    dotnet build -c %csbuildmodedir%
)
