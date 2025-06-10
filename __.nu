const WORKDIR = path self .
const CFG = path self __.toml
const CONFIG = path self config.toml

export def receiver [] {
    let c = open $CFG
    http get $"http://($c.server.host)/admin/users"
}

def cmpl-act [] {
    [Message Layout test]
}

def cmpl-data [] {
    cd ([$WORKDIR data message] | path join)
    ls | get name
}

export def send [
    file:string@cmpl-data
    --receiver(-r): list<string@receiver> = []
    --sender(-s): string = 'unknown'
    --patch(-p): record = {}
    --full
    --rpk
    --topic(-t):string@"rpk topic list" = "push"
    --partition:int=0
] {
    let f = if $full { $file } else {
        [$WORKDIR data message $file] | path join
    }
    let data = {
        receiver: $receiver,
        sender: $sender,
        content: (open $f | merge deep $patch)
    }
    let c = open ([$WORKDIR __.toml] | path join)
    if $rpk {
        let data = {
            records: {
                value: $data
                partition: $partition
            }
        }
        | to json -r
        http post -H [
            Content-Type application/vnd.kafka.json.v2+json
        ] $"http://($c.redpanda.admin)/topics/($topic)" $data
    } else {
        let c = $c | get server
        let host = $"http://($c.host)/admin/send"
        http post --content-type application/json $host $data
    }
}

export def 'watch message' [] {
    watch data/message {|op, path|
        if $op not-in ['Write'] { return }
        send --full $path
    }
}

export def 'serve' [--rpk] {
    if $rpk {
        rpk up
    }
    $env.RUST_BACKTRACE = 1
    #$env.APP_KAFKA_ENABLE = 1
    let g = job spawn {
        $env.RUSTFLAGS = "--cfg tokio_allow_from_blocking_fd"
        systemfd --no-pid -s http::3000 -- watchexec -r -- cargo run --bin gateway
    }
    ui up
    job kill $g
}


export def 'ui up' [] {
    let t = open ([$WORKDIR __.toml] | path join) | get dx
    cd ui
    ^dx serve --port $t.port
}

export def 'ui build' [] {
    cd ui
    rm -rf target/dx/ui/release/web/public/
    ^dx build --platform web --release
    dust target/dx/ui/release/web/public/
}

export def 'ui border flashing' [] {
    for _ in 1.. {
        for i in [primary, disable, secondary, accent] {
            sleep 0.2sec
            send 00.chat.layout.yaml -p {
                data: {
                    children: [
                        {},
                        {item:
                            [
                                {},
                                {attrs: {class: $'box border shadow nogrow s as ($i)'}}
                            ]
                        }
                    ]
                }
            }
        }
    }
}

export def 'ui export css' [] {
    use git *
    use git/shortcut.nu *
    use lg
    lg level 1 'begin'
    cp ui/assets/main.css ../ydncf/index.css
    let msg = git-last-commit
    let msg = $"($msg.message)\n\n($msg.body)"
    cd ../ydncf
    if (git-changes | is-not-empty) {
        git add .
        git commit -m $msg
        git push
    }
    lg level 1 'end'
}

export def 'gw build' [] {
    $env.RUSTFLAGS = "--cfg tokio_unstable"
    cargo build --release --bin gateway
}

export def 'gw profile' [] {
    cargo profiler callgrind --bin target/release/gateway
    kcachegrind callgrind.out
    rm callgrind.out
}

export def 'gw client' [] {
    let c = open $CFG
    websocat $"ws://($c.server.host)/channel"
}

export def 'gw test' [] {
    let ji = job spawn { dev serve }
    sleep 2sec
    do -i {
        dev client
    }
    job kill $ji
}

export def 'rpk send' [
    data
    --partition:int=0
    --topic(-t):string@"rpk topic list"
    --patch: record = {}
] {
    let c = open $CFG
    let data = { records: ($data | merge deep $patch | wrap value | insert partition $partition) } | to json -r
    http post -H [
        Content-Type application/vnd.kafka.json.v2+json
    ] $"http://($c.redpanda.admin)/topics/($topic)" $data
}

