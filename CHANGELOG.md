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

## 0.4.0

- **BREAKING CHANGE**: Removed Redis dependency 
- Replaced Redis storage with JSON file-based configuration
- Configuration now stored in `smail.json` in user's home directory
- Cross-platform support for Windows, Linux, and macOS
- No external dependencies required - Redis server no longer needed
- Existing credentials will need to be reconfigured due to storage change