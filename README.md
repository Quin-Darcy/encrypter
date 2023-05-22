# crypter-cli
This application provides an easy and secure way to encrypt and decrypt directories using a custom Rust implementation of the AES block cipher algorithm. 
The application uses recursion to encrypt or decrypt a given directory and uses Rayon to parallelize the recursion. The AES key for encryption is by default 
256-bit and is generated by the application. The key is stored in a user-specified location. The backbone of this application is the complementary library
[aes-crypt](https://github.com/Quin-Darcy/aes_crypt) which was also written in Rust and informed by [FIPS 197](https://csrc.nist.gov/publications/detail/fips/197/final).

## Prerequisites
This guide assumes you have Rust installed on your system. If not, please follow the offical documentation: [Installing Rust](https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/getting-started.html)


## Installation
### Step 1: Clone the repository and build the project
To begin using this command line tool, you must first clone the repository:
```
git clone https://github.com/Quin-Darcy/crypter-cli.git
```
```
cd crypter-cli
```
```
cargo build --release
```

### Step 2: Adding compiled binary to system path
After you have built the binary, you can add it to your system path to make it easier to run from any location in your terminal or command prompt. Here's how
to do it for each operating system:

#### Linux/macOS
Move the binary to a directory that is already in your `$PATH` like `/usr/local/bin`
```
mv /path/to/cloned/repository/target/release/crypter /usr/local/bin/
```
Replace `/path/to/cloned/repository` with the actual path to the cloned repository.

#### Windows
1. Copy the binary file
```
copy \path\to\cloned\repository\target\release\crypter.exe
```
2. Paste the copied binary to `C:\Windows\System32`.
3. You should now be able to run `crypter` from anywhere in your Command Prompt or PowerShell.

Please note, these instructions require administrator privileges. If you're not comfortable moving your binary to these directories or don't have the 
necessary permissions, you can instead add the directory containing your binary to your PATH.

## Usage
#### Synopsis
&nbsp; `crypter [OPTIONS] --target <FILE OR DIRECTORY> --key <KEY FILE>`

#### Options
&nbsp; `-e`&nbsp; This flag indicates the target directory is to be encrypted

&nbsp; `-d`&nbsp; This flag indicates the target directory is to be decrypted. 


## Important Node on Key Management
This tool uses AES encryption to secure your files, generating a unique key that is absolutely essential for decrypting them. If you loose this key, **you 
will not be able to decrypt your files**. It's crucial to understand this before using the tool, as improper key management can result in irreversible data loss.

Here are a few recommended practices for key management:
* **Backup your key**: As soon as the key is generated, make a copy of it and store it in a secure location. This could be an offline device or a trusted password manager.
* **Don't forget the key location**: The key is stored in the directory you create when preparing the volume. Be sure to remember where you've stored it.
* **Use a secure storage service**: Consider using a dedicated service for storing encryption keys. These services often have additional security measures to protect against data loss. 

Additionally, before encrypting any data, it's a good idea to make a backup of the original files, especially if they are important. While this tool is 
designed to be reliable, there's always a risk when dealing with encryption, and having a backup can provide an additional layer of security.

## Feedback and issues
If you have feedback or run into issues, please let me know!
