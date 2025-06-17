# Masked Email CLI - FastMail Client for Masked Email Management

A Rust-based CLI tool that helps to securely store and manage all information about your FastMail masked emails. This tool provides more detailed information than the FastMail UI and allows for flexible data export through Lua scripting.

## Overview

For a detailed view of the project's code organization, see the [Project Structure](docs/PROJECT_STRUCTURE.md).

This application allows you to:
- Securely store your FastMail masked email data locally
- View detailed metadata about your masked emails
- Search through your masked emails by email address, description, or domain
- Export your masked email data in various formats using Lua scripts
- Maintain an encrypted local database of your masked emails

Example of metadata information that is received and stored in the encrypted storage:

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

## Requirements

- Rust 2021 edition (see rust-toolchain file)
- MacOS (for KeyChain integration)
- FastMail account with API access

## Installation

1. Clone the repository
2. Build and install the application:
   ```bash
   cargo install --path .
   ```

This will compile the application and install it to your system.

## Config

Configuration is stored in the local app directory (for instance, on MacOS it will be `~/Library/Application Support/maskedemail-cli.toml`). The format is simple, just provide 2 parameters:
```toml
user_name = "<fastmail_account_full_email_address>"
storage = "/tmp/masked-email-db"
```

Alternatively, you can use environment variables to override these settings. The application will check for these environment variables as a fallback if the config file is not available or if you want to override specific settings:

```bash
# Override the FastMail username
APP_USER_NAME=your.email@fastmail.com

# Override the storage location
APP_STORAGE=/path/to/your/database
```

## Usage

### Getting Started

1. Initialize a new database configuration:
   ```bash
   masked-email-cli init
   ```
   You'll be prompted to enter your FastMail email address and the location for your database file. This creates a configuration file in your system's application support directory.

2. Set your FastMail API token:
   ```bash
   masked-email-cli update-password
   ```
   You'll be prompted to enter your FastMail API token. This token is securely stored in your system's keychain and is used to authenticate with the FastMail API.

   **Note:** To generate a FastMail API token:
   - Log in to your FastMail account
   - Go to Settings > Password & Security > App Passwords
   - Create a new app-specific password with access to your masked email data
   - Copy the generated token for use with this application

3. Download your masked emails:
   ```bash
   masked-email-cli refresh-db
   ```
   This command will fetch all your masked emails from FastMail and store them in your encrypted local database.

### Command Reference

