Just a simple tool to enumerate Dirs on websites and subdomains using
your config DNS(/etc/resolv.conf)<br><br>

Compiling: cargo build and ./target/debug/Hunt "options"

-d --directory<br>
-D --subDomain<br>
-w --wordlist<br>
-a --useragent<br>
-p --proxy<br>
-o --output<br>

Execution mode:<br><br>
    Hunt -d http://example.com -w wordlistFile.txt<br>
    Hunt -d http://example.com -w wordlistFile.txt -p 127.0.0.1:9050<br>
    Hunt -D example.com -w wordlistFile.txt -o outputFile.txt<br>
