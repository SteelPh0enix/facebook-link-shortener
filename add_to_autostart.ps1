$autostart_dir = [System.Environment]::GetFolderPath('Startup')
Copy-Item '.\target\release\fbclid-remover.exe' -Destination (Join-Path -Path $autostart_dir -ChildPath 'fbclid-remover.exe')