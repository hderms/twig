# Description
Simple(not production-ready) autosuggestion service based on a Trie. 

# Usage

`POST /insert {"string" : "foo"}`

`POST /insert {"string" : "foobar"}`

`POST /insert {"string" : "foobaz"}`

and then to get suggestions:

`GET /suggestions?string=foo&limit=3`


