# FastMail client to make a full backup of emails and store them securely

That small tool helps to store locally all the information about the generated Masked Email. Also it shows more details that UI provides.

Example of metadata information that is received and stored in an encrypted storage.

```json
{
 "createdAt": "2023-11-12 05:57:39 GMT",
 "url": null,
 "email": "mysuperemail@mysuperdomain.xyz",
 "description": "Email created for the site Example.com",
 "lastMessageAt": "2023-11-12 05:58:05 GMT",
 "createdBy": "FastMail CLI",
 "id": "masked-ABC",
 "forDomain": "https://example.com/",
 "state": "enabled"
}
```

## Installation

Run `cargo install --path .` to build and install the app.

## Config

Configuration is stored in the local app directory (for instance, on MacOS it will be `~/Library/Application Support/maskedemail-cli.toml`). The format is simple, just provide 2 parameters:
```toml
user_name = "<fastmail_account_full_email_address>"
storage = "/tmp/masked-email-db"
```

## Usage

First you need to init new database. Then call refresh-db to fetch the current emails.

```text
Usage: masked-email-cli [COMMAND]

Commands:
  init             Create or update the program configuration
  update-password  Store new fastmail password. The old record might be deleted
  refresh-db       Download the whole emails list and update the database
  export           Export all email aliases
  show             Show all email aliases
  help             Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

Export command supports exporting using lua script so you can use to transform to any desired format. See an example in `./lua/tsv.lua`.
(the command to run would be `masked-email-cli export-lua -p ./lua/tsv.lua`).

Command `show` helps to quickly search database to find by email / description / domain. It prints all the details stored in database.

## Encrypted storage

All the data that received from the FastMail server are securely stored on the local machine and use AES-256 encryption to encrypt the database. That key
is stored in MacOS KeyChain (Software mode) as a User Password. Access to that password is maintained by the KeyChain and other apps cannot read it without an
approval from the User. That key is not marked as syncronized, so it won't be copied to iCloud.

### File format specification

Header part are not encrypted and loaded whenever you try to open the database:

- file signature 4 bytes
- AES key nonce 12 bytes
- last updated TS (8 bytes)
- records count (4 bytes)

Later file part is related to the encrypted blocks:

- unique nonce 12 bytes
- tag 16 bytes (from last updated + records count)
- total encrypted block bytes length (8 bytes)
- encrypted block (see the size above)

Unique nonce every time is generated when the database is refreshed. That prevents from comparring history to identify is the database has changes.

### AES Key and FastMail API token

FastMail service API is required to work with the FastMail database. That password is securely stored in Apple KeyChain and not visible for other apps. It is
never sent to other services.

AES key is generated from the FastMail token and unique salt generated for all new databases. So, when you create a new database then you need to  
provide FastMail API token only. Argon2 algorithm is used to generate secure AES-256 key from the fastmail token and newly generated salt. That salt is stored
in the database. That means if you copy that database to a new machine then the AES-256 key will be derived providing you a chance to decrypt the database. But
noone else can decrypt the database without knowing your fastmail api token.

### Store the database on iCloud

Database can be safely stored in the iCloud or other distributed file system. To read the database created on another machine you need to provide the same API token
you used to encrypt the database.

## FastMail API Password change

Whenever you want to update the API token then the new AES key will be generated and the database will be encrypted using the new key. On another
machine the system will fail to decrypt the database and you need to update the API token to generate a new AES key that then will be
the same as the database used to encrypt. Don't share API token with someone else to prevent AES key recreating and your database decryption.

### Application artifact signature and KeyChain

Access to the password is provided by default only to the app that that password is created. After you update the app for the MacOS it will be another artifact
that has no access. In that case when you see the MacOS Window to enter the system password to provide an access you need to click "Always Allow". Otherwise the
password will be required to enter each time (the system password and not the FastMail token). Updating lua script does not affect the security system so you will
see the password asking only first time you build and install the app.
