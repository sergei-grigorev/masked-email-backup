# Project Structure

## Table of Contents
- [Overview](#overview)
- [Package Descriptions](#package-descriptions)
  - [actions](#actions)
  - [cli](#cli)
  - [config](#config)
  - [db](#db)
  - [fastmail](#fastmail)
  - [model](#model)
  - [secrets](#secrets)

## Overview

The masked-email-cli project is organized into several key packages, each with specific responsibilities:

```
masked-email-cli/
├── src/
│   ├── actions/       # Core functionality for email operations
│   ├── config/        # Configuration management
│   ├── db/            # Database operations
│   ├── fastmail/      # FastMail API integration
│   ├── model/         # Data models
│   ├── secrets/       # Secure storage
│   └── main.rs        # Application entry point
└── README.md          # Project documentation
```

## Package Descriptions

### [actions](#actions)
Handles core functionality for interacting with masked emails, including refreshing the database from FastMail, exporting emails using Lua scripts, and displaying emails in an interactive UI.

```
src/actions/
├── export.rs          # Exports masked emails using Lua scripts for custom formatting
├── show_emails.rs     # Interactive UI for displaying and selecting masked emails
└── actions.rs         # Core actions: refresh database, export emails, and show emails
```

### [cli](#cli)
Provides user interaction utilities for the command-line interface, including secure password input and text prompts.

```
src/
└── cli.rs            # CLI utilities for user input and password prompts
```

### [config](#config)
Manages application configuration, command-line arguments, and user settings stored in TOML files.

```
src/config/
├── userconfig.rs     # User configuration management with TOML file storage
└── config.rs         # Command-line argument definitions and app configuration
```

### [db](#db)
Implements an encrypted database for securely storing masked email data on disk with AES encryption.

```
src/db/
├── disk.rs           # Encrypted database operations for storing masked emails
└── db.rs             # Database module exports
```

### [fastmail](#fastmail)
Contains the FastMail API client and JSON structures for interacting with FastMail's JMAP API to retrieve masked emails.

```
src/fastmail/
├── json/
│   ├── masked_email_get.rs    # JSON structures for FastMail masked email API responses
│   ├── method_response.rs     # JSON structures for FastMail JMAP method responses
│   └── session.rs             # JSON structures for FastMail session API
├── json.rs                    # JSON module exports for FastMail API
└── fastmail.rs                # FastMail API client for retrieving masked emails
```

### [model](#model)
Defines the data structures for masked emails, including their states (active, disabled, etc.) and properties.

```
src/model/
├── masked_email.rs    # Data model for masked email with state management
└── model.rs           # Model module exports
```

### [secrets](#secrets)
Implements secure storage for sensitive information like passwords and encryption keys, with platform-specific keychain integration and memory zeroing for security.

```
src/secrets/
├── encryption.rs      # AES encryption utilities for secure data storage
├── fastmail.rs        # FastMail account credentials management
├── keychain.rs        # Secure storage using system keychain for passwords and keys
└── secrets.rs         # Secure data types with memory zeroing for passwords and keys
```
