#!/bin/bash

fullpath=`realpath $0`
currentdir=`dirname ${fullpath}`
logsdir=${currentdir}/../log

if [[ $# -lt 2 ]]
then
  echo "Usage: ./filterLogs.sh <game ID> <turn #>"
  exit 1
fi

game=$1
turn=$2

grep "\[.*${game}.* ${turn}\]" ${logsdir}/*.log \
  | sed 's/^.* - \[[^]]*\] //'
