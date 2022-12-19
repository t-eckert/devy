#!/bin/bash

curl -X POST "localhost:3000/api/uploads" \
	-H 'Content-Type: application/json' \
	-d '{"url":"https://github.com/t-eckert/blog"}'
