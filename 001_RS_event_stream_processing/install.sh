#!/bin/bash
# ------------------------------------------------------------------
# This script is derived from the following stackoverflow post:
# https://stackoverflow.com/questions/14008125/shell-script-common-template
# ------------------------------------------------------------------
# [Juweins] Install Script
#           Install exchange binary on your local machine
# ------------------------------------------------------------------

VERSION=0.1.0
SUBJECT=some-unique-id
USAGE="Usage: command -ihv args"

# --- Options processing -------------------------------------------
if [ $# == 0 ] ; then
    echo $USAGE
    exit 1;
fi

while getopts ":l:vh" optname
  do
    case "$optname" in
      "v")
        echo "Version $VERSION"
        exit 0;
        ;;
      "l")
        echo "$OPTARG"
        ;;
      "h")
        echo $USAGE
        exit 0;
        ;;
      "?")
        echo "Unknown option $OPTARG"
        exit 0;
        ;;
      ":")
        echo "No argument value for option $OPTARG"
        exit 0;
        ;;
      *)
        echo "Unknown error while processing options"
        exit 0;
        ;;
    esac
  done

shift $(($OPTIND - 1))

param1=$1
param2=$2

# --- Locks -------------------------------------------------------
LOCK_FILE=/tmp/$SUBJECT.lock
if [ -f "$LOCK_FILE" ]; then
   echo "Script is already running"
   exit
fi

trap "rm -f $LOCK_FILE" EXIT
touch $LOCK_FILE

# --- Body --------------------------------------------------------
#  SCRIPT LOGIC GOES HERE
# User input question
echo "Exchange"

# multi line comment
cat << "EOF"
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ______          _                            
   |  ____|        | |                           
   | |__  __  _____| |__   __ _ _ __   __ _  ___ 
   |  __| \ \/ / __| '_ \ / _` | '_ \ / _` |/ _ \
   | |____ >  < (__| | | | (_| | | | | (_| |  __/
   |______/_/\_\___|_| |_|\__,_|_| |_|\__, |\___|
                                       __/ |     
                                      |___/ 
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
EOF

echo "Enter the path you want to install exhange: "
read path
echo "Enter the IP of your local kafka installation (default: localhost:9092): "
read kafka_ip
echo "Installing exchange to $path..."
# -----------------------------------------------------------------