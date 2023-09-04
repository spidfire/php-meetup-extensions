https://crates.io/crates/ext-php-rs



start docker file with `make run`

open rust directory in the docker image

then run `make build`

and to get the phpinfo:

`make build run FILE=test1.php | grep -B 10 -A 10 "My first rust"`

