const WORKDIR = path self .

export def 'dev start' [] {
    let t = open ([$WORKDIR __.toml] | path join) | get dx
    dx serve --port $t.port
}

def cmpl-data [] {
    cd ([$WORKDIR data message] | path join)
    ls | get name
}

export def 'send message' [file:string@cmpl-data] {
    let d = open ([$WORKDIR data message $file] | path join)
    let c = open ([$WORKDIR __.toml] | path join) | get server
    let host = $"http://($c.host)/admin/message"
    let data = {
        receiver: [],
        message: {
            user: "test",
            content: $d
        }
    }
    http post --content-type application/json $host $data
}

export def 'ui init' [] {
    send message 00.layout.yaml
    send message 01.layout.yaml
}
