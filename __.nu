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
    let host = $"http://($c.host)/admin/message"
    let data = {
        receiver: [],
        sender: "",
        content: ($d | merge deep $patch)
    }
    http post --content-type application/json $host $data
}


export def 'update tmpl' [
    src: string = "data/message/01.msg.tmp1.yaml"
    dest: string = "../flange/assets/ai.tmpl.json"
] {
    cat $src
    | from yaml
    | update data {|x| $x.data | to json -r }
    | to json
    | save -f $dest
}
