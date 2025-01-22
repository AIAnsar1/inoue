# About Inoue

Inoue is a high-performance, scalable HTTP load-testing tool for web services. 

## Features
* Fast and scalable HTTP load testing
* Supports structured, real-time metrics
* Detailed performance analytics


## Install


```bash
    sh install.sh
```





```shell
$ cargo build --release
```






## Usage manual

```console
Usage: Inoue [OPTIONS] --target <TARGET>

Options:
  -v, --verbose                      Runs in verbose mode
  -t, --target <TARGET>              URL to be requested using an operation [default: GET] Ex. GET http://localhost:3000/
  -r, --request-body <REQUEST_BODY>  File path for the request body
  -c, --clients <CLIENTS>            Number of concurrent clients [default: 1]
  -i, --iterations <ITERATIONS>      Total number of iterations [default: 1]
  -d, --duration <DURATION>          Duration of the test in second
      --headers <HEADERS>            Headers, multi value in format headerName:HeaderValue
      --scenario <SCENARIO>          Scenario file
  -h, --help                         Prints help
  -V, --version                      Prints version information
```

#### `--target` `-t`
Specifies the operation and url to make the request, default to GET.<br>
Format: GET https://localhost:3000<br>

#### `--request-body` `-r` Optional
Specifies the path of file with the body to send.<br>

#### `--clients` `-c`
Specifies the number of concurrent calls to be used, defaults to 1.

#### `--iterations` `-i`
Specifies the total number of calls to be performed, default to 1.

#### `--duration` `-d`
Specifies the duration of the test in seconds.

#### `--headers`  Optional
Specifies the headers to be sent.<br>

#### `--scenario`  Optional
Specifies the scenario file in yaml format.<br>

````yaml
target: POST http://localhost:3000/
clients: 50
requests: 1000
headers:
  - key: "bar"
    value: "foo"
  - key: "Content-Type"
    value: "application/json"

body: "{\"firstName\": \"Terry\",
        \"lastName\": \"Medhurst\",
        \"maidenName\": \"Smitham\",
        \"age\": 50}"


````


###### Simple targets

```
inoue --target "GET http://localhost:3000"
inoue --target http://localhost:3000?foo=bar
inoue -c 50 -i 1000 --target http://localhost:3000
inoue -c 50 --duration 60 --target http://localhost:3000
```

###### Targets with custom headers

```
Inoue --target "GET http://localhost:3000" --headers Content-Type:application/json --headers bar:foo 
```

###### Targets with custom bodies

```
Inoue -c 50 -i 1000 -r body.json --target "POST http://localhost:3000"

```

###### Targets with custom bodies and headers

```
Inoue -r body.json --target "POST http://localhost:3000" --headers Content-Type:application/json --headers bar:foo 

```

###### Output

```
Concurrency level 50
Time taken 4 seconds
Total requests 1000
Mean request time 169.90099999999998 ms
Max request time 415 ms
Min request time 5 ms
95'th percentile: 319 ms
99.9'th percentile: 367 ms
```

