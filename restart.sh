#!/usr/bin/env bash
export PORT=4000
BASE_PATH=$(pwd)
BIN_FILENAME=$1
LOG_FILENAME="$BIN_FILENAME.$(date -u +%Y-%m-%dT%H.%M.%SZ).log"
BIN_PATH="$BASE_PATH/$BIN_FILENAME"
LOG_PATH="$BASE_PATH/$LOG_FILENAME"
CMD="$BIN_PATH"
KILL_CMD="ps aux | grep '$CMD' | grep -v grep | awk '{print \$2}' | xargs kill -9"
bash -c "$KILL_CMD"
bash -c "nohup $CMD > $LOG_PATH 2>&1 &"