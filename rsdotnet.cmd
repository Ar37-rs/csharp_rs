@ECHO off
set rsbuildmodedir=debug
set csbuildmodedir=debug
rem if net5.0
rem set framework=net5.0
set framework=netcoreapp3.1

cargo build
IF EXIST "bin\%csbuildmodedir%\%framework%" (
   copy "target\%rsbuildmodedir%\*.dll" "bin\%csbuildmodedir%\%framework%\*.dll"
   ECHO    ok.
   dotnet run
) else (
    mkdir "bin\%csbuildmodedir%\%framework%"
    copy "target\%rsbuildmodedir%\*.dll" "bin\%csbuildmodedir%\%framework%\*.dll"
    ECHO    ok.
    dotnet run
)
