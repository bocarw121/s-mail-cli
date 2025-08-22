# S-Mail Cli

Send emails with attachments and body content from the command line using your gmail account
More accounts support coming soon

---

### Usage

#### Instructions on how to get your Gmail app password

Run the following command to get instructions on how to get your Gmail app password

```bash
 smail instructions
```

#### Store your credentials

```bash
$ smail credentials -p smtp.gmail.com  -e myemail@gmail.com -c super-secret-password  # stores your credentials for future use this is encrypted
```

#### Send an email with a subject and file attachment

```bash
$ smail send -t recipient-email@gmail.com -s "Here is the pdf file" -a ./filename.pdf # send an email with a subject and an attachment
```

#### Send an email with a body

```bash
$ smail send -t recipient-email@gmail.com -s "Hello there" -b "This is the content of my email" # send an email with a subject and body
```

#### Send an email with both body and attachment

```bash
$ smail send -t recipient-email@gmail.com -s "Monthly Report" -b "Please find attached the monthly report" -a ./report.pdf # send an email with a subject, body, and attachment
```

#### List all credentials

```bash
smail list
# list all credentials ie
{
    "provider": "smtp.gmail.com",
    # Hashed password
    "password": "d29lZHJteGNxcHh3cmFyaA==",
    "email": "myemail@gmail.com",
}
```


#### Commands

```bash
   Cli tool to easily send emails with attachments

Usage: smail <COMMAND>

Commands:
 instructions  Get instructions on how to set up your credentials
 credentials   Store your email, password, and provider
 send          Send an email
 list          Get a list of all the credentials
 help          Print this message or the help of the given subcommand(s)

Options:
 -h, --help     Print help
 -V, --version  Print version
```

---

### Configuration Storage

- **JSON Configuration**: Your email credentials are stored securely in a `smail.json` file in your home directory.
- **Cross-platform**: Works on Windows, Linux, and macOS without any external dependencies.
- **Location**: 
  - Windows: `C:\Users\username\smail.json`
  - Linux: `/home/username/smail.json`
  - macOS: `/Users/username/smail.json`

#### Notes:
- **Security**: The email and provider are stored in plain text, while the password is encrypted before being saved.
- **Portability**: The configuration file can be easily backed up or transferred between systems.

---

### TODO

- [ ] Add support for more email providers
- [ ] Add support for sending emails with multiple attachments
- [x] Add support for sending emails with body text
- [ ] Add tests
- [ ] refactor code to make it more modular


