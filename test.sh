#!/bin/sh

echo "WORDSEG"
curl -d '{"text":"กากกา"}' http://localhost:3000/wordseg
echo
echo
echo "DAG"
curl -d '{"text":"กากกา"}' http://localhost:3000/dag
echo
echo
echo "DAG COMPLEX"
curl -d '{"text":"รอบอก"}' http://localhost:3000/dag
echo
echo
echo "Invalid"
curl -v -d '{"textx":"กากกา"}' http://localhost:3000/dag
