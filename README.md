# wordcut-server

wordcut-server is a HTTP server for wordcut-engine.

## Run

````
cargo run
````

## Test

````
curl http://localhost:3000/wordseg -d '{"text":"กากาก"}'
````

## Windows service

### Creating service example

````
PS> New-Service -Name wcsrv -BinaryPathName "D:\Develop\wordcut-server\target\debug\wordcut-server.exe D:\Develop\wordcut-server\config"
````