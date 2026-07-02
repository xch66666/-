#!/bin/bash
# 每完成一个阶段，自动 commit + push
# 用法: ./auto-push.sh "描述信息"

set -e

MSG="$1"

if [ -z "$MSG" ]; then
  echo "用法: ./auto-push.sh \"提交信息\""
  exit 1
fi

cd "$(dirname "$0")"

# 检查是否有变更
if git diff --quiet && git diff --cached --quiet && [ -z "$(git ls-files --others --exclude-standard)" ]; then
  echo "[auto-push] 没有文件变更，跳过"
  exit 0
fi

git add -A
git commit -m "$MSG"
git push origin main

echo "[auto-push] ✅ 已推送: $MSG"
