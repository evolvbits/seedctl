#!/bin/bash

BIN="/usr/bin/ollama"
INSTRUCTION="IA.txt"
# RECOMMENDED MODELS:
# - qwen2.5-coder:1.5b
# - qwen2.5-coder:7b
# - deepseek-coder:6.7b
# - llama3
# - qwen2.5-coder:14b-instruct-q4_K_M
MODEL="deepseek-coder:6.7b"

if [[ -f "$INSTRUCTION" ]] && [[ -f "$BIN" ]]; then
	"$BIN" pull "$MODEL"
	cat "$INSTRUCTION" | "$BIN" run "$MODEL"
fi
