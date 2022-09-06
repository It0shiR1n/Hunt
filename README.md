Just one simple tool to enumerate Dirs on websites and subdomains using
your config DNS(/etc/resolv.conf)<br><br>

Compiling: 
    cargo build 
    ./target/debug/Hunt "options"

-d --directory<br>
-D --subDomain<br>
-w --wordlist<br>
-a --useragent<br>
-p --proxy<br>
-o --output<br>

Execution mode:
    Hunt -d http://example.com -w wordlistFile.txt 
    Hunt -d http://example.com -w wordlistFile.txt -p 127.0.0.1:9050
    Hunt -D example.com -w wordlistFile.txt -o outputFile.txt
