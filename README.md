# rassforge #
### A powerful Rust-based command-line tool for generating wordlists for bruteforce and password spraying attacks. ###
![2023-05-22_21-06](https://github.com/vulfilip/rassforge/assets/131812836/52481959-dbfa-4718-ac3b-1a093db9fb33)

# 🎓 What is **Rassforge** ? 🎓 #
**Rassforge** is a powerful password forging tool built in Rust, designed to assist in the generation of custom wordlists for password cracking, bruteforcing, password spraying or other security testing purposes. With Rassforge, you can create personalized wordlists tailored to your specific needs and target profiles.

Rassforge stands for **Rust Password Forge**.
## ⚡️ Fast and Efficient: ##
Rassforge leverages the speed and performance of Rust to deliver quick results, ensuring that your wordlists are generated in a timely manner.

# ℹ️  Introduction & Usage ℹ️  #
Rassforge is split into multiple modes, each of them having their own specific flags.
```
./rassforge <mode> <FLAGS...>
```
Where \<mode\> is one of the following: **standard, crunch, encode**.
and <FLAGS...> the modes specific flags, which can be seen in:
`./rassforge <mode> --help`
There are also general flags, which are available across all the modes.
### Global flags ###
`./rassforge --help` to show all

**`head`** - used to specify a value which will always be at the beggining of the generated word. 

**`tail`** - used to specify a value which will always be at the end of the generated word. 

**`output`** - used to specify a file path where Rassforge needs to write the results to. Default is: `rassforge_passlist.txt`


Rassforge is split into multiple modes:
## ⚙️ Standard Mode: ##
Generate wordlists using standard mode, which enables you to combine keywords, years, and symbols to create a diverse range of passwords. Specify a file containing keywords and a year range, and Rassforge will intelligently mix and match them to produce unique password combinations. You can also set custom symbols to be included in the generation process. Particularly good for common password spraying.
![2023-05-22_21-10](https://github.com/vulfilip/rassforge/assets/131812836/ab7250d1-6e34-4f4a-a16e-0e2f94c2d1df)

![2023-05-22_21-13](https://github.com/vulfilip/rassforge/assets/131812836/d35f139d-473d-4dd0-87fa-1ce8ab715583)

## 💥 Crunch Mode: ##
Crunch mode allows you to generate wordlists using a flexible combination string approach. Define a character set, minimum and maximum size for the combination, and Rassforge will create a variety of passwords based on your specifications. Similar to already known **crunch** tool.
![2023-05-22_21-11](https://github.com/vulfilip/rassforge/assets/131812836/ebf67506-7f20-4289-a5c4-fc5a6f4b7692)
![2023-05-22_21-08](https://github.com/vulfilip/rassforge/assets/131812836/3e60f036-225d-46c5-aa5a-823984ac71e5)

## 🔐 Encode Mode: ##
In encode mode, Rassforge offers the ability to encode existing wordlists with various hash algorithms such as MD5, SHA1, SHA256, SHA512, Base32, Base64, and ROT13. Specify the encoding algorithm and the input file, and Rassforge will encode each line of the file, generating an encoded wordlist for further use.
![2023-05-22_21-12](https://github.com/vulfilip/rassforge/assets/131812836/d427cda4-cf37-4db1-b81f-dedc8eff0b2c)
![2023-05-22_21-17](https://github.com/vulfilip/rassforge/assets/131812836/ee8ec012-f053-4706-ba35-86def60a31b2)
![2023-05-22_21-21](https://github.com/vulfilip/rassforge/assets/131812836/2d234ee5-08bd-40bd-a776-893db21e5c6d)

# 🔨 Installation 🔨 #
To do...


