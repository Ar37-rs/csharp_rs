export rsbuildmodedir=release
export csbuildmodedir=Release
# if net5.0
# export framework=net5.0
export framework=netcoreapp3.1

cargo build --release
if [ -d "bin/$csbuildmodedir/$framework" ];
then
   cp target/$rsbuildmodedir/*.so bin/$csbuildmodedir/$framework
   echo    ok.
   dotnet build -c $csbuildmodedir
else
   mkdir -p "bin/$csbuildmodedir/$framework"
   cp target/$rsbuildmodedir/*.so bin/$csbuildmodedir/$framework
   echo    ok.
   dotnet build -c $csbuildmodedir
fi