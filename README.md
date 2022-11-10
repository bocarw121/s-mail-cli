# S-Mail Cli 

 Send emails with attachments from the command line


--------------------
### Usage

#### Instructions on how to get your Gmail app password
1. Go to [Google Account](https://myaccount.google.com/) and click on the avatar on the top right corner and click on `Manage your Google Account`
2. Find the Security section on the left side
3. In the Signing in to Google section, Enable 2-Step Verification if not already enabled
4. In the same section, click on App passwords
5. Select `Mail` as the app and `Other` as the device and give it a name
6. Click on Generate and copy the password


#### Store your credentials 
```bash
$ smail credentials -p smtp.gmail.com  -e myemail@gmail.com -c super-secret-password  # stores your credentials for future use 
```

#### Send an email with a subject file attachment

```bash
$ smail email -t recipient-email@gmail.com -s "Here is the pdf file" -a ./filename.pdf # send an email with a subject and an attachment
```

#### List all credentials

```bash
$ smail list # list all credentials ie {
    "provider": "smtp.gmail.com",
    # Hashed password 
    "password": "d29lZHJteGNxcHh3cmFyaA==",
    "email": "myemail@gmail.com",
}
```

#### For more help

```bash
$ smail --help
```


