export rsbuildmodedir=debug
export csbuildmodedir=debug
# if net5.0
# export framework=net5.0
export framework=netcoreapp3.1

cargo build
if [ -d "bin/$csbuildmodedir/$framework" ];
then
   cp target/$rsbuildmodedir/*.so bin/$csbuildmodedir/$framework
   echo    ok.
   dotnet run
else
   mkdir -p bin/$csbuildmodedir/$framework
   cp target/$rsbuildmodedir/*.so bin/$csbuildmodedir/$framework
   echo    ok.
   dotnet run
fi