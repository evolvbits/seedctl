#!/bin/bash
# Agent AI: by William C. Canin
#
# RAPID TEST:
# cat ${ROOT}/prompt.txt | $BIN run $MODEL
#
# PREPARE:
# >>> sudo pacman -S ollama vim
# >>> sudo systemctl start ollama
# >>> ollama pull <MODEL>
#
# RECOMMENDED MODELS:
# - qwen2.5-coder:1.5b
# - qwen2.5-coder:7b
# - deepseek-coder:6.7b
# - llama3
# - qwen2.5-coder:14b-instruct-q4_K_M
#
#
MODEL="qwen2.5-coder:1.5b"
BIN="/usr/bin/ollama"
EDITOR="/usr/bin/vim"
ROOT=".ia"
PROMPT="${ROOT}/prompt.txt"
MEMORY="${ROOT}/memory.txt"
SUMMARY="${ROOT}/summary.txt"
CONTEXT="${ROOT}/context.txt"
TMP="${ROOT}/fullprompt.txt"
RESPONSE="${ROOT}/response.txt"

[ ! -d ${ROOT} ] && mkdir -p ${ROOT}

for f in ${PROMPT} ${MEMORY} ${SUMMARY} ${CONTEXT} ${TMP} ${RESPONSE}
do
  [ -f "$f" ] || touch "$f"
done

if [ ! -s "${CONTEXT}" ]; then
  echo "The file ${CONTEXT} is empty. Opening it in vim..."
  $EDITOR +startinsert "${CONTEXT}"
fi

if [ ! -s "${PROMPT}" ]; then
  echo "The file ${PROMPT} is empty. Opening it in vim..."
  $EDITOR +startinsert "${PROMPT}"
fi

echo "=== OLLAMA LOCAL AI ==="
echo

# Monta prompt completo
cat "$CONTEXT" > "$TMP"

echo >> "$TMP"
echo "=== SUMMARY MEMORY ===" >> "$TMP"
cat "$SUMMARY" >> "$TMP"

echo >> "$TMP"
echo "=== RECENT MEMORY ===" >> "$TMP"

tail -n 200 "$MEMORY" >> "$TMP"

echo >> "$TMP"
echo "=== NEW REQUEST ===" >> "$TMP"

cat "$PROMPT" >> "$TMP"

echo >> "$TMP"
echo "=== RESPONSE ===" >> "$TMP"

echo "Running model..."
echo

$BIN run "$MODEL" < "$TMP" | tee "$RESPONSE"

echo
echo "Saving memory..."

echo "USER:" >> "$MEMORY"
cat "$PROMPT" >> "$MEMORY"

echo >> "$MEMORY"

echo "ASSISTANT:" >> "$MEMORY"
cat "$RESPONSE" >> "$MEMORY"

echo >> "$MEMORY"
echo "------------------------" >> "$MEMORY"
echo >> "$MEMORY"


# Se memória crescer muito → resumir
LINES=$(wc -l < "$MEMORY")

if [ "$LINES" -gt 800 ]; then

  echo
  echo "Summarizing memory..."

  cat <<EOF > ${ROOT}/summarize.txt
Summarize the following conversation memory.

Keep:

- important decisions
- code fixes
- technical conclusions

Memory:

EOF

    tail -n 600 "$MEMORY" >> ${ROOT}/summarize.txt

    $BIN run "$MODEL" < ${ROOT}/summarize.txt > "$SUMMARY"

    tail -n 200 "$MEMORY" > ${ROOT}/memory_trimmed.txt
    mv ${ROOT}/memory_trimmed.txt "$MEMORY"

fi

echo
printf "" > $PROMPT
echo "Done."
