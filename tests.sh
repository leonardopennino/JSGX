#!/bin/zsh
curl -k -X POST http://localhost:3000/AddUser -d '[{"username" : "curl" , "id" : "test"}]' -H 'Content-Type: application/json' ; echo
curl -k -X POST http://localhost:3000/IncreaseWithNumber -d '[10]' -H 'Content-Type: application/json' ; echo
curl -k -X POST http://localhost:3000/IncreaseWithArray -d '[[ 1, 2, 3]]' -H 'Content-Type: application/json' ; echo
curl -k -X POST http://localhost:3000/Show -d '[]' -H 'Content-Type: application/json' ; echo # should show 16
curl -k -X POST http://localhost:3000/MultipleParams -d '[ 1, "2", 3.0]' -H 'Content-Type: application/json' ; echo
