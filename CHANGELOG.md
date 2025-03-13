# CHANGELOG

## 0.2.0

- Removed the `from_email` flag from the `send` command. the
  email is now being fetched dynamically.
- Changed the `email` command to `send`.

## 0.3.0

- Added the ability to include a body in emails via the `--body` flag
- Improved email handling to support sending:
  - Email with just a body
  - Email with just an attachment
  - Email with both body and attachment