#!/usr/bin/env bash
export PORT=4000
BASE_PATH=$(pwd)
BIN_FILENAME=$1
LOG_FILENAME="$BIN_FILENAME.$(date -u +%Y-%m-%dT%H.%M.%SZ).log"
BIN_PATH="$BASE_PATH/$BIN_FILENAME"
#LOG_PATH="$BASE_PATH/$LOG_FILENAME"
LOG_PATH="/dev/null"
CMD="$BIN_PATH"
KILL_CMD="pidof '$CMD' | xargs kill -9"
bash -c "chmod +x $1"
bash -c "$KILL_CMD"
bash -c "nohup $CMD > $LOG_PATH 2>&1 &"