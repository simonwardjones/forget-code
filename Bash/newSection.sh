#!/bin/sh

# Author : Simon Ward-Jones

# The argument provided is the section we wish to Initialise


if [ $# -eq 0 ]
  then
    echo "No arguments supplied \nPlease provide section name e.g. \"Python\""
    exit 1
fi

mkdir $1
echo "# learn$1" > $1/Learn$1.md

header="
<sub>[Back to ForgetCode](../README.md)</sub>
<!-- MarkdownTOC autolink=\"true\" bracket=\"round\" indent=\"    \"-->

- [General](#general)

<!-- /MarkdownTOC -->

# General
"
echo "$header"
echo "$header" >>  $1/Learn$1.md