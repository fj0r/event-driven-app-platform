const WORKDIR = path self .

export def 'dev start' [] {
    let t = open ([$WORKDIR __.toml] | path join) | get dx
    ^dx serve --port $t.port
}

export def 'dev build' [] {
    rm -rf target/dx/ui/release/web/public/
    ^dx build --platform web --release
    dust target/dx/ui/release/web/public/
}

def cmpl-data [] {
    cd ([$WORKDIR data message] | path join)
    ls | get name
}

export def 'send message' [
    file:string@cmpl-data
    --patch(-p): record = {}
    --full
] {
    let f = if $full { $file } else {
        [$WORKDIR data message $file] | path join
    }
    let c = open ([$WORKDIR __.toml] | path join) | get server
    let host = $"http://($c.host)/admin/send"
    let data = {
        receiver: [],
        sender: "",
        content: (open $f | merge deep $patch)
    }
    http post --content-type application/json $host $data
}

export def 'watch message' [] {
    watch data/message {|op, path|
        if $op not-in ['Write'] { return }
        send message --full $path
    }
}

export def 'border flashing' [] {
    for _ in 1.. {
        for i in [primary, disable, secondary, accent] {
            sleep 0.2sec
            send message 00.chat.layout.yaml -p {
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


export def 'export css' [] {
    use git *
    use git/shortcut.nu *
    use lg
    lg level 1 'begin'
    cp assets/main.css .../ydncf/index.css
    let msg = git-last-commit
    let msg = $"($msg.message)\n\n($msg.body)"
    cd .../ydncf
    if (git-changes | is-not-empty) {
        git add .
        git commit -m $msg
        git push
    }
    lg level 1 'end'
}
