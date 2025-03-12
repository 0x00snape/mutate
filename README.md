_____________________________________________
![maxresdefault](https://github.com/user-attachments/assets/c13da007-630b-4e08-91ff-d567717c3d2d)
______________________________________________
# mutate
Mutate is a program which modify the code at run-time and propagate itself into a binary of its running path. It destructively overwrites the binary at running path.

# Description
Mutate has successfully embedded itself into the hello program and overwrite it with morphed version of itself also hides the original hello binary. On each runtime generates a unique hash due to binary changes and runs the original hello executable from its hidden binary. On each propagation it uses unique version of the morphed code. 

```bash
:$ ls -la
-rwxr-xr-x  1 3926856 Mar 12 18:26 hello
-rwxr-xr-x  1 374296 Mar 12 18:26 mutate

:$ shasum mutate hello 
fd0ab851b9856c192eceeac65b071a9c5a4073c6  mutate
0d09428faeeface8a0a7b0d96047ef8aa6d3147a  hello

:$ /hello 
Hello from ar.p

:$ ./mutate

:$ ls -la
-rwxr-xr-x  1 3926856 Mar 12 18:26 .hello
-rwxr-xr-x  1 374296 Mar 12 18:26 hello

:$ shasum hello .hello 
033caaad3a54e3623150f5b5823e519a068ae456  hello
0d09428faeeface8a0a7b0d96047ef8aa6d3147a  .hello

:$ ./hello
Hello from ar.p

:$ shasum hello 
f365de28bb408d70af4e1ac53e51dffdf434e6a7  hello
```
## Disclaimer
This project is intended for educational purposes to better understand the mutating technique. I will not take any responsibility for any malicious use.

## License
This project is licensed under [MIT](https://github.com/0x00snape/mutate/blob/main/LICENSE)

