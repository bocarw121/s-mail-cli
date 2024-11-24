pub const INSTRUCTIONS_MESSAGE: &str = "Instructions on how to get your Gmail app password
  1. Go to [Google Account](https://myaccount.google.com/) and click on the avatar on the top right corner and click on `Manage your Google Account`
  2. Find the Security section on the left side
  3. In the How you sign in to Google section, Enable 2-Step Verification if not already enabled
  4. Then got to https://myaccount.google.com/apppasswords
  5. Add your application name and click on the create button.
  6. Copy the generated password.
  7. Now your ready to add your credentials run the command smail credentials -e <your email> -c <your password> -p <your provider> ie smtp.gmail.com and you're all set!
  ";