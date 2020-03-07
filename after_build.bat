mkdir dist
cd dist
mkdir bin share lib
mkdir share\glib-2.0 share\icons lib\gdk-pixbuf-2.0
xcopy .\..\target\release\open2internet.exe .\bin 1> nul
for %%a in (C:\msys64\mingw64\bin\libfribidi-0.dll,C:\msys64\mingw64\bin\libatk-1.0-0.dll,C:\msys64\mingw64\bin\libbz2-1.dll,C:\msys64\mingw64\bin\libcairo-2.dll,C:\msys64\mingw64\bin\libcairo-gobject-2.dll,C:\msys64\mingw64\bin\libepoxy-0.dll,C:\msys64\mingw64\bin\libexpat-1.dll,C:\msys64\mingw64\bin\libffi-6.dll,C:\msys64\mingw64\bin\libfontconfig-1.dll,C:\msys64\mingw64\bin\libfreetype-6.dll,C:\msys64\mingw64\bin\libgcc_s_seh-1.dll,C:\msys64\mingw64\bin\libgdk-3-0.dll,C:\msys64\mingw64\bin\libgdk_pixbuf-2.0-0.dll,C:\msys64\mingw64\bin\libgio-2.0-0.dll,C:\msys64\mingw64\bin\libglib-2.0-0.dll,C:\msys64\mingw64\bin\libgmodule-2.0-0.dll,C:\msys64\mingw64\bin\libgobject-2.0-0.dll,C:\msys64\mingw64\bin\libgraphite2.dll,C:\msys64\mingw64\bin\libgtk-3-0.dll,C:\msys64\mingw64\bin\libharfbuzz-0.dll,C:\msys64\mingw64\bin\libiconv-2.dll,C:\msys64\mingw64\bin\libintl-8.dll,C:\msys64\mingw64\bin\libpango-1.0-0.dll,C:\msys64\mingw64\bin\libpangocairo-1.0-0.dll,C:\msys64\mingw64\bin\libpangoft2-1.0-0.dll,C:\msys64\mingw64\bin\libpangowin32-1.0-0.dll,C:\msys64\mingw64\bin\libpcre-1.dll,C:\msys64\mingw64\bin\libpixman-1-0.dll,C:\msys64\mingw64\bin\libpng16-16.dll,C:\msys64\mingw64\bin\libstdc++-6.dll,C:\msys64\mingw64\bin\libwinpthread-1.dll,C:\msys64\mingw64\bin\zlib1.dll,C:\msys64\mingw64\bin\libthai-0.dll,C:\msys64\mingw64\bin\libdatrie-1.dll) do xcopy %%a .\bin 1> nul
xcopy C:\msys64\mingw64\share\glib-2.0 .\share\glib-2.0 /E 1> nul
xcopy C:\msys64\mingw64\share\icons .\share\icons /E 1> nul
xcopy C:\msys64\mingw64\lib\gdk-pixbuf-2.0 .\lib\gdk-pixbuf-2.0 /E 1> nul
cd ..
7z a open2internet-mingw64.7z dist\*