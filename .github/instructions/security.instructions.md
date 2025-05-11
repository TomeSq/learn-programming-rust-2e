---
applyTo: "**"
---

## セキュリティ対策

- 安全なコーディングプラクティスを遵守。
- ユーザー入力の適切なバリデーションとサニタイズ。
- 機密情報の暗号化と安全な保管。
- 最小権限の原則を適用し、コンポーネントの権限を最小限に。

## Sensitive Files

DO NOT read or modify:

- .env files
- \*_/config/secrets._
- \*_/_.pem
- Any file containing API keys, tokens, or credentials

## Security Practices

- Never commit sensitive files
- Use environment variables for secrets
- Keep credentials out of logs and output