```text
Usage: masked-email-cli [COMMAND]

Commands:
  init             Create or update the program configuration
  update-password  Store new fastmail password. The old record might be deleted
  refresh-db       Download the whole emails list and update the database
  export-lua       Export all email aliases using provided lua script
  show             Show all email aliases
  help             Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Searching Emails

The `show` command helps you quickly search the database to find emails by address, description, or domain:
```bash
masked-email-cli show
```
This will open an interactive search interface where you can filter and view detailed information about your masked emails.

### Exporting Data with Lua Scripts

The application supports exporting your masked email data using Lua scripts, allowing you to transform the data into any desired format:

```bash
masked-email-cli export-lua -p ./lua/tsv.lua
```

For example, this will export your masked emails in a tab-separated values format:

```
email                   description             web_site               state     created_at
----------------------- ----------------------- ---------------------- --------- -----------------------
myemail1@mydomain.xyz   Email for example.com   https://example.com    enabled   2023-11-12 05:57:39 GMT
```

See the [Lua Scripting](#lua-scripting) section below for details on all available script formats.

## Encrypted storage

All data received from the FastMail server is securely stored on the local machine using AES-256 encryption. The encryption key is stored in MacOS KeyChain (Software mode) as a User Password. Access to this key is managed by the KeyChain, and other applications cannot read it without explicit approval from the User. The key is not marked for synchronization, so it won't be copied to iCloud.

### File format specification

The database file uses a binary format with a clear separation between the unencrypted header and the encrypted body:

#### Unencrypted Header (28 bytes total)
The header is not encrypted and is loaded whenever you try to open the database:

- **File signature** (4 bytes): `[b'M', b'E', b'F', 1u8]` - Identifies the file as a Masked Email Format file with version 1
- **AES key derivation salt** (12 bytes): Used with the FastMail API token to derive the AES-256 encryption key via Argon2
- **Last updated timestamp** (8 bytes): UTC timestamp of when the database was last modified
- **Records count** (4 bytes): Number of masked email records stored in the database

#### Encrypted Body
The body section contains the actual masked email data and is fully encrypted:

- **Unique encryption nonce** (12 bytes): Randomly generated each time the database is refreshed
- **Authentication tag** (16 bytes): Generated during encryption to verify data integrity, derived from the last updated timestamp and records count as associated data
- **Encrypted block size** (8 bytes): Total length of the encrypted data block in bytes
- **Encrypted block** (variable size): The serialized and encrypted masked email records

A new unique nonce is generated each time the database is refreshed. This prevents comparing file history to identify whether the database has changed, enhancing privacy and security.

#### Encryption Process
1. The masked email records are serialized to binary format
2. The serialized data is encrypted using AES-256-GCM with the derived key and a fresh nonce
3. The encryption produces a tag that is stored alongside the encrypted data for integrity verification
4. During decryption, the tag is used to verify the data hasn't been tampered with

### AES Key and FastMail API token

FastMail service API is required to work with the FastMail database. That password is securely stored in Apple KeyChain and not visible for other apps. It is
never sent to other services.

AES key is generated from the FastMail token and unique salt generated for all new databases. So, when you create a new database then you need to
provide FastMail API token only. Argon2 algorithm is used to generate secure AES-256 key from the fastmail token and newly generated salt. That salt is stored
in the database. That means if you copy that database to a new machine then the AES-256 key will be derived providing you a chance to decrypt the database. But
noone else can decrypt the database without knowing your fastmail api token.

### Store the database on iCloud

The database can be safely stored on iCloud, Dropbox, Google Drive, or any other public cloud storage service without security concerns. This is possible due to the robust encryption architecture:

1. **Strong AES-256-GCM Encryption**: All sensitive data is encrypted with AES-256-GCM, a military-grade encryption standard that is practically impossible to break with current technology.

2. **Key Derivation Security**: The encryption key is never stored directly in the database. Instead, it's derived from your FastMail API token using the Argon2 algorithm and a unique salt stored in the database header.

3. **Two-Factor Security Model**: To decrypt the database, an attacker would need both:
   - Physical access to your database file
   - Your personal FastMail API token (which is never stored in the database)

4. **Tamper-Proof Design**: The AES-GCM authentication tag ensures that any modification to the encrypted data will be detected during decryption, preventing tampering attacks.

To read the database created on another machine, you simply need to provide the same FastMail API token that was used to encrypt it. This design allows for convenient synchronization across devices while maintaining strong security guarantees.

## FastMail API Password change

Whenever you want to update the API token then the new AES key will be generated and the database will be encrypted using the new key. On another
machine the system will fail to decrypt the database. In this case, you'll need to update the API token to generate a new AES key that will be
the same as the database used to encrypt. Do not share your API token with anyone else to prevent unauthorized AES key recreation and potential database decryption.

## FastMail API Token Management

The application uses your FastMail API token to authenticate with the FastMail service and access your masked email data. This token is also used in the encryption process to derive the AES key for your database.

### Updating Your FastMail API Token

If you need to update your FastMail API token (for example, if you've revoked the previous token or it has expired), use the following command:

```bash
masked-email-cli update-password
```

This will:
1. Prompt you to enter your new FastMail API token
2. Securely store the new token in your system's keychain
3. Generate a new AES key for database encryption on the next refresh

**Important:** If you change your FastMail API token and have an existing database, you'll need to run `refresh-db` afterward to ensure the database can be decrypted with the new token-derived key.

### API Token Security

Your FastMail API token is:
- Never stored in plain text
- Securely stored in your system's keychain
- Used to derive the encryption key for your database
- Never transmitted except to authenticate with FastMail

### Application Signature and KeyChain Access

MacOS KeyChain restricts password access to the specific application binary that created it. When updating the application:

1. The new binary will have a different signature and initially lacks KeyChain access
2. When prompted for system password, select "Always Allow" to grant persistent access
3. Without this permission, you'll need to enter your system password (not FastMail token) each time

Note: Modifying Lua scripts does not affect application signatures or KeyChain permissions.

## Architecture

The application is built with the following components:

- **CLI Interface**: Built with `clap` for command-line argument parsing
- **Configuration Management**: Uses `config` crate to manage user configuration
- **Secure Storage**: 
  - Uses MacOS KeyChain for secure storage of FastMail API tokens
  - Implements AES-256 encryption for the local database
- **FastMail API Integration**: Communicates with FastMail's API to retrieve masked email data
- **Lua Scripting Engine**: Uses `mlua` to provide a flexible data export system
- **Interactive Search**: Uses `skim` for fuzzy searching through the database

## Key Files

### Lua Script Interpreters (Export)
- `src/actions/export.rs` - Handles exporting masked emails using Lua scripts for custom formatting
- Example scripts are available in the `lua` directory (tsv.lua, json.lua, xml.lua, email_only.lua)

### Encryption
- `src/secrets/encryption.rs` - Implements AES encryption utilities for secure data storage
- `src/db/disk.rs` - Handles encrypted database operations for storing masked emails

### API Sessions
- `src/fastmail/json/session.rs` - Contains JSON structures for FastMail session API
- `src/fastmail/json/method_response.rs` - Handles JSON structures for FastMail JMAP method responses
- `src/fastmail.rs` - Implements the FastMail API client for retrieving masked emails

### Keychain for Storing Credentials
- `src/secrets/keychain.rs` - Provides secure storage using system keychain for passwords and keys
- `src/secrets/fastmail.rs` - Manages FastMail account credentials
- `src/secrets.rs` - Implements secure data types with memory zeroing for passwords and keys

## Lua Scripting

The application supports custom Lua scripts for exporting data. For best practices and detailed guidelines on writing Lua scripts for this project, see the [Lua Best Practices](docs/LUA_BEST_PRACTICES.md) document.

Lua scripts should implement the following functions:

- `prepare(table)`: Called first, allows you to set up any necessary state
- `header()`: Called before processing records, should return header text if needed
- `next(record)`: Called for each record, should return formatted output for that record
- `footer()`: Called after all records are processed, should return footer text if needed

Each record passed to the `next` function contains the following fields:
- `email`: The masked email address
- `description`: Description of the email
- `web_site`: Associated website/domain
- `state`: Current state (enabled/disabled)
- `created_at`: Creation timestamp

### Available Lua Scripts

The following Lua scripts are provided in the `lua` directory:

#### 1. `email_only.lua`

Outputs only the email addresses of enabled masked emails, one per line.

**Example output:**
```
myemail1@mydomain.xyz
myemail2@mydomain.xyz
```

#### 2. `json.lua`

Exports all masked email data in JSON format with all available fields.

**Example output:**
```json
{
  "records": [
    {
      "email": "myemail1@mydomain.xyz",
      "description": "Email for example.com",
      "web_site": "https://example.com",
      "state": "enabled",
      "created_at": "2023-11-12 05:57:39 GMT"
    },
    {
      "email": "myemail2@mydomain.xyz",
      "description": "Email for test.org",
      "web_site": "https://test.org",
      "state": "enabled",
      "created_at": "2023-12-15 14:22:10 GMT"
    }
  ]
}
```

#### 3. `tsv.lua`

Exports masked email data in tab-separated values format with headers.

**Example output:**
```
email                   description             web_site               state     created_at
----------------------- ----------------------- ---------------------- --------- -----------------------
myemail1@mydomain.xyz   Email for example.com   https://example.com    enabled   2023-11-12 05:57:39 GMT
myemail2@mydomain.xyz   Email for test.org      https://test.org       enabled   2023-12-15 14:22:10 GMT
```

#### 4. `xml.lua`

Exports masked email data in XML format.

**Example output:**
```xml
<records>
  <record>
    <email>myemail1@mydomain.xyz</email>
    <description>Email for example.com</description>
    <web_site>https://example.com</web_site>
    <state>enabled</state>
    <created_at>2023-11-12 05:57:39 GMT</created_at>
  </record>
  <record>
    <email>myemail2@mydomain.xyz</email>
    <description>Email for test.org</description>
    <web_site>https://test.org</web_site>
    <state>enabled</state>
    <created_at>2023-12-15 14:22:10 GMT</created_at>
  </record>
</records>
```

You can create your own custom Lua scripts by following the examples in the `lua` directory and the guidelines in the [Lua Best Practices](docs/LUA_BEST_PRACTICES.md) document.

## Development

This project uses [just](https://github.com/casey/just) as a command runner. Once you have `just` installed, you can use the following commands:

```bash
# Show all available commands
just

# Format code (recommended before committing)
just format

# Run linting checks
just lint

# Run tests
just test

# Clean build artifacts
just clean

# Build in release mode
just build

# Run in debug mode
just run
```

Recommendations:
- Always run `just format` and `just lint` before committing code to ensure consistent style
- Use `just test` to verify your changes don't break existing functionality
- For production builds, use `just build` to create an optimized binary

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
