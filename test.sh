#!/bin/sh
PREFIX=http://localhost:3134
OPT='-v'
echo "WORDSEG"
curl $OPT -d '{"text":"กากกา"}' $PREFIX/wordseg
echo
echo
echo "DAG"
curl $OPT -d '{"text":"กากกา"}' $PREFIX/dag
echo
echo
echo "DAG COMPLEX"
curl $OPT -d '{"text":"รอบอก"}' $PREFIX/dag
echo
echo
echo "Invalid"
curl $OPT -v -d '{"textx":"กากกา"}' $PREFIX/dag
