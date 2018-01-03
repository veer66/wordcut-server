#!/bin/sh

echo "WORDSEG"
curl -d '{"text":"กากกา"}' http://localhost:3000/wordseg
echo
echo "DAG"
curl -d '{"text":"กากกา"}' http://localhost:3000/dag
echo
