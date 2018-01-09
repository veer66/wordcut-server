#!/bin/sh
PREFIX=http://localhost:3134
echo "WORDSEG"
curl -d '{"text":"กากกา"}' $PREFIX/wordseg
echo
echo
echo "DAG"
curl -d '{"text":"กากกา"}' $PREFIX/dag
echo
echo
echo "DAG COMPLEX"
curl -d '{"text":"รอบอก"}' $PREFIX/dag
echo
echo
echo "Invalid"
curl -v -d '{"textx":"กากกา"}' $PREFIX/dag
