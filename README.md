# S-Mail Cli

Send emails with attachments from the command line using your gmail account
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

#### Send an email with a subject file attachment

```bash
$ smail email -t recipient-email@gmail.com -s "Here is the pdf file" -a ./filename.pdf # send an email with a subject and an attachment
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
 email         Send an email
 list          Get a list of all the credentials
 help          Print this message or the help of the given subcommand(s)

Options:
 -h, --help     Print help
 -V, --version  Print version
```
