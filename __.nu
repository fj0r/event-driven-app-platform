const WORKDIR = path self .
const CFG = path self __.toml
const GW = path self gateway.toml
const CHAT = path self chat.toml

export def workdir [] {
    $WORKDIR
}

def wait-cmd [action -i: duration = 1sec  -t: string='waiting'] {
    mut time = 0
    loop {
        print -e $"(ansi dark_gray)($t) (ansi dark_gray_italic)($i * $time)(ansi reset)"
        let c = do --ignore-errors $action | complete | get exit_code
        if ($c == 0) { break }
        sleep $i
        $time = $time + 1
    }
}

export def receiver [] {
    let c = open $CFG
    http get $"http://($c.server.host)/admin/sessions"
}

def cmpl-act [] {
    [Message Layout test]
}

def cmpl-data [] {
    cd ([$WORKDIR data message] | path join)
    ls | get name
}

def cmpl-external [] {
    ip route
    | lines
    | parse -r ([
        '(?<default>default via)?'
        '(?<gateway>[0-9\./]+)'
        'dev (?<dev>[\w\-]+)'
        'proto (?<proto>dhcp|kernel scope link)'
        'src (?<src>[0-9\.]+)'
    ] | str join '\s*')
    | get src
    | uniq
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
    let c = open $CFG
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


def cmpl-reg [] {
    open $CFG | get hooks | columns
}

export def 'hooks list' [] {
    let c = open $CFG | get server
    http get $"http://($c.host)/config/hooks"
}

export def 'hooks upload' [name: string@cmpl-reg] {
    let c = open $CFG
    let d = $c | get hooks | get $name
    let h = $c.server.host
    for i in ($d | transpose k v) {
        http post --allow-errors --content-type application/json $"http://($h)/config/hooks/($i.k)" $i.v
    }
}

export def 'serve' [
    --rpk
    --external: string@cmpl-external = 'localhost'
] {
    if $rpk {
        rpk up --external $external
    }
    $env.RUST_BACKTRACE = 1
    #$env.GATEWAY_KAFKA_ENABLE = 1
    let g = job spawn {
        gw up
    }
    ui up
    job kill $g
}


export def 'ui up' [] {
    let t = open $CFG | get dx
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

export def 'chat up' [
] {
    cargo run --bin chat
}

export def 'chat build' [] {
    cargo build --release --bin gateway
}

export def 'gw up' [
] {
    cargo run --bin gateway
    watch gateway --glob **/*.rs -q {|op, path, newPath|
        if $op not-in ['Write'] { return }

        let x = ps -l | where command == target/debug/gateway
        if ($x | is-not-empty) {
            kill $x.pid
        }
        cargo run --bin gateway
    }
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

export def 'pg cli' [--db:string = 'chat'] {
    let cfg = open $CHAT | get database
    let db = $db | default $cfg.db
    let cmd = $"
        INSTALL postgres;
        LOAD postgres;
        ATTACH 'dbname=($db) user=($cfg.user) host=127.0.0.1 port=($cfg.port) password=($cfg.passwd)' AS ($db) \(TYPE postgres\);
        USE ($db)
    "
    duckdb -cmd $cmd
}


export def 'pg start' [
    --dry-run
] {
    let cfg = open $CHAT | get database
    let image = 'postgres:17'
    mut args = [run -d --name chat_db]
    let ports = {
        $cfg.port: 5432
    }
    for i in ($ports | transpose k v) {
        $args ++= [-p $"($i.k):($i.v)"]
    }
    let envs = {
        POSTGRES_DB: $cfg.db
        POSTGRES_USER: $cfg.user
        POSTGRES_PASSWORD: $cfg.passwd
    }
    for i in ($envs | transpose k v) {
        $args ++= [-e $"($i.k)=($i.v)"]
    }
    $args ++= [-v $"($WORKDIR)/data/postgres/data:/var/lib/postgresql/data"]
    $args ++= [$image]
    if $dry_run {
        print $"($env.CNTRCTL) ($args | str join ' ')"
    } else {
        ^$env.CNTRCTL ...$args
    }
}

export def 'pg up' [--reset] {
    if $reset {
        const d = path self data/postgres/data/
        print $"rm -rf ($d)"
        sudo rm -rf $d
    }
    let cfg = open $CHAT | get database
    dcr chat_db
    pg start
    wait-cmd -t 'wait postgresql' {
        ^$env.CNTRCTL ...[
            exec chat_db
            bash -c
            $'pg_isready -U ($cfg.user)'
        ]
    }
}

export def 'pg migrate' [] {
    cargo run --bin migrate
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
    --external: string@cmpl-external = 'localhost'
] {
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

export def 'rpk up' [
    --product
    --consume
    --external: string@cmpl-external = 'localhost'
] {
    dcr redpanda
    rpk start --external $external

    wait-cmd -t 'wait redpanda' {
        ^$env.CNTRCTL ...[
            exec redpanda
            rpk cluster info
        ]
    }

    let s = open $GW
    rpk topic create $s.queue.outgo.topic
    rpk topic create $s.queue.income.topic.0

    if $product {
        rpk send --topic $s.queue.outgo.topic (open data/message/event.yaml)
    }

    if $consume {
        rpk consume $s.queue.outgo.topic
    }
}

export def 'docker up' [
    --external: string@cmpl-external = 'localhost'
] {
    let image = 'ghcr.io/fj0r/edap:lastest'
    ^$env.CNTRCTL pull $image
    ^$env.CNTRCTL run ...[
        --name edap
        --rm -it
        -p 3001:3000
        -e $"GATEWAY_QUEUE_EVENT_BROKER=[($external):19092]"
        -e $"GATEWAY_QUEUE_PUSH_BROKER=[($external):19092]"
        -w /app
        $image
        /app/gateway
    ]
}

export def 'test render' [] {
    curl -H 'Content-Type: application/json' -X POST http://localhost:3000/debug/render/user.json -d'{"info": {"username": "test"}}'
}

export def clippy [dir] {
    cd $dir
    cargo clippy
}

export def jsonschema [] {
    cd layout
    cargo run --bin schema
}

export def benchmark [n: int] {
    #drill -b drill.yaml -s
    let url = [
        http://localhost:3000/admin/sessions
        http://localhost:3003/v1/user/alice
        http://localhost:3003/v1/users
    ]
    let url = $url | get $n
    print $"====> ($url)"
    oha -c 50 -n 200000 $url
}


export def git-hooks [act ctx] {
    if $act == 'pre-commit' and $ctx.branch == 'main' {
        cargo fmt
        git add .
    }
}
