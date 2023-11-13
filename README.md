# FastMail CLI client

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

# Encrypted storage

All the data that received from the FastMail server are securely stored on the local machine and use AES-256 encryption to encrypt the database. That key
is stored in MacOS KeyChain (Software mode) as a User Password. Access to that password is maintained by the KeyChain and other apps cannot read it without an
approval from the User. 

## AES Key and FastMail API token

FastMail service API is required to work with the FastMail database. That password is securely stored in Apple KeyChain and not visible for other apps. It is 
never sent to other services.

AES key is generated from the user password and FastMail token. So, when you create a new database then you need to create a new strong database password and 
provide FastMail API token. Argon2 algorithm is used to generate secure AES-256 key from user password and fastmail api.

## Store the database on iCloud

Database can be safely stored in the iCloud or other distributed file system. To read the database created on another machine you need to provide the same
password and API token you used to encrypt the database. Then on a new machine the same AES key will be derived that is then can decrypt the database.

## Password change

Whenever you want to update the password or API token then the new AES key will be generated and the database will be encrypted using the new key. On another
machine the system will fail to decrypt the database and you need to update the API token and Database password to generate a new AES key that then will be
the same as the database used to encrypt. Don't share API token and Database Password with someone else to prevent AES key recreating and your database decryption.

### Application artifact signature and KeyChain

Access to the password is provided by default only to the app that that password is created. After you update the app for the MacOS it will be another artifact
that has no access. In that case when you see the MacOS Window to enter the system password to provide an access you need to click "Always Allow". Otherwise the 
password will be required to enter each time (the system password and not the FastMail token).
