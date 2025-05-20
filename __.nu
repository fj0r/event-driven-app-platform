const WORKDIR = path self .

export def 'dev start' [] {
    let t = open ([$WORKDIR __.toml] | path join) | get dx
    ^dx serve --port $t.port
}

export def 'dev build' [] {
    ^dx build --platform web --release
}

def cmpl-data [] {
    cd ([$WORKDIR data message] | path join)
    ls | get name
}

export def 'send message' [
    file:string@cmpl-data
    --patch(-p): record = {}
] {
    let d = open ([$WORKDIR data message $file] | path join)
    let c = open ([$WORKDIR __.toml] | path join) | get server
    let host = $"http://($c.host)/admin/send"
    let data = {
        receiver: [],
        sender: "",
        content: ($d | merge deep $patch)
    }
    http post --content-type application/json $host $data
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


export def 'update tmpl' [
    src: string = "data/message/01.msg.tmp1.yaml"
    dest: string = "../flange/assets/ai.tmpl.json"
] {
    cat $src
    | from yaml
    | update data {|x| $x.data | to json -r }
    | to json
    | str replace -a '\"{{' '{{'
    | str replace -a '}}\"' '}}'
    | save -f $dest
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