export def 'rpk subscribe' [topic:string@"rpk topic list"] {
    let c = open $CFG
    let data = { topics: [$topic] } | to json -r
    curl -sL $"http://($c.redpanda.admin)/topics/($topic)/partitions/0/records?offset=0" -H "Content-Type: application/vnd.kafka.json.v2+json" --data $data
}

export def 'rpk consume' [topic:string@"rpk topic list"] {
    mut args = [exec -it redpanda]
    $args ++= [rpk topic consume $topic]
    ^$env.CNTRCTL ...$args
}

export def 'rpk group list' [] {
    mut args = [exec -it redpanda]
    $args ++= [rpk group list]
    ^$env.CNTRCTL ...$args | from ssv
}

export def 'rpk group delete' [group:string@"rpk group list"] {
    mut args = [exec -it redpanda]
    $args ++= [rpk group delete $group]
    ^$env.CNTRCTL ...$args
}

export def 'rpk topic list' [] {
    let c = open $CFG
    http get $"http://($c.redpanda.admin)/topics" | from json
}

export def 'rpk topic create' [name:string] {
    mut args = [exec -t redpanda]
    $args ++= [rpk topic create $name]
    ^$env.CNTRCTL ...$args
}

export def 'rpk topic delete' [name:string@'rpk topic list'] {
    mut args = [exec -t redpanda]
    $args ++= [rpk topic delete $name]
    ^$env.CNTRCTL ...$args
}

export def 'rpk start' [
    --dry-run
] {
    let external = $env.external? | default 'localhost'
    let image = 'redpandadata/redpanda:latest'
    mut args = [run -d --name redpanda]
    let ports = {
        '18081': 18081
        '18082': 18082
        '19092': 19092
        '19644': 9644
    }
    for i in ($ports | transpose k v) {
        $args ++= [-p $"($i.k):($i.v)"]
    }
    let envs = {
    }
    for i in ($envs | transpose k v) {
        $args ++= [-e $"($i.k)=($i.v)"]
    }
    $args ++= [$image]
    $args ++= [
        redpanda
        start
        --kafka-addr
        'internal://0.0.0.0:9092,external://0.0.0.0:19092'
        --advertise-kafka-addr
        $'internal://127.0.0.1:9092,external://($external):19092'
        --pandaproxy-addr
        'internal://0.0.0.0:8082,external://0.0.0.0:18082'
        --advertise-pandaproxy-addr
        $'internal://127.0.0.1:8082,external://($external):18082'
        --schema-registry-addr
        'internal://0.0.0.0:8081,external://0.0.0.0:18081'
        --rpc-addr
        localhost:33145
        --advertise-rpc-addr
        localhost:33145
        --mode
        dev-container
        --smp 1
        --default-log-level=info
    ]
    if $dry_run {
        print $"($env.CNTRCTL) ($args | str join ' ')"
    } else {
        ^$env.CNTRCTL ...$args
    }
}

export def 'rpk up' [--product --consume] {
    dcr redpanda
    rpk start

    let readyness = {
        ^$env.CNTRCTL ...[
            exec redpanda
            rpk cluster info
        ]
    }
    mut time = 0
    loop {
        print -e $"(ansi light_gray)wait redpanda (ansi light_gray_italic)(1sec * $time)(ansi reset)"
        let c = do --ignore-errors $readyness | complete | get exit_code
        if ($c == 0) { break }
        sleep 1sec
        $time = $time + 1
    }

    let s = open $CONFIG
    rpk topic create $s.queue.event.topic
    rpk topic create $s.queue.push.topic.0

    if $product {
        rpk send --topic $s.queue.event.topic (open data/message/event.yaml)
    }

    if $consume {
        rpk consume $s.queue.event.topic
    }
}

export def 'docker up' [] {
    let external = $env.external? | default 'localhost'
    ^$env.CNTRCTL run ...[
        --name edap
        --rm -it
        -p 5000:3000
        -e $"APP_QUEUE_EVENT_BROKER=[($external):19092]"
        -e $"APP_QUEUE_PUSH_BROKER=[($external):19092]"
        -w /app
        ghcr.io/fj0r/edap:lastest
        /app/gateway
    ]
}

export def 'test render' [] {
    curl -H 'Content-Type: application/json' -X POST http://localhost:3000/debug/render/user.json -d'{"info": {"username": "test"}}'
}
