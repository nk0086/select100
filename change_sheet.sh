#!/bin/zsh
#modify sheet.md to make easier use.
FILE_NAME=sheet.md
while read line
do
    NUM1=${line:1:1}
    NUM2=${line:2:1}
    NUM3=${line:3:1}
    #echo $line >> e-sheet.md
    if ! [[ $NUM1 =~ [0-9] ]];
    then
	echo ${line}>> e-sheet.md
	continue
    fi

    if ! [[ $NUM2 =~ [0-9] ]];
    then
	echo "${line:0:-1}\$00$NUM1|" >> e-sheet.md
	continue
    fi

    if ! [[ $NUM3 =~ [0-9] ]];
    then
	echo "${line:0:-1}\$0$NUM1$NUM2|" >> e-sheet.md
	continue
    fi

    echo "${line:0:-1}\$100|" >> e-sheet.md


done < $FILE_NAME
